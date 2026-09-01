use std::collections::HashMap;
use std::net::TcpStream;
use image::{ImageBuffer};
use tungstenite::{connect, Message, Utf8Bytes, WebSocket, Bytes, Result};
use tungstenite::stream::MaybeTlsStream;
use server_edge_datatypes as datatypes;
use server_edge_datatypes::{RegisterCameras, UnregisterCamera, Configuration, SetNameCamera, StartViewingCamera, StopViewingCamera, ViewCameraConfig, ImageAnalysis};

pub struct Client {
	websocket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
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
			websocket: None,
			camera_list,
			start_viewing_camera,
			stop_viewing_camera,
			configuration,
			view_camera_config
		}
	}
	pub fn websocket_connect_manual(&mut self) {
		let (websocket, _) = connect("ws://localhost:9001/getCaseCount").expect("Can't connect");
		self.websocket = Option::from(websocket);
	}
	pub fn websocket_connect_automatic(&mut self) {
		// TO DO automatic connection
	}
	pub async fn send_register_cameras(self, cameras: RegisterCameras) {
		let mut msg = "RegisterCameras".to_string();
		msg.push_str(&*serde_json::to_string(&cameras).unwrap());
		self.websocket.unwrap().send(Message::Text(Utf8Bytes::from(msg))).expect("TODO: panic message");
	}
	pub async fn send_unregister_camera(&mut self, camera: UnregisterCamera) {
		let mut msg = "UnregisterCamera".to_string();
		msg.push_str(&*serde_json::to_string(&camera).unwrap());
		self.websocket.unwrap().send(Message::Text(Utf8Bytes::from(msg))).expect("TODO: panic message");
	}
	pub async fn send_image(self, frame: ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
		self.websocket.unwrap().send(Message::Binary(Bytes::from(frame.clone().into_raw()))).expect("TODO: panic message");
	}
	pub async fn send_configuration(self, configuration: Configuration) {
		let mut msg = "Configuration".to_string();
		msg.push_str(&*serde_json::to_string(&configuration).unwrap());
		self.websocket.unwrap().send(Message::Text(Utf8Bytes::from(msg))).expect("TODO: panic message");
	}
	pub async fn send_image_analysis(self, image_analysis: ImageAnalysis) {
		let mut msg = "ImageAnalysis".to_string();
		msg.push_str(&*serde_json::to_string(&image_analysis).unwrap());
		self.websocket.unwrap().send(Message::Text(Utf8Bytes::from(msg))).expect("TODO: panic message");
	}
	pub async fn websocket_incoming_data(mut self) -> Result<()>  {
		loop {
			match self.websocket.as_mut().unwrap().read()? {
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
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use server_edge_datatypes::{RegisterCameras, UnregisterCamera, Configuration, SetNameCamera, StartViewingCamera, StopViewingCamera, ViewCameraConfig, CameraState, ImageAnalysis};
	#[tokio::test]
	async fn send_register_cameras() {
		let client = Client::new();
		let mut cameras_hashmap: HashMap<u64, (String, CameraState)> = HashMap::new();
		cameras_hashmap.insert(1, ("".to_string(), CameraState::Active));
		let cameras = RegisterCameras{
			cameras: cameras_hashmap
		};
		client.send_register_cameras(cameras).await;
	}
	#[tokio::test]
	async fn send_unregister_camera() {
		let mut client = Client::new();
		let camera = UnregisterCamera{camera_id: 1};
		client.send_unregister_camera(camera).await;
	}
	#[tokio::test]
	async fn send_image() {
		let client = Client::new();
		let frame = ImageBuffer::new(800, 800);
		client.send_image(frame).await;
	}
	#[tokio::test]
	async fn send_configuration() {
		let client = Client::new();
		let configuration = Configuration{conn_id: 0, camera_id: 0, configuration: "".to_string()};
		client.send_configuration(configuration).await;
	}
	#[tokio::test]
	async fn send_image_analysis() {
		let client = Client::new();
		let image_analysis = ImageAnalysis{camera_id: 1, intensity: 1.0, deviation: 1.0};
		client.send_image_analysis(image_analysis).await;
	}

	#[tokio::test]
	async fn websocket_incoming_data() {
		let client = Client::new();
		client.websocket_incoming_data().await.expect("TODO: panic message");
	}
}
