use std::collections::HashMap;

use async_std::task;
use image::ImageBuffer;
use server_edge_datatypes::{
	edge::{DisconnectCamera, ImageAnalysis, RegisterCameras},
	shared::{CameraState, Configuration},
};

use crate::{
	client::Client, settings::Settings, tcam_interface::Camera, vision_processor::VisionVariant,
};

/// # Conductor
///
/// The main execution context.
///
/// TODO: Does this need to hold state?
pub struct Conductor {
	camera: Camera,
	client: Client,
	vision_processor: VisionVariant,
}

impl Conductor {
	/// This should only be called once.
	///
	/// TODO: Error handling.
	pub async fn new() -> Self {
		let camera = Camera::new(0).unwrap();
		let client = Client::new();
		let vision_processor = VisionVariant::default();

		Self {
			camera,
			client,
			vision_processor,
		}
	}

	pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut settings: Option<Settings> = None;
		loop {
			let loop_settings = placeholder_autoconnect(settings);

			settings = Some(loop_settings)
		}
	}

	pub async fn loop_once(
		&mut self,
		settings: Settings,
	) -> Result<Settings, Box<dyn std::error::Error + Send + Sync>> {
		// Structs for test websocket data
		let mut cameras_hashmap: HashMap<u64, (String, CameraState)> = HashMap::new();
		cameras_hashmap.insert(1, ("".to_string(), CameraState::Active));
		let cameras = RegisterCameras {
			cameras: cameras_hashmap,
		};

		let camera = DisconnectCamera { camera_id: 1 };

		let frame = ImageBuffer::new(800, 800);

		let configuration = Configuration {
			conn_id: 0,
			camera_id: 0,
			configuration: "".to_string(),
		};

		let image_analysis = ImageAnalysis {
			camera_id: 1,
			intensity: 1.0,
			deviation: 1.0,
		};

		let mut stream = self.client.websocket_connect_manual().await;

		loop {
			self.client
				.send_register_cameras(&mut stream, cameras.clone())
				.await?;
			self.client
				.send_disconnect_camera(&mut stream, camera.clone())
				.await?;
			self.client
				.send_configuration(&mut stream, configuration.clone())
				.await?;
			self.client.send_image(&mut stream, frame.clone()).await?;
			self.client
				.send_image_analysis(&mut stream, image_analysis.clone())
				.await?;
			self.client.websocket_incoming_data(&mut stream).await?;
			self.client.keep_alive(&mut stream).await?;
			task::sleep(settings.get_update_delay()).await;
		}
	}
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
