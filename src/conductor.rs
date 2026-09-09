use std::collections::HashMap;

use async_std::task;
use image::ImageBuffer;
use server_edge_datatypes::{
	edge::{DisconnectCamera, ImageAnalysis, RegisterCameras},
	shared::{CameraState, Configuration},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::{
	client::Client, settings::Settings, tcam_interface::Persistence,
	vision_processor::VisionVariant,
};

/// # Conductor
///
/// The main execution context.
pub struct Conductor {
	persistence: Persistence,
	client: Client,
	vision_processor: VisionVariant,
}

impl Conductor {
	async fn loop_once(
		&mut self,
		settings: Settings,
		stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
	) -> Result<Settings, Box<dyn std::error::Error + Send + Sync>> {
		// TODO: Delay for polling cameras, so it doesn't happen every loop.

		// This function detects new cameras and instantiates them.
		let new_cameras = self.persistence.poll_for_new_cameras();

		// Structs for test websocket data
		// TODO: Replace with above.
		let mut cameras_hashmap: HashMap<u64, (String, CameraState)> = HashMap::new();
		cameras_hashmap.insert(1, ("".to_string(), CameraState::Active));
		let cameras = RegisterCameras {
			cameras: cameras_hashmap,
		};

		self.client
			.send_register_cameras(stream, cameras.clone())
			.await?;

		let camera = DisconnectCamera { camera_id: 1 };

		let frame = ImageBuffer::new(800, 800);

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

		self.client
			.send_disconnect_camera(stream, camera.clone())
			.await?;
		self.client
			.send_configuration(stream, configuration.clone())
			.await?;
		self.client.send_image(stream, frame.clone()).await?;
		self.client
			.send_image_analysis(stream, image_analysis.clone())
			.await?;
		self.client.websocket_incoming_data(stream).await?;
		self.client.keep_alive(stream).await?;

		task::sleep(settings.get_update_delay()).await;

		Ok(settings)
	}
}

impl Default for Conductor {
	fn default() -> Self {
		let persistence = Persistence::default();
		let client = Client::new();
		let vision_processor = VisionVariant::default();

		Self {
			persistence,
			client,
			vision_processor,
		}
	}
}

/// Call this function to begin the system.
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	let mut settings: Option<Settings> = None;
	let mut conductor = Conductor::default();

	let mut stream = conductor.client.websocket_connect_manual().await;

	loop {
		let loop_settings = placeholder_autoconnect(settings);
		settings = Some(conductor.loop_once(loop_settings, &mut stream).await?)
	}
}

// Not a proper spot for these, but they demontrate the general flow.

// TODO: Place this in a more suitable module (not client).
pub fn placeholder_autoconnect(settings: Option<Settings>) -> Settings {
	let connection_established = true;

	if !connection_established {
		todo!("connect to the server");

		#[allow(unreachable_code)]
		placeholder_get_default_settings(settings);
	}

	Settings::default()
}

// TODO: Place this in a more suitable module (not client).
pub fn placeholder_get_default_settings(settings: Option<Settings>) -> Settings {
	// TODO: Get the default settings from the server
	// Build the settings struct from the received data.
	// If the server fails to send correct settings, use Settings::default() as
	// a fallback and report the issue to the server in as much detail as
	// possible.
	Settings::default()
}
