use std::collections::HashMap;
use async_std::task;
use image::{ImageBuffer};
use server_edge_datatypes::edge::{
	DisconnectCamera, ImageAnalysis, RegisterCameras
};
use server_edge_datatypes::shared::{
	Configuration, CameraState
};
use crate::{client::Client, settings, Camera, Demonstration};
use settings::UPDATE_DELAY;

/// # Conductor
///
/// The main execution context.
pub struct Conductor {
	camera: Camera,
	client: Client,
	demo: Demonstration,
}

impl Conductor {
	/// This should only be called once.
	///
	/// TODO: Error handling.
	pub async fn new() -> Self {
		let camera = Camera::new(0).unwrap();
		let client = Client::new();
		let demo = Demonstration::default();

		Self {
			camera,
			client,
			demo,
		}
	}




	pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

		// Structs for test websocket data
		let mut cameras_hashmap: HashMap<u64, (String, CameraState)> = HashMap::new();
		cameras_hashmap.insert(1, ("".to_string(), CameraState::Active));
		let cameras = RegisterCameras{
			cameras: cameras_hashmap
		};

		let camera = DisconnectCamera{camera_id: 1};

		let frame = ImageBuffer::new(800, 800);

		let configuration = Configuration{conn_id: 0, camera_id: 0, configuration: "".to_string()};

		let image_analysis = ImageAnalysis{camera_id: 1, intensity: 1.0, deviation: 1.0};

		let mut stream = self.client.websocket_connect_manual().await;

		loop {
			self.client.send_register_cameras(&mut stream, cameras.clone()).await?;
			self.client.send_disconnect_camera(&mut stream, camera.clone()).await?;
			self.client.send_configuration(&mut stream, configuration.clone()).await?;
			self.client.send_image(&mut stream, frame.clone()).await?;
			self.client.send_image_analysis(&mut stream, image_analysis.clone()).await?;
			self.client.websocket_incoming_data(&mut stream).await?;
			self.client.keep_alive(&mut stream).await?;
			task::sleep(UPDATE_DELAY).await;
		}

	}
}
