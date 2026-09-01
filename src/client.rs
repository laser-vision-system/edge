use std::collections::HashMap;
use std::io::{Read, Write};
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use image::{ImageBuffer};
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes, Bytes};
use tokio_tungstenite::accept_async;
use server_edge_datatypes::{RegisterCameras, DisconnectCamera, Configuration, SetNameCamera, StartViewingCamera, StopViewingCamera, ViewCameraConfig, ImageAnalysis};


pub struct Client {
	camera_list: SetNameCamera,
	start_viewing_camera: StartViewingCamera,
	stop_viewing_camera: StopViewingCamera,
	configuration: Configuration,
	view_camera_config: ViewCameraConfig,

}
impl Client {
	pub fn new() -> Self {
		let camera_list = SetNameCamera { camera_id: 0, name: "".to_string()};
		let start_viewing_camera = StartViewingCamera { camera_id: 0};
		let stop_viewing_camera = StopViewingCamera { camera_id: 0};
		let configuration = Configuration {conn_id: 0, camera_id: 0, configuration: "".to_string()};
		let view_camera_config = ViewCameraConfig { conn_id: 0, camera_id: 0 };

		Self {
			camera_list,
			start_viewing_camera,
			stop_viewing_camera,
			configuration,
			view_camera_config
		}
	}
	pub async fn websocket_connect_manual(self) -> Result<TcpListener, std::io::Error> {
		let listener = TcpListener::bind("127.0.0.1:9002").await;
		return listener
	}

	pub fn websocket_connect_automatic(self) {
		// TO DO automatic connection
	}
	pub async fn send_register_cameras(&mut self, stream: &mut TcpStream, cameras: RegisterCameras) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let ws = accept_async(stream).await?;
		let mut msg = "RegisterCameras".to_string();
		msg.push_str(&*serde_json::to_string(&cameras)?);
		let (mut write, mut read) = ws.split();
		write.send(Message::Text(Utf8Bytes::from(msg))).await?;
		Ok(())
	}
	pub async fn send_disconnect_camera(&mut self, stream: &mut TcpStream, camera: DisconnectCamera) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let ws = accept_async(stream).await?;
		let mut msg = "DisconnectCamera".to_string();
		msg.push_str(&*serde_json::to_string(&camera)?);
		let (mut write, mut read) = ws.split();
		write.send(Message::Text(Utf8Bytes::from(msg))).await?;
		Ok(())
	}

	pub async fn send_image(&mut self, stream: &mut TcpStream, frame: ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let ws = accept_async(stream).await?;
		let msg = frame.clone().into_raw();
		let (mut write, mut read) = ws.split();
		write.send(Message::Binary(Bytes::from(msg))).await?;
		Ok(())
	}

	pub async fn send_configuration(&mut self, stream: &mut TcpStream, configuration: Configuration) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let ws = accept_async(stream).await?;
		let mut msg = "Configuration".to_string();
		msg.push_str(&*serde_json::to_string(&configuration)?);
		let (mut write, mut read) = ws.split();
		write.send(Message::Text(Utf8Bytes::from(msg))).await?;
		Ok(())
	}
	pub async fn send_image_analysis(&mut self, stream: &mut TcpStream, image_analysis: ImageAnalysis) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let ws = accept_async(stream).await?;
		let mut msg = "ImageAnalysis".to_string();
		msg.push_str(&*serde_json::to_string(&image_analysis)?);
		let (mut write, mut read) = ws.split();
		write.send(Message::Text(Utf8Bytes::from(msg))).await?;
		Ok(())
	}

	pub async fn websocket_incoming_data(&mut self, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let ws = accept_async(stream).await?;
		let (mut write, mut read) = ws.split();
		while let Some(msg) = read.next().await {
			match msg? {
				Message::Text(text) => {
					let msg = text.trim();

					let mut args = msg.splitn(2, ':');

					match args.next().unwrap() {
						"SetNameCamera" => match args.next() {
							Some(data) => {
								let data_unchecked: serde_json::Result<SetNameCamera> = serde_json::from_str(data);
								if data_unchecked.is_ok() {
									let json = data_unchecked.unwrap();
									self.camera_list = SetNameCamera {camera_id: json.camera_id, name: json.name}
								}
							}
							None => {}
						}
						"StartViewingCamera" => match args.next() {
							Some(data) => {
								let data_unchecked: serde_json::Result<StartViewingCamera> = serde_json::from_str(data);
								if data_unchecked.is_ok() {
									let json = data_unchecked.unwrap();
									self.start_viewing_camera = StartViewingCamera {camera_id: json.camera_id};
								}
							}
							None => {}
						}
						"StopViewingCamera" => match args.next() {
							Some(data) => {
								let data_unchecked: serde_json::Result<StopViewingCamera> = serde_json::from_str(data);
								if data_unchecked.is_ok() {
									let json = data_unchecked.unwrap();
									self.stop_viewing_camera = StopViewingCamera {camera_id: json.camera_id};
								}
							}
							None => {}
						}
						"Configuration" => match args.next() {
							Some(data) => {
								let data_unchecked: serde_json::Result<Configuration> = serde_json::from_str(data);
								if data_unchecked.is_ok() {
									let json = data_unchecked.unwrap();
									self.configuration = Configuration {conn_id: json.conn_id, camera_id: json.camera_id, configuration: json.configuration};
								}
							}
							None => {}
						}
						"ViewCameraConfig" => match args.next() {
							Some(data) => {
								let data_unchecked: serde_json::Result<ViewCameraConfig> = serde_json::from_str(data);
								if data_unchecked.is_ok() {
									let json = data_unchecked.unwrap();
									self.view_camera_config = ViewCameraConfig {conn_id: json.conn_id, camera_id: json.camera_id};
								}
							}
							None => {}
						},
						_ => {}
					}

				}
				Message::Ping(_) | Message::Pong(_) | Message::Close(_) | Message::Frame(_) => {}

				_ => {}
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use server_edge_datatypes::{RegisterCameras, DisconnectCamera, Configuration, CameraState, ImageAnalysis};
	#[tokio::test]
	async fn send_register_cameras() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
		let mut client = Client::new();

		let listener = TcpListener::bind("127.0.0.1:9002").await?;
		let (mut stream, _) = listener.accept().await?;

		let mut cameras_hashmap: HashMap<u64, (String, CameraState)> = HashMap::new();
		cameras_hashmap.insert(1, ("".to_string(), CameraState::Active));
		let cameras = RegisterCameras{
			cameras: cameras_hashmap
		};

		while listener.accept().await.is_ok() {
			client.send_register_cameras(&mut stream, cameras.clone()).await?;
		}

		Ok(())
	}
	#[tokio::test]
	async fn send_disconnect_camera() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut client = Client::new();

		let listener = TcpListener::bind("127.0.0.1:9002").await?;
		let (mut stream, _) = listener.accept().await?;

		let camera = DisconnectCamera{camera_id: 1};

		while listener.accept().await.is_ok() {
			client.send_disconnect_camera(&mut stream, camera.clone()).await?;
		}

		Ok(())
	}

	#[tokio::test]
	async fn send_image() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut client = Client::new();

		let listener = TcpListener::bind("127.0.0.1:9002").await?;
		let (mut stream, _) = listener.accept().await?;

		let frame = ImageBuffer::new(800, 800);

		while listener.accept().await.is_ok() {
			client.send_image(&mut stream, frame.clone()).await?;
		}

		Ok(())
	}
	#[tokio::test]
	async fn send_configuration() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut client = Client::new();

		let listener = TcpListener::bind("127.0.0.1:9002").await?;
		let (mut stream, _) = listener.accept().await?;

		let configuration = Configuration{conn_id: 0, camera_id: 0, configuration: "".to_string()};

		while listener.accept().await.is_ok() {
			client.send_configuration(&mut stream, configuration.clone()).await?;
		}

		Ok(())
	}
	#[tokio::test]
	async fn send_image_analysis() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut client = Client::new();

		let listener = TcpListener::bind("127.0.0.1:9002").await?;
		let (mut stream, _) = listener.accept().await?;

		let image_analysis = ImageAnalysis{camera_id: 1, intensity: 1.0, deviation: 1.0};

		while listener.accept().await.is_ok() {
			client.send_image_analysis(&mut stream, image_analysis.clone()).await?;
		}

		Ok(())
	}
	#[tokio::test]
	async fn websocket_incoming_data() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut client = Client::new();

		let listener = TcpListener::bind("127.0.0.1:9002").await?;
		let (mut stream, _) = listener.accept().await?;

		while listener.accept().await.is_ok() {
			client.websocket_incoming_data(&mut stream).await?;
		}

		Ok(())
	}
}
