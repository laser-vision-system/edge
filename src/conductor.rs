use std::collections::HashMap;
use std::time::Duration;
use image::{ImageBuffer};
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};
use tokio::io::{AsyncWriteExt, Interest};
use socket2;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};
use futures::{future, Sink, Stream, };
use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncReadExt;
// use tungstenite::{connect, Message, Utf8Bytes};
use server_edge_datatypes::{CameraState, Configuration, DisconnectCamera, ImageAnalysis, RegisterCameras};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::{Bytes, Utf8Bytes};
use crate::{
	Camera, Demonstration,
	client::Client,
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

		// Connection to WS
		// Keep alive WIP
		// let mut stream = TcpStream::connect("ws://127.0.0.1:8080/edge_ws").await?;
		let mut stream = self.client.websocket_connect_manual().await;

		loop {
			self.client.send_register_cameras(&mut stream, cameras.clone()).await?;
			self.client.send_disconnect_camera(&mut stream, camera.clone()).await?;
			self.client.send_configuration(&mut stream, configuration.clone()).await?;
			self.client.send_image(&mut stream, frame.clone()).await?;
			self.client.send_image_analysis(&mut stream, image_analysis.clone()).await?;
			self.client.keep_alive(&mut stream).await?;
		}

		println!("Connected to the edge connector");
		// let mut stream: TcpStream = socket;

		// stream.writable().await?;
		//
		//
		//
		//
		//

		// stream.readable().await?;
		// self.client.websocket_incoming_data(&mut stream).await?;

		Ok(())
	}
}
