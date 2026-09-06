use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use image::{ImageBuffer};
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes, Bytes};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use server_edge_datatypes::edge::{
	RegisterCameras, DisconnectCamera, SetNameCamera, StartViewingCamera, StopViewingCamera, ImageAnalysis
};
use server_edge_datatypes::shared::{
	Configuration, ViewCameraConfig
};
use crate::settings::{API, HOST, PORT};

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
	pub async fn websocket_connect_manual(&mut self) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
		let url = "ws://".to_string() + HOST + ":" + PORT + API;
		let (stream, response) = connect_async(url).await.expect("Failed to connect");
		return stream
	}

	pub fn websocket_connect_automatic(self) {
		// TO DO automatic connection
	}
	pub async fn keep_alive(&mut self, stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
		let msg = "hello";
		stream.send(Message::Pong(Bytes::from(msg))).await?;
		Ok(())
	}
	pub async fn send_register_cameras(&mut self, stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, cameras: RegisterCameras) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut msg = "RegisterCameras".to_string();
		msg.push_str(&*serde_json::to_string(&cameras)?);
		stream.send(Message::Text(Utf8Bytes::from(msg))).await?;
		Ok(())
	}
	pub async fn send_disconnect_camera(&mut self, stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, camera: DisconnectCamera) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut msg = "DisconnectCamera".to_string();
		msg.push_str(&*serde_json::to_string(&camera)?);
		stream.send(Message::Text(Utf8Bytes::from(msg))).await?;
		Ok(())
	}

	pub async fn send_image(&mut self, stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, frame: ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let msg = frame.clone().into_raw();
		stream.send(Message::Binary(Bytes::from(msg))).await?;
		Ok(())
	}

	pub async fn send_configuration(&mut self, stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, configuration: Configuration) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut msg = "Configuration".to_string();
		msg.push_str(&*serde_json::to_string(&configuration)?);
		stream.send(Message::Text(Utf8Bytes::from(msg))).await?;
		Ok(())
	}
	pub async fn send_image_analysis(&mut self, stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, image_analysis: ImageAnalysis) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut msg = "ImageAnalysis".to_string();
		msg.push_str(&*serde_json::to_string(&image_analysis)?);
		stream.send(Message::Text(Utf8Bytes::from(msg))).await?;
		Ok(())
	}

	pub async fn websocket_incoming_data(&mut self, stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		while let Some(msg) = stream.next().await {
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
	use server_edge_datatypes::edge::{
		DisconnectCamera, ImageAnalysis, RegisterCameras
	};
	use server_edge_datatypes::shared::{
		Configuration, CameraState
	};
	use std::collections::HashMap;
	#[tokio::test]
	async fn send_register_cameras() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
		let mut client = Client::new();
		let mut stream = client.websocket_connect_manual().await;

		let mut cameras_hashmap: HashMap<u64, (String, CameraState)> = HashMap::new();
		cameras_hashmap.insert(1, ("".to_string(), CameraState::Active));
		let cameras = RegisterCameras{
			cameras: cameras_hashmap
		};
		client.send_register_cameras(&mut stream, cameras.clone()).await?;
		
		Ok(())
	}
	#[tokio::test]
	async fn send_disconnect_camera() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut client = Client::new();
		let mut stream = client.websocket_connect_manual().await;

		let camera = DisconnectCamera{camera_id: 1};

		client.send_disconnect_camera(&mut stream, camera.clone()).await?;

		Ok(())
	}

	#[tokio::test]
	async fn send_image() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut client = Client::new();
		let mut stream = client.websocket_connect_manual().await;
		let frame = ImageBuffer::new(800, 800);
		client.send_image(&mut stream, frame.clone()).await?;
		Ok(())
	}
	#[tokio::test]
	async fn send_configuration() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut client = Client::new();
		let mut stream = client.websocket_connect_manual().await;

		let configuration = Configuration{conn_id: 0, camera_id: 0, configuration: "".to_string()};
		client.send_configuration(&mut stream, configuration.clone()).await?;

		Ok(())
	}
	#[tokio::test]
	async fn send_image_analysis() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut client = Client::new();
		let mut stream = client.websocket_connect_manual().await;

		let image_analysis = ImageAnalysis{camera_id: 1, intensity: 1.0, deviation: 1.0};
		client.send_image_analysis(&mut stream, image_analysis.clone()).await?;

		Ok(())
	}
	#[tokio::test]
	async fn websocket_incoming_data() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut client = Client::new();
		let mut stream = client.websocket_connect_manual().await;
		client.websocket_incoming_data(&mut stream).await?;
		Ok(())
	}
}
