use std::collections::HashMap;

use async_std::task;
use image::ImageBuffer;
use server_edge_datatypes::{
	edge::{DisconnectCamera, ImageAnalysis, RegisterCameras},
	shared::{CameraState, Configuration},
};
use settings::UPDATE_DELAY;

use crate::{
	client::Client, logger, settings, tcam_interface::Camera, vision_processor::VisionVariant,
};

/// # Conductor
///
/// The main execution context.
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
		logger::logger()
			.lock()
			.unwrap()
			.log_info("Camera opened successfully");
		let client = Client::new();
		let vision_processor = VisionVariant::default();

		Self {
			camera,
			client,
			vision_processor,
		}
	}

	pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
		// TODO: Make this log happen after default values are read
		logger::logger()
			.lock()
			.unwrap()
			.log_info("Websocket construction successful.");

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
			task::sleep(UPDATE_DELAY).await;
		}
	}
}
