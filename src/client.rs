use futures_util::SinkExt;
use reqwest::Client as ReqwestClient;
use serde::Serialize;
use serde::de::DeserializeOwned;
use server_edge_datatypes::{
	edge::{SetNameCamera, StartViewingCamera, StopViewingCamera},
	shared::{Configuration, ViewCameraConfig},
};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{Bytes, Message, Utf8Bytes};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use crate::settings::{API, HOST, PORT};

pub type WebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct Client {
	camera_list: SetNameCamera,
	start_viewing_camera: StartViewingCamera,
	stop_viewing_camera: StopViewingCamera,
	configuration: Configuration,
	view_camera_config: ViewCameraConfig,
	restful_client: ReqwestClient,
}

impl Client {
	pub fn identify_network(&mut self) {}

	pub fn new() -> Self {
		let camera_list = SetNameCamera {
			camera_id: 0,
			name: "".to_string(),
		};
		let start_viewing_camera = StartViewingCamera { camera_id: 0 };
		let stop_viewing_camera = StopViewingCamera { camera_id: 0 };
		let configuration = Configuration {
			conn_id: 0,
			camera_id: 0,
			configuration: "".to_string(),
		};
		let view_camera_config = ViewCameraConfig {
			conn_id: 0,
			camera_id: 0,
		};

		Self {
			camera_list,
			start_viewing_camera,
			stop_viewing_camera,
			configuration,
			view_camera_config,
			restful_client: ReqwestClient::new(),
		}
	}

	pub async fn websocket_connect_manual(&mut self) -> WebSocket {
		let url = "ws://".to_string() + HOST + ":" + PORT + API;
		let (stream, response) = connect_async(url).await.expect("Failed to connect");
		return stream;
	}

	pub fn websocket_connect_automatic(self) {
		// TO DO automatic connection
	}

	pub fn unpack<T: DeserializeOwned>(data: &str) -> Option<T> {
		let data_unchecked: serde_json::Result<T> = serde_json::from_str(data);
		data_unchecked.ok()
	}

	pub async fn send_text<T: Serialize>(
		&mut self,
		stream: &mut WebSocket,
		mut prefix: String,
		data: T,
	) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		prefix.push_str(&*serde_json::to_string(&data)?);
		stream.send(Message::Text(Utf8Bytes::from(prefix))).await?;
		Ok(())
	}

	pub async fn keep_alive(
		&mut self,
		stream: &mut WebSocket,
	) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let msg = "hello";
		stream.send(Message::Pong(Bytes::from(msg))).await?;
		Ok(())
	}

	pub async fn send_image(
		&mut self,
		stream: &mut WebSocket,
		camera_id: u64,
		frame: &mut Vec<u8>,
	) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut msg = camera_id.to_be_bytes().to_vec();
		msg.append(frame);
		stream.send(Message::Binary(Bytes::from(msg))).await?;
		Ok(())
	}

	pub async fn websocket_incoming_data(&mut self, text: &str) {
		let msg = text.trim();

		println!("Received from Server: {}", msg);

		let mut args = msg.splitn(2, ':');

		match args.next().unwrap() {
			"SetNameCamera" => match args.next() {
				Some(data) => {
					self.camera_list = Self::unpack(data).unwrap();
				},
				None => {},
			},
			"StartViewingCamera" => match args.next() {
				Some(data) => {
					self.start_viewing_camera = Self::unpack(data).unwrap();
				},
				None => {},
			},
			"StopViewingCamera" => match args.next() {
				Some(data) => {
					self.stop_viewing_camera = Self::unpack(data).unwrap();
				},
				None => {},
			},
			"Configuration" => match args.next() {
				Some(data) => {
					self.configuration = Self::unpack(data).unwrap();
				},
				None => {},
			},
			"ViewCameraConfig" => match args.next() {
				Some(data) => {
					self.view_camera_config = Self::unpack(data).unwrap();
				},
				None => {},
			},
			_ => {},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use server_edge_datatypes::{
		edge::{NewCameraInfo, RegisterCameras},
		shared::CameraStatus,
	};

	use std::collections::HashMap;

	#[tokio::test]
	async fn send_text() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut client = Client::new();
		let mut stream = client.websocket_connect_manual().await;

		let mut cameras_hashmap: HashMap<u64, NewCameraInfo> = HashMap::new();
		cameras_hashmap.insert(1, NewCameraInfo::new("".to_string(), CameraStatus::Active));
		let cameras = RegisterCameras {
			cameras: cameras_hashmap,
		};
		client
			.send_text(&mut stream, "RegisterCameras:".to_string(), cameras.clone())
			.await?;

		Ok(())
	}

	#[tokio::test]
	async fn send_image() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let camera_id = 1;
		let mut client = Client::new();
		let mut stream = client.websocket_connect_manual().await;
		// let frame = ImageBuffer::new(800, 800);
		// client.send_image(&mut stream, camera_id,  frame.clone()).await?;
		Ok(())
	}

	#[tokio::test]
	async fn websocket_incoming_data() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut client = Client::new();
		let mut stream = client.websocket_connect_manual().await;
		// client.websocket_incoming_data(&mut stream).await?;
		Ok(())
	}
}
