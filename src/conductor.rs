use server_edge_datatypes::{
	edge::{ImageAnalysis, RegisterCameras, UpdateCameraStatus},
	shared::{CameraStatus, Configuration, ViewCameraConfig},
};

use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio::time::interval;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::{
	auto_connect::{connect_loop, websocket_connect_automatic},
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
		websocket: WebSocket,
		settings: Settings,
	) -> Result<(WebSocket, Settings), Box<dyn std::error::Error + Send + Sync>> {
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

		let mut interval = interval(settings.websocket_poll_interval);

		while websocket.query_websockets(interval).await {}

		websocket.keep_alive().await?;

		//placeholder_config_request(&mut self.persistence);

		//placeholder_send_config(&mut self.persistence);

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

		Ok((websocket, settings))
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

	loop {
		let (loop_websocket, loop_settings) = connect_loop(websocket, settings).await.unwrap();
		let result = conductor.loop_once(loop_websocket, loop_settings).await?;
		websocket = Some(result.0);
		settings = Some(result.1);
	}
}

fn placeholder_config_request(persistence: &mut Persistence) {
	// TODO: Check if we have a request to modify camera config.
	let example = Configuration {
			camera_id: 598275214,
			conn_id: 12,
			configuration: "{\"Saturation\":{\"IntegerRange\":{\"control_type\":\"Saturation\",\"value\":64,\"default\":64,\"step\":1,\"min\":0,\"max\":128}},\"Brightness\":{\"IntegerRange\":{\"control_type\":\"Brightness\",\"value\":0,\"default\":0,\"step\":1,\"min\":-64,\"max\":64}},\"Contrast\":{\"IntegerRange\":{\"control_type\":\"Contrast\",\"value\":32,\"default\":32,\"step\":1,\"min\":0,\"max\":64}},\"Gain\":{\"IntegerRange\":{\"control_type\":\"Gain\",\"value\":0,\"default\":0,\"step\":1,\"min\":0,\"max\":100}},\"Gamma\":{\"IntegerRange\":{\"control_type\":\"Gamma\",\"value\":100,\"default\":100,\"step\":1,\"min\":72,\"max\":500}},\"Hue\":{\"IntegerRange\":{\"control_type\":\"Hue\",\"value\":0,\"default\":0,\"step\":1,\"min\":-40,\"max\":40}}}".into(),
		};
	for (id, entry) in persistence.inner().iter_mut() {
		if id.calculate_hash() == example.camera_id {
			let Some(camera) = entry.camera_mut() else {
				logger()
					.lock()
					.unwrap()
					.log_error("", &EdgeError::CameraNotPresent);
				break;
			};
			match serde_json::from_str(&example.configuration) {
				Ok(map) => {
					if let Err(why) = camera.set_controls(map) {
						logger()
							.lock()
							.unwrap()
							.log_error("Failed to set requested cntrols from server", &why);
					};
				},
				Err(why) => {
					logger().lock().unwrap().log_error(
						"Failed to parse requested configuration",
						&EdgeError::DecodeControls { reason: why },
					);
				},
			};

			logger()
				.lock()
				.unwrap()
				.log_info("We've found the correct camera by id");
			break;
		}
	}
	logger().lock().unwrap().send_logs();
}

fn placeholder_send_config(persistence: &mut Persistence) {
	// TODO: Check if we should send a camera config.
	let example = ViewCameraConfig {
		camera_id: 9825723522,
		conn_id: 21,
	};
	for (id, entry) in persistence.inner().iter_mut() {
		if id.calculate_hash() == example.camera_id {
			let Some(camera) = entry.camera() else {
				logger()
					.lock()
					.unwrap()
					.log_error("", &EdgeError::CameraNotPresent);
				break;
			};
			let important_controls = camera.get_controls().stripped();
			let configuration =
				Configuration::build_response(example, important_controls.serialise());
			//self.client
			//	.send_configuration(stream, configuration)
			//	.await?;
			break;
		}
	}
	logger().lock().unwrap().send_logs();
}
