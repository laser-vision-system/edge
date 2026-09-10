use crate::{
	client::Client, settings, tcam_interface::Camera,
	vision_processor::demonstration::Demonstration,
};
use server_edge_datatypes::edge::{DisconnectCamera, ImageAnalysis, RegisterCameras};
use server_edge_datatypes::shared::{CameraState, Configuration};
use settings::UPDATE_DELAY;
use std::collections::HashMap;
use futures_util::StreamExt;
use tokio_tungstenite::tungstenite::Message;
use tokio::{time::interval};

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

		let mut interval = interval(UPDATE_DELAY);

		let mut stream = self.client.websocket_connect_manual().await;

		self.client
		.send_text(&mut stream, "RegisterCameras:".to_string(), cameras.clone())
		.await?;


		loop {
			tokio::select! {
					Some(Ok(msg)) = stream.next() => {
						 match msg {
							Message::Text(text) => {
							println!("{}", text );
							self.client.websocket_incoming_data(&text).await;
						}

                    	Message::Ping(_) | Message::Pong(_) | Message::Close(_) | Message::Frame(_) => {},
						_ => {},

                		}
					}
					_ = interval.tick() => {
						let frame = self.camera.get_image_shrink()?;
						let compressed_frame = self.client.encode_image(frame.clone());

						self.client
						.send_text(&mut stream, "DisconnectCamera:".to_string(), camera.clone())
						.await?;

						self.client
						.send_text(&mut stream, "Configuration:".to_string(), configuration.clone())
						.await?;

						self.client
						.send_text(&mut stream, "ImageAnalysis:".to_string(), image_analysis.clone())
						.await?;

						self.client
						.send_image(&mut stream, camera_id,  compressed_frame.clone().as_mut())
						.await?;

						self.client.keep_alive(&mut stream).await?;
						}
					}
				}

		}
	}
