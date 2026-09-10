use async_std::task;
use image::ImageBuffer;
use server_edge_datatypes::{
	edge::{ImageAnalysis, RegisterCameras},
	shared::{CameraStatus, Configuration, ViewCameraConfig},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::{
	client::Client,
	error::EdgeError,
	logger::logger,
	settings::Settings,
	tcam_interface::{Metadata, Persistence},
	vision_processor::VisionVariant,
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
		stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
	) -> Result<Settings, Box<dyn std::error::Error + Send + Sync>> {
		// TODO: Delay for polling cameras, so it doesn't happen every loop.

		// This function detects new cameras and instantiates them.
		let new_cameras = self.persistence.poll_for_new_cameras();

		self.client
			.send_register_cameras(
				stream,
				RegisterCameras {
					cameras: new_cameras,
				},
			)
			.await?;

		// TODO: Check if we have a request to modify camera config.
		let example = Configuration {
			camera_id: 598275214,
			conn_id: 12,
			configuration: "{\"Saturation\":{\"IntegerRange\":{\"control_type\":\"Saturation\",\"value\":64,\"default\":64,\"step\":1,\"min\":0,\"max\":128}},\"Brightness\":{\"IntegerRange\":{\"control_type\":\"Brightness\",\"value\":0,\"default\":0,\"step\":1,\"min\":-64,\"max\":64}},\"Contrast\":{\"IntegerRange\":{\"control_type\":\"Contrast\",\"value\":32,\"default\":32,\"step\":1,\"min\":0,\"max\":64}},\"Gain\":{\"IntegerRange\":{\"control_type\":\"Gain\",\"value\":0,\"default\":0,\"step\":1,\"min\":0,\"max\":100}},\"Gamma\":{\"IntegerRange\":{\"control_type\":\"Gamma\",\"value\":100,\"default\":100,\"step\":1,\"min\":72,\"max\":500}},\"Hue\":{\"IntegerRange\":{\"control_type\":\"Hue\",\"value\":0,\"default\":0,\"step\":1,\"min\":-40,\"max\":40}}}".into(),
		};
		for (id, entry) in self.persistence.inner().iter_mut() {
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

		// TODO: Check if we should send a camera config.
		let example = ViewCameraConfig {
			camera_id: 9825723522,
			conn_id: 21,
		};
		for (id, entry) in self.persistence.inner().iter_mut() {
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
				self.client
					.send_configuration(stream, configuration)
					.await?;
				break;
			}
		}
		logger().lock().unwrap().send_logs();

		for active_metadata in self.persistence.filter_by(Box::new(|metadata| {
			metadata.status() == CameraStatus::Active
		})) {
			on_active_camera(active_metadata, settings.clone());
		}
		// Send logs.

		// Vision

		// Send result

		// Send statistics

		//let camera = DisconnectCamera { camera_id: 1 };

		let frame = ImageBuffer::new(800, 800);

		let image_analysis = ImageAnalysis {
			camera_id: 1,
			deviation: 1.0,
		};

		//self.client
		//	.send_disconnect_camera(stream, camera.clone())
		//	.await?;

		self.client.send_image(stream, frame.clone()).await?;
		self.client
			.send_image_analysis(stream, image_analysis.clone())
			.await?;
		self.client.websocket_incoming_data(stream).await?;
		self.client.keep_alive(stream).await?;

		task::sleep(settings.update_delay).await;

		Ok(settings)
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

	let mut stream = conductor.client.websocket_connect_manual().await;

	loop {
		let loop_settings = placeholder_autoconnect(settings);
		settings = Some(conductor.loop_once(loop_settings, &mut stream).await?)
	}
}

fn on_active_camera(metadata: &mut Metadata, settings: Settings) {
	if !read_image(metadata, settings.clone()) {
		metadata.add_error(settings.camera_err_increment);

		if metadata.err_accumulation() < settings.camera_err_threshold {
			metadata.status = CameraStatus::Disconnected;
			// Save to log.
			// Tell server disconnect.
		} else {
			metadata.status = CameraStatus::Broken;
			// Save to log.
			// Tell server bokrne.
		}
	}
	// Save error count to log.
}

fn read_image(metadata: &mut Metadata, settings: Settings) -> bool {
	let camera = metadata
		.camera_mut()
		.expect("camera expected by status being active");

	let Ok(image) = camera.get_image_shrink() else {
		// Save to log.
		// Drop camera.
		return false;
	};

	// Save to log.
	metadata.reduce_error(settings.camera_err_decrement);
	// Decode the frame.
	// TODO: Add decoding function here.
	// if !decode {
	// If out of memory, send log and panic.
	// Save to log.
	// return false;
	//}

	// Save to log.

	if metadata.currently_viewed {
		// Send to server.
	}

	// Create batch item.

	true
}

// Not a proper spot for these, but they demontrate the general flow.

// TODO: Place this in a more suitable module (not client).
pub fn placeholder_autoconnect(settings: Option<Settings>) -> Settings {
	let connection_established = true;

	if !connection_established {
		todo!("connect to the server");

		#[allow(unreachable_code)]
		placeholder_get_default_settings(settings);
	}

	Settings::default()
}

// TODO: Place this in a more suitable module (not client).
pub fn placeholder_get_default_settings(settings: Option<Settings>) -> Settings {
	// TODO: Get the default settings from the server
	// Build the settings struct from the received data.
	// If the server fails to send correct settings, use Settings::default() as
	// a fallback and report the issue to the server in as much detail as
	// possible.
	Settings::default()
}
