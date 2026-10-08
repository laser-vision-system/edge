// TODO: Remove all unwraps and then this.
#![expect(clippy::unwrap_used, reason = "WIP development")]

use std::{collections::HashMap, error};

use server_edge_datatypes::{
	edge::{
		EdgeRequests, ImageAnalysis, RegisterCameras, ServerRequests,
		UpdateCameraStatus,
	},
	shared::{CameraStatus, VisionTechnique},
};
use tokio::time::interval;

use crate::{
	error::EdgeError,
	logger::{Logger, logger},
	network::{
		NetworkError, auto_connect::assert_connection, websocket::WebSocket,
	},
	settings::{EnvVariables, Settings},
	tcam_interface::{
		CameraDevice, Persistence, WhyNo, polling::ConnectionDetail,
	},
	vision_processor::{VisionInterface as _, vision},
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
		mut websocket: WebSocket,
		settings: Settings,
	) -> Result<(WebSocket, Settings), Box<dyn error::Error + Send + Sync>> {
		let mut limit_interval = interval(settings.update_delay);
		limit_interval.tick().await;

		// TODO: Delay for polling cameras, so it doesn't happen every loop.
		self.update_camera_device_connections(&mut websocket).await;

		// Save the most recent vision swap request, instead of trying to swap
		// more than once.
		let mut requested_vision_type = None;
		while let Some(request) = websocket
			.query_websockets(settings.websocket_poll_interval)
			.await
		{
			self.handle_request(
				request,
				&mut websocket,
				&mut requested_vision_type,
			)
			.await;
		}

		logger().lock().unwrap().send_logs();

		_ = websocket.keep_alive().await;

		if let Some(swap_to) = requested_vision_type
			&& let Err(error) = vision().swap(swap_to)
		{
			error.log();
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
					_ = websocket
						.send_text(ServerRequests::UpdateCameraStatus(
							UpdateCameraStatus {
								camera_id: status_change.id,
								status: CameraStatus::Disconnected,
							},
						))
						.await;
				},
				WhyNo::Broken => {
					_ = websocket
						.send_text(ServerRequests::UpdateCameraStatus(
							UpdateCameraStatus {
								camera_id: status_change.id,
								status: CameraStatus::Broken,
							},
						))
						.await;
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
			let camid = item.camera_id;
			if let Some(deviation) = deviation_maybe {
				let result = websocket
					.send_text(ServerRequests::ImageAnalysis(ImageAnalysis {
						camera_id: camid,
						deviation,
					}))
					.await;

				_ = result.inspect_err(|_| {
					EdgeError::from(NetworkError::ServerConnection).log();
				});
			}

			if item.send_to_server {
				let mut encoded = CameraDevice::encode_image(&item.image);
				let result =
					websocket.send_image(item.camera_id, &mut encoded).await;
				_ = result.inspect_err(|_| {
					EdgeError::from(NetworkError::ServerConnection).log();
				});
			}
		}

		// Send statistics

		// This should guarantee that the loop will take at least this long.
		limit_interval.tick().await;

		Ok((websocket, settings))
	}

	async fn handle_request(
		&mut self,
		request: EdgeRequests,
		websocket: &mut WebSocket,
		requested_vision_type: &mut Option<VisionTechnique>,
	) {
		match request {
			EdgeRequests::SetNameCamera(cam_name_req) => {
				self.persistence.set_entry_name(cam_name_req.clone());
				match websocket
					.send_text(ServerRequests::SetNameCamera(cam_name_req))
					.await
				{
					Ok(()) => (),
					Err(_e) => {},
				}
			},
			EdgeRequests::StartViewingCamera(start_view_cam_req) => {
				self.persistence.start_viewing_camera(start_view_cam_req);
			},
			EdgeRequests::StopViewingCamera(stop_view_cam_req) => {
				self.persistence.stop_viewing_camera(stop_view_cam_req);
			},
			EdgeRequests::Configuration(set_config_req) => {
				self.persistence.set_configuration(set_config_req);
			},
			EdgeRequests::ViewCameraConfig(view_config_req) => {
				if let Some(configuration) =
					self.persistence.get_configuration(view_config_req)
					&& let Err(_) = websocket
						.send_text(ServerRequests::Configuration(configuration))
						.await
				{
					// TODO: Error type for "Failed to send configuration
					// request"
					EdgeError::from(NetworkError::WebSocketConnection).log();
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
		websocket: &mut WebSocket,
	) {
		let mut results = self.persistence.poll_for_new_cameras();

		let new_cameras = results.extract_if(|_, value| match value {
			ConnectionDetail::New { .. } => true,
			ConnectionDetail::Update { .. } => false,
		});

		let new_cameras: HashMap<_, _> = new_cameras
			.into_iter()
			.filter_map(|(key, value)| match value {
				ConnectionDetail::New { info } => Some((key, info)),
				ConnectionDetail::Update { .. } => None,
			})
			.collect();

		for (camera_id, update) in results {
			let ConnectionDetail::Update { status } = update else {
				continue;
			};
			_ = websocket
				.send_text(ServerRequests::UpdateCameraStatus(
					UpdateCameraStatus { camera_id, status },
				))
				.await;
		}

		if !new_cameras.is_empty() {
			_ = websocket
				.send_text(ServerRequests::RegisterCameras(RegisterCameras {
					cameras: new_cameras,
				}))
				.await;
		}
	}
}

impl Default for Conductor {
	fn default() -> Self {
		let persistence = Persistence::default();
		Self { persistence }
	}
}

/// Call this function to begin the system.
pub async fn run() -> Result<(), Box<dyn error::Error + Send + Sync>> {
	let mut websocket: Option<WebSocket> = None;
	let mut settings: Option<Settings> = None;
	let mut conductor = Conductor::default();
	let env_variables = EnvVariables::initialise();
	Logger::initialise_logger(env_variables.verbosity());

	loop {
		let (loop_websocket, loop_settings) =
			assert_connection(websocket, settings, &mut conductor.persistence, &env_variables)
				.await;
		let result = conductor.loop_once(loop_websocket, loop_settings).await?;
		websocket = Some(result.0);
		settings = Some(result.1);
	}
}
