use crate::settings::{API, HOST, PORT};
use futures_util::SinkExt;
use image::ImageBuffer;
use serde::Serialize;
use serde::de::DeserializeOwned;
use server_edge_datatypes::{
	edge::{SetNameCamera, Setpoint, StartViewingCamera, StopViewingCamera},
	shared::{Configuration, ViewCameraConfig},
};
use std::io::Cursor;
use tokio;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{Bytes, Message, Utf8Bytes};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

pub struct Client {
	camera_list: SetNameCamera,
	start_viewing_camera: StartViewingCamera,
	stop_viewing_camera: StopViewingCamera,
	configuration: Configuration,
	view_camera_config: ViewCameraConfig,
	setpoint: Setpoint,
}

impl Client {
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
		let setpoint = Setpoint { camera_id: 0 };

		Self {
			camera_list,
			start_viewing_camera,
			stop_viewing_camera,
			configuration,
			view_camera_config,
			setpoint,
		}
	}
}
