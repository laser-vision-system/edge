use std::collections::HashMap;
use std::net::TcpStream;
use image::{ImageBuffer};
use tungstenite::{Message, Utf8Bytes, WebSocket, Bytes, Result};
use tungstenite::stream::MaybeTlsStream;
use server_edge_datatypes as datatypes;
use server_edge_datatypes::{Configuration, SetNameCamera, StartViewingCamera, StopViewingCamera, ViewCameraConfig};

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
		let start_viewing_camera = StartViewingCamera { conn_id: 0, camera_id: 0};
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
	pub async fn send_register_cameras(&mut self, mut websocket: WebSocket<MaybeTlsStream<TcpStream>>, cameras: HashMap<u32, String>) {
		let mut msg = "RegisterCameras".to_string();
		let send = datatypes::RegisterCameras{
			cameras: cameras.clone()
		};
		msg.push_str(&*serde_json::to_string(&send).unwrap());
		websocket.send(Message::Text(Utf8Bytes::from(msg))).expect("TODO: panic message");
	}
	pub async fn send_unregister_camera(&mut self, mut websocket: WebSocket<MaybeTlsStream<TcpStream>>, camera: u32) {
		let mut msg = "UnregisterCamera".to_string();
		let send = datatypes::UnregisterCamera{
			camera_id: camera
		};
		msg.push_str(&*serde_json::to_string(&send).unwrap());
		websocket.send(Message::Text(Utf8Bytes::from(msg))).expect("TODO: panic message");
	}
	pub async fn send_image(&mut self, mut websocket: WebSocket<MaybeTlsStream<TcpStream>>, frame: ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
		websocket.send(Message::Binary(Bytes::from(frame.clone().into_raw()))).expect("TODO: panic message");
	}
	pub async fn send_configuration(&mut self, mut websocket: WebSocket<MaybeTlsStream<TcpStream>>, conn_id: u32, camera_id: u32, configuration: String) {
		let mut msg = "Configuration".to_string();
		let send = Configuration{
			conn_id, camera_id, configuration
		};
		msg.push_str(&*serde_json::to_string(&send).unwrap());
		websocket.send(Message::Text(Utf8Bytes::from(msg))).expect("TODO: panic message");
	}
	pub async fn send_image_analysis(&mut self, mut websocket: WebSocket<MaybeTlsStream<TcpStream>>, camera_id: u32, intensity: f32, deviation: f32) {
		let mut msg = "ImageAnalysis".to_string();
		let send = datatypes::ImageAnalysis{
			camera_id, intensity, deviation
		};
		msg.push_str(&*serde_json::to_string(&send).unwrap());
		websocket.send(Message::Text(Utf8Bytes::from(msg))).expect("TODO: panic message");
	}
	pub async fn websocket_incoming_data(&mut self, mut websocket: WebSocket<MaybeTlsStream<TcpStream>>,) -> Result<()>  {
		loop {
			match websocket.read()? {
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
									self.start_viewing_camera = StartViewingCamera {conn_id: json.conn_id, camera_id: json.camera_id};
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
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tungstenite::connect;
	#[tokio::test]
	async fn send_register_cameras() {
		let mut client = Client::new();
		let (websocket, _) = connect("ws://localhost:9001/getCaseCount").expect("Can't connect");
		let mut cameras = HashMap::new();
		cameras.insert(1, "".to_string());
		client.send_register_cameras(websocket, cameras).await;
	}
	#[tokio::test]
	async fn send_unregister_camera() {
		let mut client = Client::new();
		let (websocket, _) = connect("ws://localhost:9001/getCaseCount").expect("Can't connect");
		let camera = 1;
		client.send_unregister_camera(websocket, camera).await;
	}
	#[tokio::test]
	async fn send_image() {
		let mut client = Client::new();
		let (websocket, _) = connect("ws://localhost:9001/getCaseCount").expect("Can't connect");
		let frame = ImageBuffer::new(800, 800);
		client.send_image(websocket, frame).await;
	}
	#[tokio::test]
	async fn send_configuration() {
		let mut client = Client::new();
		let (websocket, _) = connect("ws://localhost:9001/getCaseCount").expect("Can't connect");
		client.send_configuration(websocket,1, 1, "configuration".to_string()).await;
	}
	#[tokio::test]
	async fn send_image_analysis() {
		let mut client = Client::new();
		let (websocket, _) = connect("ws://localhost:9001/getCaseCount").expect("Can't connect");
		client.send_image_analysis(websocket,1, 1.5, 1.5).await;
	}

	#[tokio::test]
	async fn websocket_incoming_data() {
		let mut client = Client::new();
		let (websocket, _) = connect("ws://localhost:9001/getCaseCount").expect("Can't connect");
		client.websocket_incoming_data(websocket).await.expect("TODO: panic message");
	}
}
