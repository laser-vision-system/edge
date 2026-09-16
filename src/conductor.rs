use std::collections::HashMap;

use server_edge_datatypes::{
	edge::{EdgeRequests, ImageAnalysis, RegisterCameras, UpdateCameraStatus},
	shared::{CameraStatus, VisionTechnique},
};
use tokio::time::interval;

use crate::{
	auto_connect::assert_connection,
	error::EdgeError,
	logger::logger,
	settings::Settings,
	tcam_interface::{
		CameraDevice, Persistence,
		active::{MaybeBatch, WhyNo},
		polling::ConnectionDetail,
	},
	vision_processor::{VisionInterface, VisionVariant, vision},
	websocket::WebSocket,
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
		vision_processor: VisionVariant,
	) -> Result<(WebSocket, Settings, VisionVariant), Box<dyn std::error::Error + Send + Sync>> {
		let mut limit_interval = interval(settings.update_delay);

		// TODO: Delay for polling cameras, so it doesn't happen every loop.
		self.update_camera_device_connections(&mut websocket).await;

		// Save the most recent vision swap request, instead of trying to swap
		// more than once.
		let mut requested_vision_type = None;
		while let Some(request) = websocket
			.query_websockets(settings.websocket_poll_interval)
			.await
		{
			self.handle_request(request, &mut websocket, &mut requested_vision_type)
				.await;
		}
		logger().lock().unwrap().send_logs();

		let _ = websocket.keep_alive().await;

		let vision_processor = match requested_vision_type {
			Some(swap_to) => vision_processor.swap(swap_to),
			None => vision_processor,
		};

		// TODO: There should be something in between this call that does the below.
		let batch_items = self.persistence.poll_active_entries(
			settings.camera_err_increment,
			settings.camera_err_threshold,
			settings.camera_err_decrement,
		);

		let mut items = Vec::with_capacity(batch_items.len());

		// TODO: Currently, the actual camera status is not being updated.
		for (maybebatch, cam_id) in batch_items {
			match maybebatch {
				MaybeBatch::Batch(batch) => items.push(batch),
				MaybeBatch::WhyNo(whyno) => match whyno {
					WhyNo::Disconnected => {
						let _ = websocket
							.send_text(
								"UpdateCameraStatus:".into(),
								UpdateCameraStatus {
									camera_id: cam_id,
									status: CameraStatus::Disconnected,
								},
							)
							.await;
					},
					WhyNo::Broken => {
						let _ = websocket
							.send_text(
								"UpdateCameraStatus:".into(),
								UpdateCameraStatus {
									camera_id: cam_id,
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
		vision().lock().unwrap().process(&mut items);

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
				let mut encoded = CameraDevice::encode_image(item.image.clone());
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

	async fn handle_request(
		&mut self,
		request: EdgeRequests,
		websocket: &mut WebSocket,
		requested_vision_type: &mut Option<VisionTechnique>,
	) {
		match request {
			EdgeRequests::SetNameCamera(cam_name_req) => {
				self.persistence.set_entry_name(cam_name_req);
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
				if let Some(configuration) = self.persistence.get_configuration(view_config_req) {
					if let Err(_) = websocket
						.send_text("Configuration:".into(), configuration)
						.await
					{
						logger().lock().unwrap().log_error(
							"Failed to send configuration request",
							&EdgeError::WebSocketConnection,
						);
					}
				}
			},
			EdgeRequests::Setpoint(calibration_req) => {
				self.persistence.request_calibration(calibration_req);
			},
			EdgeRequests::SetProcessingType(processing_req) => {
				requested_vision_type.replace(processing_req.processing_type);
			},
			_ => {
				logger()
					.lock()
					.unwrap()
					.log_warn("received request that is not currently supported");
			},
		}
	}

	async fn update_camera_device_connections(&mut self, websocket: &mut WebSocket) {
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
			let _ = websocket
				.send_text(
					"UpdateCameraStatus:".into(),
					UpdateCameraStatus { camera_id, status },
				)
				.await;
		}

		if !new_cameras.is_empty() {
			let _ = websocket
				.send_text(
					"RegisterCameras:".into(),
					RegisterCameras {
						cameras: new_cameras,
					},
				)
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
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	let mut websocket: Option<WebSocket> = None;
	let mut settings: Option<Settings> = None;
	let mut conductor = Conductor::default();
	let mut vision_processor = VisionVariant::default();

	loop {
		let (loop_websocket, loop_settings) = assert_connection(websocket, settings).await;
		let result = conductor
			.loop_once(loop_websocket, loop_settings, vision_processor)
			.await?;
		websocket = Some(result.0);
		settings = Some(result.1);
		vision_processor = result.2;
	}
}
