// TODO: Remove all unwraps and then this.
#![expect(clippy::unwrap_used, reason = "WIP development")]

use std::{
	collections::HashMap,
	error,
};
use api::{
	edge::{
		EdgeRequests,
		ImageAnalysis,
		RegisterCameras,
		ServerRequests,
		Settings,
		UpdateCameraStatus,
	},
	shared::{
		CameraStatus,
		VisionTechnique,
	},
};
use tokio::{join, spawn};
use edge_cameras::persistence::{
	Persistence,
	WhyNo,
	polling::ConnectionDetail,
};
use edge_client::{
	error::EdgeClientError,
};
use edge_core::{
	logger::{
		Logger,
		logger,
	},
	settings::EnvVariables,
};
use edge_vision::{
	vision,
	vision_interface::VisionInterface as _,
};
use tokio::time::interval;
use edge_client::client::client::Client;
use edge_client::handle::client_handle::ClientHandle;
use edge_client::interface::types::{ListenerRx, ListenerTx, PersistenceRx, StatusRx, VisionRx};
use crate::error::EdgeError;

/// # Conductor
///
/// The main execution context.
pub struct Conductor {
	persistence: Persistence,
}

impl Conductor {
	async fn loop_once(
		&mut self,
		client: &mut ClientHandle,
		status_rx: StatusRx,
		mut listener: ListenerRx,
		settings: Settings,
	) -> Result<(StatusRx, ListenerRx, Settings), Box<dyn error::Error + Send + Sync>>
	{
		let mut limit_interval = interval(settings.update_delay);
		limit_interval.tick().await;

		let status_rx = self.handle_reconnect(client, status_rx);

		// TODO: Delay for polling cameras, so it doesn't happen every loop.
		self.update_camera_device_connections(client).await;

		// Save the most recent vision swap request, instead of trying to swap
		// more than once.
		let mut requested_vision_type = None;
		while let Ok(request) = listener
			.try_recv()
		{
			self.handle_request(
				request,
				client,
				&mut requested_vision_type,
			);
		}

		logger().lock().unwrap().send_logs();

		if let Some(swap_to) = requested_vision_type
			&& let Err(error) = vision().swap(swap_to)
		{
			logger().lock().unwrap().log_error(&error);
		}

		// TODO: There should be something in between this call that does the
		// below.
		let (mut batch_items, status_changes) =
			self.persistence.poll_active_entries(
				settings.camera_err_increment,
				settings.camera_err_threshold,
				settings.camera_err_decrement,
			);

		for status_change in status_changes {
			match status_change.why {
				WhyNo::Disconnected => {
					_ = client.update_camera_status(
						status_change.id,
						CameraStatus::Disconnected
					)
				},
				WhyNo::Broken => {
					_ = client.update_camera_status(
						status_change.id,
						CameraStatus::Broken
					)
				},
			}
		}

		logger().lock().unwrap().send_logs();

		// Vision
		vision().interact(|vision| {
			vision.process(&mut batch_items);
		});

		logger().lock().unwrap().send_logs();

		// Send result
		for item in &batch_items {
			let deviation_maybe = item.state.get_deviation();
			let cam_id = item.camera_id;
			if let Some(deviation) = deviation_maybe {
				let result = client.image_analysis(
					cam_id,
					deviation,
				);

				_ = result.inspect_err(|_| {
					EdgeError::from(EdgeClientError::ServerConnection).log();
				});
			}

			if item.send_to_server {
				let encoded = item.image.encode();
				let result = client.send_image(
					item.camera_id,
					encoded,
				);
				_ = result.inspect_err(|_| {
					EdgeError::from(EdgeClientError::ServerConnection).log();
				});
			}
		}

		// Send statistics

		// This should guarantee that the loop will take at least this long.
		limit_interval.tick().await;

		Ok((status_rx, listener, settings))
	}

	fn handle_reconnect(
		&mut self,
		client: &mut ClientHandle,
		mut status_rx: StatusRx,
	) -> StatusRx {
		let mut is_updated = false;
		while let Ok(status) = status_rx.try_recv() {
			is_updated = status;
		}

		if is_updated {
			let _ = client.register_cameras(
				self.persistence.get_current_cameras()
			);
		}
		status_rx
	}

	fn handle_request(
		&mut self,
		request: EdgeRequests,
		client: &mut ClientHandle,
		requested_vision_type: &mut Option<VisionTechnique>,
	) {
		match request {
			EdgeRequests::SetNameCamera(cam_name_req) => {
				self.persistence.set_entry_name(cam_name_req.clone());
				let _ = client.set_name_camera(cam_name_req.camera_id, cam_name_req.name);

			},
			EdgeRequests::ToggleViewingCamera(toggle_view_cam_req) => {
				if toggle_view_cam_req.start_viewing {
					self.persistence.start_viewing_camera(toggle_view_cam_req.camera_id);
				} else {
					self.persistence.stop_viewing_camera(toggle_view_cam_req.camera_id);
				}

			},
			EdgeRequests::Configuration(set_config_req) => {
				self.persistence.set_configuration(set_config_req);
			},
			EdgeRequests::ViewCameraConfig(view_config_req) => {
				if let Some(configuration) =
					self.persistence.get_configuration(view_config_req)
					&& let Err(_) = client.configuration(
					configuration.conn_id,
					configuration.camera_id,
					configuration.configuration,
				)
				{
					// TODO: Error type for "Failed to send configuration
					// request"
					EdgeError::from(EdgeClientError::WebSocketConnection).log();
				}
			},
			EdgeRequests::Setpoint(calibration_req) => {
				self.persistence.request_calibration(calibration_req);
			},
			EdgeRequests::SetProcessingType(processing_req) => {
				requested_vision_type.replace(processing_req.processing_type);
			},
			_ => {
				logger().lock().unwrap().log_warn(
					"received request that is not currently supported",
				);
			},
		}
	}

	async fn update_camera_device_connections(
		&mut self,
		client: &mut ClientHandle,
	) {
		let mut results = self.persistence.poll_for_new_cameras();

		let new_cameras = results.extract_if(|_, value| {
			match value {
				ConnectionDetail::New {
					..
				} => true,
				ConnectionDetail::Update {
					..
				} => false,
			}
		});

		let new_cameras: HashMap<_, _> = new_cameras
			.into_iter()
			.filter_map(|(key, value)| {
				match value {
					ConnectionDetail::New {
						info,
					} => Some((key, info)),
					ConnectionDetail::Update {
						..
					} => None,
				}
			})
			.collect();

		for (camera_id, update) in results {
			let ConnectionDetail::Update {
				status,
			} = update
			else {
				continue;
			};
			_ = client.update_camera_status(camera_id, status);
		}

		if !new_cameras.is_empty() {
			_ = client.register_cameras(new_cameras);
		}
	}
}

impl Default for Conductor {
	fn default() -> Self {
		let persistence = Persistence::default();
		Self {
			persistence,
		}
	}
}

async fn main_loop(
	mut client: ClientHandle,
	mut conductor: Conductor,
	mut status_rx: StatusRx,
) -> Result<(), Box<dyn error::Error + Send + Sync>> {
	let mut listener = client.subscribe_new_listener().await;
	let mut settings = client.get_default_setting().await
		.unwrap_or(Settings::default());
	loop {
		client.assert_connection().await;
		let result = conductor.loop_once(&mut client, status_rx, listener, settings).await?;
		status_rx = result.0;
		listener = result.1;
		settings = result.2;
	};
}

/// Call this function to begin the system.
pub async fn run() -> Result<(), Box<dyn error::Error + Send + Sync>> {
	let conductor = Conductor::default();

	let env_variables = EnvVariables::initialise();
	Logger::initialise_logger(env_variables.verbosity());

	let handles =  Client::new().await;
	let client = handles.3;
	let status_rx = handles.0;

	// TODO: Integrate these into the system during proper integration
	//let vision_rx: VisionRx = handles.1;
	//let persistence_rx: PersistenceRx = handles.2;

	let client_thread = handles.4;
	let thread = spawn(client_thread.run());

	join!(
		async move {
			thread.await
			.expect("Could not join websocket thread")
			.expect("websocket thread panicked");
		},
		async move {
			main_loop(
				client,
				conductor,
				status_rx,
			).await.expect("main thread panicked");
		}
	);

	Ok(())
	

}
