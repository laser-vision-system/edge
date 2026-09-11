use server_edge_datatypes::{
	edge::{ImageAnalysis, RegisterCameras, UpdateCameraStatus},
	shared::{CameraStatus, Configuration, ViewCameraConfig},
};

use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio::time::interval;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;

use crate::{
	client::{Client, WebSocket},
	error::EdgeError,
	logger::logger,
	settings::Settings,
	tcam_interface::{Camera, Persistence, WhyNo},
	vision_processor::{VisionInterface, VisionVariant},
	auto_connect::websocket_connect_automatic
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
		settings: Settings,
		stream: &mut WebSocket,
	) -> Result<Settings, Box<dyn std::error::Error + Send + Sync>> {
		let mut limit_interval = interval(settings.update_delay);

		// TODO: Delay for polling cameras, so it doesn't happen every loop.

		// This function detects new cameras and instantiates them.
		let new_cameras = self.persistence.poll_for_new_cameras();

		self.client
			.send_text(
				stream,
				"RegisterCameras:".into(),
				RegisterCameras {
					cameras: new_cameras,
				},
			)
			.await?;

		while self.query_websockets(settings.clone(), stream).await {}

		self.client.keep_alive(stream).await?;

		//placeholder_config_request(&mut self.persistence);

		//placeholder_send_config(&mut self.persistence);

		let batch_items = self.persistence.run_on_active(settings.clone());

		let mut items = Vec::with_capacity(batch_items.len());

		for item in batch_items {
			match item {
				Ok(batch) => items.push(batch),
				Err((_, details)) => match details {
					WhyNo::Disconnected(id) => {
						let _ = self
							.client
							.send_text(
								stream,
								"UpdateCameraStatus:".into(),
								UpdateCameraStatus {
									camera_id: id,
									status: CameraStatus::Disconnected,
								},
							)
							.await;
					},
					WhyNo::Broken(id) => {
						let _ = self
							.client
							.send_text(
								stream,
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
				let result = self
					.client
					.send_text(
						stream,
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
				let result = self
					.client
					.send_image(stream, item.camera_id, &mut encoded)
					.await;
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

		Ok(settings)
	}

	async fn query_websockets(&mut self, settings: Settings, stream: &mut WebSocket) -> bool {
		let mut interval = interval(settings.websocket_poll_interval);
		tokio::select! {
			Some(Ok(msg)) = stream.next() => {
				match msg {
					Message::Text(text) => {
						println!("{}", text );
						// TODO
						self.client.websocket_incoming_data(&text).await;
						true
					}
					_ => {true},
				}
			}
			_ = interval.tick() => false
		}
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
	let mut settings: Option<Settings> = None;
	let mut conductor = Conductor::default();

	let url = websocket_connect_automatic().await?;
	let mut stream = conductor.client.websocket_connect(url).await;


	loop {
		let loop_settings = connect_loop(settings);
		settings = Some(conductor.loop_once(loop_settings, &mut stream).await?)
	}
}

// Not a proper spot for these, but they demontrate the general flow.

// TODO: Place this in a more suitable module (not client).

// TODO: Place this in a more suitable module (not client).
fn placeholder_get_default_settings(settings: Option<Settings>) -> Settings {
	// TODO: Get the default settings from the server
	// Build the settings struct from the received data.
	// If the server fails to send correct settings, use Settings::default() as
	// a fallback and report the issue to the server in as much detail as
	// possible.
	Settings::default()
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
