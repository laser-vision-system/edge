use std::collections::HashMap;
use std::io::{Cursor, Write};
use std::net::TcpStream;
use reqwest_websocket::{Upgrade};
use image::{EncodableLayout, ImageBuffer, RgbImage};
use tungstenite::{connect, Message, Utf8Bytes, WebSocket, Bytes};
use serde::{Deserialize, Serialize,};
use tungstenite::handshake::client::Response;
use tungstenite::stream::MaybeTlsStream;
use websocket::futures::Sink;
use server_edge_datatypes as datatypes;
use crate::{
	settings::{
		ENDPOINT_CAMERA_CONFIG, ENDPOINT_CAMERA_READINGS, ENDPOINT_GUI_UPDATE, ENDPOINT_IMAGE,
		HOST, PORT, API
	},
	vision_processor::CameraReadings,
};

pub struct Client {
	websocket: (WebSocket<MaybeTlsStream<TcpStream>>),
}
impl Client {
	pub async fn new() -> Self {
		let (websocket, _) = connect("ws://localhost:9001/getCaseCount").expect("Can't connect");

		Self {
			websocket,
		}
	}
	pub async fn send_register_cameras(&mut self, cameras: HashMap<u32, String>) {
		let mut msg = "RegisterCameras".to_string();
		let send = datatypes::RegisterCameras{
			cameras: cameras.clone()
		};
		msg.push_str(&*serde_json::to_string(&send).unwrap());
		self.websocket.send(Message::Text(Utf8Bytes::from(msg))).expect("TODO: panic message");
	}
	pub async fn send_unregister_camera(&mut self, camera: u32) {
		let mut msg = "UnregisterCamera".to_string();
		let send = datatypes::UnregisterCamera{
			camera_id: camera
		};
		msg.push_str(&*serde_json::to_string(&send).unwrap());
		self.websocket.send(Message::Text(Utf8Bytes::from(msg))).expect("TODO: panic message");
	}
	pub async fn send_image(&mut self, frame: ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
		let mut msg = "Image".to_string();
		self.websocket.send(Message::Binary(Bytes::from(frame.clone().into_raw()))).expect("TODO: panic message");
	}
	pub async fn send_configuration(&mut self, conn_id: u32, camera_id: u32, configuration: String) {
		let mut msg = "Configuration".to_string();
		let send = datatypes::Configuration{
			conn_id, camera_id, configuration
		};
		msg.push_str(&*serde_json::to_string(&send).unwrap());
		self.websocket.send(Message::Text(Utf8Bytes::from(msg))).expect("TODO: panic message");
	}
	pub async fn send_image_analysis(&mut self, camera_id: u32, intensity: f32, deviation: f32) {
		let mut msg = "ImageAnalysis".to_string();
		let send = datatypes::ImageAnalysis{
			camera_id, intensity, deviation
		};
		msg.push_str(&*serde_json::to_string(&send).unwrap());
		self.websocket.send(Message::Text(Utf8Bytes::from(msg))).expect("TODO: panic message");
	}
}


#[cfg(test)]
mod tests {
	use std::process::Termination;

	use super::*;

	#[tokio::test]
	async fn post_readings() -> Result<(), reqwest_websocket::Error> {
		let client = Client::new();
		let reading = CameraReadings::new(72.2, 0.21);
		client.post_readings(reading).await
	}

	#[tokio::test]
	async fn post_camera_config() -> Result<(), reqwest::Error> {
		let client = Client::new();
		let config = CameraConfig::new(30.0, 100, 0, 100);
		client.send_configuration().await;
	}

	#[tokio::test]
	async fn get_camera_config() -> Result<CameraConfig, reqwest::Error> {
		let client = Client::new();
		client.get_camera_config().await
	}

	#[tokio::test]
	async fn get_gui_update() -> Result<GuiUpdate, reqwest::Error> {
		let client = Client::new();
		client.get_gui_update().await
	}

	#[tokio::test]
	async fn put_image() -> Result<(), reqwest::Error> {
		let client = Client::new();
		let image = ImageBuffer::<image::Rgb<u8>, Vec<_>>::new(50, 50);
		client.put_image(image).await
	}
}
