use crate::{
	client::Client, settings, tcam_interface::Camera,
	vision_processor::demonstration::Demonstration,
};
use async_std::task;
use server_edge_datatypes::edge::{DisconnectCamera, ImageAnalysis, RegisterCameras};
use server_edge_datatypes::shared::{CameraState, Configuration};
use settings::UPDATE_DELAY;
use std::collections::HashMap;

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
		let cameras = RegisterCameras {
			cameras: cameras_hashmap,
		};

		let camera = DisconnectCamera { camera_id: 1 };

		let camera_id = 1;

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
			let frame = self.camera.get_image()?;
			self.client
				.send_register_cameras(&mut stream, cameras.clone())
				.await?;
			self.client
				.send_disconnect_camera(&mut stream, camera.clone())
				.await?;
			self.client
				.send_configuration(&mut stream, configuration.clone())
				.await?;
			self.client.send_image(&mut stream, camera_id, frame.clone()).await?;
			self.client
				.send_image_analysis(&mut stream, image_analysis.clone())
				.await?;
			self.client.websocket_incoming_data(&mut stream).await?;
			self.client.keep_alive(&mut stream).await?;
			task::sleep(UPDATE_DELAY).await;
		}
	}
}
