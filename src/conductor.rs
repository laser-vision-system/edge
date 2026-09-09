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
	client::Client,
	settings::Settings,
	tcam_interface::{Metadata, Persistence, Status},
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

		for active_metadata in self
			.persistence
			.filter_by(Box::new(|metadata| metadata.status() == Status::Active))
		{
			on_active_camera(active_metadata, settings.clone());
		}
		// Send logs.

		// Vision

		// Send result

		// Send statistics

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

		task::sleep(settings.update_delay).await;

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

fn on_active_camera(metadata: &mut Metadata, settings: Settings) {
	if !read_image(metadata, settings.clone()) {
		metadata.add_error(settings.camera_err_increment);

		if metadata.err_accumulation() < settings.camera_err_threshold {
			metadata.status = Status::Disconnected;
			// Save to log.
			// Tell server disconnect.
		} else {
			metadata.status = Status::Broken;
			// Save to log.
			// Tell server bokrne.
		}
	}
	// Save error count to log.
}

fn read_image(metadata: &mut Metadata, settings: Settings) -> bool {
	let camera = metadata
		.camera_mut()
		.expect("camera expected by status being active");

	let Ok(image) = camera.get_image_shrink() else {
		// Save to log.
		// Drop camera.
		return false;
	};

	// Save to log.
	metadata.reduce_error(settings.camera_err_decrement);
	// Decode the frame.
	// TODO: Add decoding function here.
	// if !decode {
	// If out of memory, send log and panic.
	// Save to log.
	// return false;
	//}

	// Save to log.

	if metadata.currently_viewed {
		// Send to server.
	}

	// Create batch item.

	true
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
