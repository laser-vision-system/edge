#![expect(deprecated, reason = "Temporarily used for backwards compatibility")]

use std::{
	collections::HashMap,
	error,
};

use api::{
	edge::{
		EdgeRequests,
		Settings,
	},
	shared::{
		CameraStatus,
		VisionTechnique,
	},
};
use edge_cameras::persistence::{
	Persistence,
	WhyNo,
	polling::ConnectionDetail,
};
use edge_client::{
	client::{
		Client,
		ClientResult,
	},
	error::EdgeClientError,
	handle::{
		conductor_client_handle::ConductorClientHandle,
		status_handle::StatusHandle,
	},
	interface::ListenerRx,
};
use edge_core::logger::logger;
use edge_vision::{
	vision,
	vision_interface::VisionInterface as _,
};
use tokio::{
	spawn,
	task::JoinHandle,
	time::interval,
	try_join,
};

/// # Conductor
///
/// The main execution context.
pub struct Conductor {
	persistence: Persistence,
}

impl Conductor {
	async fn loop_once(
		&mut self,
		client: &mut ConductorClientHandle,
		status_rx: StatusHandle,
		mut listener: ListenerRx,
		settings: Settings,
	) -> Result<
		(StatusHandle, ListenerRx, Settings),
		Box<dyn error::Error + Send + Sync>,
	> {
		let mut limit_interval = interval(settings.update_delay);
		limit_interval.reset();

		let status_rx = self.handle_reconnect(client, status_rx);

		// TODO: Delay for polling cameras, so it doesn't happen every loop.
		self.update_camera_device_connections(client);

		// Save the most recent vision swap request, instead of trying to swap
		// more than once.
		let mut requested_vision_type = None;
		while let Ok(request) = listener.try_recv() {
			self.handle_request(request, client, &mut requested_vision_type);
		}

		logger().send_logs();

		if let Some(swap_to) = requested_vision_type
			&& let Err(error) = vision().swap(swap_to)
		{
			logger().log_error(&error);
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
			match status_change.why() {
				WhyNo::Disconnected => {
					_ = client.update_camera_status(
						status_change.id(),
						CameraStatus::Disconnected,
					);
				},
				WhyNo::Broken => {
					_ = client.update_camera_status(
						status_change.id(),
						CameraStatus::Broken,
					);
				},
			}
		}

		logger().send_logs();

		// Vision
		vision().interact(|vision| {
			vision.process(&mut batch_items);
		});

		logger().send_logs();

		// Send result
		for item in &batch_items {
			let deviation_maybe = item.state.get_deviation();
			let cam_id = item.camera_id;
			if let Some(deviation) = deviation_maybe
				&& let Err(why) = client.image_analysis(cam_id, deviation)
			{
				logger().log_error(&why);
			}

			if item.send_to_server {
				let encoded = item.image.encode();
				if let Err(why) = client.send_image(item.camera_id, encoded) {
					logger().log_error(&why);
				}
			}
		}

		// Send statistics

		// This should guarantee that the loop will take at least this long.
		let duration_of_loop = limit_interval.period().as_millis(); 
		print!("loop duration: {duration_of_loop}"); 
		limit_interval.tick().await;

		
		

		Ok((status_rx, listener, settings))
	}

	fn handle_reconnect(
		&mut self,
		client: &mut ConductorClientHandle,
		mut status_rx: StatusHandle,
	) -> StatusHandle {
		let mut is_updated = false;
		while let Ok(status) = status_rx.try_recv() {
			is_updated = status;
		}

		if is_updated {
			_ = client.register_cameras(self.persistence.get_current_cameras());
		}
		status_rx
	}

	fn handle_request(
		&mut self,
		request: EdgeRequests,
		client: &mut ConductorClientHandle,
		requested_vision_type: &mut Option<VisionTechnique>,
	) {
		match request {
			EdgeRequests::SetNameCamera(cam_name_req) => {
				self.persistence.set_entry_name(cam_name_req.clone());
				_ = client
					.set_name_camera(cam_name_req.camera_id, cam_name_req.name);
			},
			EdgeRequests::ToggleViewingCamera(toggle_view_cam_req) => {
				self.persistence.toggle_viewing_camera(
					toggle_view_cam_req.camera_id,
					toggle_view_cam_req.start_viewing,
				);
			},
			EdgeRequests::Configuration(set_config_req) => {
				self.persistence.set_configuration(set_config_req);
			},
			EdgeRequests::ViewCameraConfig(view_config_req) => {
				if let Some(configuration) =
					self.persistence.get_configuration(view_config_req)
					&& let Err(why) = client.configuration(
						configuration.conn_id,
						configuration.camera_id,
						configuration.configuration,
					) {
					// TODO: Error type for "Failed to send configuration
					// request"
					logger().log_error(&why);
				}
			},
			EdgeRequests::Setpoint(calibration_req) => {
				self.persistence.request_calibration(calibration_req);
			},
			EdgeRequests::SetProcessingType(processing_req) => {
				requested_vision_type.replace(processing_req.processing_type);
			},
			_ => {
				logger().log_warn(
					"received request that is not currently supported",
				);
			},
		}
	}

	fn update_camera_device_connections(
		&mut self,
		client: &mut ConductorClientHandle,
	) {
		let mut results = self.persistence.poll_for_new_cameras();

		let new_cameras = results.extract_if(|_, value| {
			match value {
				ConnectionDetail::New(_) => true,
				ConnectionDetail::Update(_) => false,
			}
		});

		let new_cameras: HashMap<_, _> = new_cameras
			.into_iter()
			.filter_map(|(key, value)| {
				match value {
					ConnectionDetail::New(info) => Some((key, info)),
					ConnectionDetail::Update(_) => None,
				}
			})
			.collect();

		for (camera_id, update) in results {
			let ConnectionDetail::Update(status) = update else {
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
	mut client: ConductorClientHandle,
	mut conductor: Conductor,
	mut status_rx: StatusHandle,
) -> Result<(), Box<dyn error::Error + Send + Sync>> {
	let mut listener = client.subscribe_new_listener().await;
	let mut settings = client
		.get_default_setting()
		.await
		.unwrap_or(Settings::default());
	loop {
		client.assert_connection().await;
		let result = conductor
			.loop_once(&mut client, status_rx, listener, settings)
			.await?;
		status_rx = result.0;
		listener = result.1;
		settings = result.2;
	}
}

async fn flatten<T>(
	handle: JoinHandle<Result<T, EdgeClientError>>
) -> Result<T, Box<dyn error::Error + Send + Sync>> {
	match handle.await {
		Ok(Ok(result)) => Ok(result),
		Ok(Err(err)) => Err(Box::new(err)),
		Err(err) => Err(Box::new(err)),
	}
}

/// Call this function to begin the system.
///
/// # Errors
///
/// If either the main loop or client thread return an error.
pub async fn run() -> Result<(), Box<dyn error::Error + Send + Sync>> {
	let ClientResult(status_handle, _vision_client_handle, _persistence_client_handle, conductor_client_handle, client) =
		Client::create();
	let conductor = Conductor::default();

	let client_thread = spawn(client.run());

	try_join!(
		biased;
		flatten(client_thread),
		main_loop(conductor_client_handle, conductor, status_handle),
	)
	.map(|((), ())| ())
}
