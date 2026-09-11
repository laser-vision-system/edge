use server_edge_datatypes::{
	edge::{EdgeRequests, ImageAnalysis, RegisterCameras, UpdateCameraStatus},
	shared::{CameraStatus, Configuration},
};
use tokio::time::interval;

use crate::{
	auto_connect::connect_loop,
	client::Client,
	error::EdgeError,
	logger::logger,
	settings::Settings,
	tcam_interface::{Camera, Persistence, WhyNo},
	vision_processor::{VisionInterface, VisionVariant},
	websocket::WebSocket,
};

/// # Conductor
///
/// The main execution context.
pub struct Conductor {
	persistence: Persistence,
	client: Client,
	vision_processor: VisionVariant,
}

impl Conductor {
	async fn loop_once(
		&mut self,
		mut websocket: WebSocket,
		settings: Settings,
		vision_processor: VisionVariant,
	) -> Result<(WebSocket, Settings, VisionVariant), Box<dyn std::error::Error + Send + Sync>> {
		let mut limit_interval = interval(settings.update_delay);

		// TODO: Delay for polling cameras, so it doesn't happen every loop.

		// This function detects new cameras and instantiates them.
		let new_cameras = self.persistence.poll_for_new_cameras();

		websocket
			.send_text(
				"RegisterCameras:".into(),
				RegisterCameras {
					cameras: new_cameras,
				},
			)
			.await?;

		let mut requested_type = None;

		while let Some(request) = websocket
			.query_websockets(settings.websocket_poll_interval)
			.await
		{
			match request {
				EdgeRequests::SetNameCamera(cam_name_req) => {
					if let Some(meta) = self.persistence.get_with_id(cam_name_req.camera_id) {
						meta.name = cam_name_req.name
					}
				},
				EdgeRequests::StartViewingCamera(start_view_cam_req) => {
					if let Some(meta) = self.persistence.get_with_id(start_view_cam_req.camera_id) {
						meta.currently_viewed = true
					}
				},
				EdgeRequests::StopViewingCamera(stop_view_cam_req) => {
					if let Some(meta) = self.persistence.get_with_id(stop_view_cam_req.camera_id) {
						meta.currently_viewed = false
					}
				},
				EdgeRequests::Configuration(set_config_req) => {
					if let Some(meta) = self.persistence.get_with_id(set_config_req.camera_id) {
						let Some(camera) = meta.camera_mut() else {
							continue;
						};
						logger()
							.lock()
							.unwrap()
							.log_error("", &EdgeError::CameraNotPresent);
						match serde_json::from_str(&set_config_req.configuration) {
							Ok(map) => {
								if let Err(why) = camera.set_controls(map) {
									logger().lock().unwrap().log_error(
										"Failed to set requested cntrols from server",
										&why,
									);
								};
							},
							Err(why) => {
								logger().lock().unwrap().log_error(
									"Failed to parse requested configuration",
									&EdgeError::DecodeControls { reason: why },
								);
							},
						};
					}
				},
				EdgeRequests::ViewCameraConfig(view_config_req) => {
					if let Some(meta) = self.persistence.get_with_id(view_config_req.camera_id) {
						let Some(camera) = meta.camera() else {
							logger()
								.lock()
								.unwrap()
								.log_error("", &EdgeError::CameraNotPresent);
							continue;
						};
						let important_controls = camera.get_controls().stripped();
						let configuration = Configuration::build_response(
							view_config_req,
							important_controls.serialise(),
						);
						websocket.send_text("Configuration:".into(), configuration);
					}
				},
				EdgeRequests::Setpoint(calibration_req) => {
					if let Some(meta) = self.persistence.get_with_id(calibration_req.camera_id) {
						meta.request_calibration();
					}
				},
				EdgeRequests::SetProcessingType(processing_req) => {
					requested_type = Some(processing_req.processing_type);
				},
			}
		}
		logger().lock().unwrap().send_logs();

		websocket.keep_alive().await?;

		let vision_processor = match requested_type {
			Some(swap_to) => vision_processor.swap(swap_to),
			None => vision_processor,
		};

		let batch_items = self.persistence.run_on_active(settings.clone());

		let mut items = Vec::with_capacity(batch_items.len());

		for item in batch_items {
			match item {
				Ok(batch) => items.push(batch),
				Err((_, details)) => match details {
					WhyNo::Disconnected(id) => {
						let _ = websocket
							.send_text(
								"UpdateCameraStatus:".into(),
								UpdateCameraStatus {
									camera_id: id,
									status: CameraStatus::Disconnected,
								},
							)
							.await;
					},
					WhyNo::Broken(id) => {
						let _ = websocket
							.send_text(
								"UpdateCameraStatus:".into(),
								UpdateCameraStatus {
									camera_id: id,
									status: CameraStatus::Broken,
								},
							)
							.await;
					},
				},
			}
		}

		// Send logs.
		logger().lock().unwrap().send_logs();

		// Vision
		self.vision_processor.process(&mut items);

		// Send result
		for item in items.iter() {
			let deviation_maybe = item.state.get_deviation();
			let camid = item.camera_id;
			if let Some(deviation) = deviation_maybe {
				let result = websocket
					.send_text(
						"ImageAnalysis:".into(),
						ImageAnalysis {
							camera_id: camid,
							deviation,
						},
					)
					.await;

				let _ = result.inspect_err(|why| {
					logger().lock().unwrap().log_error(
						&format!("failed to send to server: {why}"),
						&EdgeError::ServerConnection,
					)
				});
			};

			if item.send_to_server {
				let mut encoded = Camera::encode_image(item.image.clone());
				let result = websocket.send_image(item.camera_id, &mut encoded).await;
				let _ = result.inspect_err(|why| {
					logger().lock().unwrap().log_error(
						&format!("failed to image send to server: {why}"),
						&EdgeError::ServerConnection,
					)
				});
			}
		}

		// Send statistics

		// This should guarantee that the loop will take at least this long.
		limit_interval.tick().await;

		Ok((websocket, settings, vision_processor))
	}
}

impl Default for Conductor {
	fn default() -> Self {
		let persistence = Persistence::default();
		let client = Client::new();
		let vision_processor = VisionVariant::default();

		Self {
			persistence,
			client,
			vision_processor,
		}
	}
}

/// Call this function to begin the system.
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	let mut websocket: Option<WebSocket> = None;
	let mut settings: Option<Settings> = None;
	let mut conductor = Conductor::default();
	let mut vision_processor = VisionVariant::default();

	loop {
		let (loop_websocket, loop_settings) = connect_loop(websocket, settings).await;
		let result = conductor
			.loop_once(loop_websocket, loop_settings, vision_processor)
			.await?;
		websocket = Some(result.0);
		settings = Some(result.1);
		vision_processor = result.2;
	}
}
