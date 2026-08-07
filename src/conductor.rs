use std::thread;

use crate::{
	Camera, Client, Demonstration,
	client::CameraConfig,
	settings::{PRINT_RESULTS, UPDATE_DELAY},
	vision_processor::VisionInterface,
};

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

		let dummy_config = CameraConfig::new(30.0, 50, 0, 50);
		client.post_camera_config(dummy_config).await.unwrap();

		Self {
			camera,
			client,
			demo,
		}
	}

	pub async fn run(&mut self) {
		loop {
			let gui_update = self.client.get_gui_update().await.unwrap();

			if gui_update.config() {
				let config = self.client.get_camera_config().await.unwrap();
				// Not implemented in camera yet, just print results for now.
				println!("Received new config: {config:?}");
			}

			let frame = self.camera.get_image_shrink().unwrap();
			let readings = self.demo.process_frame(&frame, gui_update.setpoint());

			self.client.post_readings(readings).await.unwrap();
			self.client.put_image(frame).await.unwrap();

			if PRINT_RESULTS {
				println!(
					"Percieved lightness: {:.4} | Deviation: {:.4} | Press Ctrl+C to stop.",
					readings.intensity(),
					readings.deviation()
				);
			}

			thread::sleep(UPDATE_DELAY);
		}
	}
}
