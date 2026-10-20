use api::{
    shared::{
        AlertType,
        CameraId,
        CameraStatus,
        ConnId,
        Deviation,
    },
};
use tokio::sync::mpsc::{
    UnboundedReceiver,
    UnboundedSender,
};

use crate::{
    error::EdgeClientResult,
    handler::basic_client_handler::BasicClientHandler,
    interface::commands::Command,
};

pub struct CameraClientHandler {
	camera_id: CameraId,
	base: BasicClientHandler,
}

/// Stripped down version of `ClientHandler`, with an internal `CameraId` and
/// access to camera-related websocket commands.
impl CameraClientHandler {
	pub fn new(
		camera_id: CameraId,
		cmd_tx: UnboundedSender<Command>,
		status: bool,
		internal_status_rx: UnboundedReceiver<bool>,
	) -> Self {
		CameraClientHandler {
			camera_id,
			base: BasicClientHandler::new(cmd_tx, status, internal_status_rx),
		}
	}

	pub fn status(&mut self) -> bool {
		self.base.status()
	}

	/// Sends the `UpdateCameraStatus` command to the server
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	/// This function will panic if the client thread is closed.
	pub fn update_camera_status(
		&mut self,
		status: CameraStatus,
	) -> EdgeClientResult<()> {
		self.base.update_camera_status(self.camera_id, status)
	}

	/// Sends the `ImageAnalysis` command to the server
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	/// This function will panic if the client thread is closed.
	pub fn image_analysis(
		&mut self,
		deviation: Deviation,
	) -> EdgeClientResult<()> {
		self.base.image_analysis(self.camera_id, deviation)
	}

	/// Sends the `Configuration` command to the server
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	/// This function will panic if the client thread is closed.
	pub fn configuration(
		&mut self,
		conn_id: ConnId,
		configuration: String,
	) -> EdgeClientResult<()> {
		self.base
			.configuration(self.camera_id, conn_id, configuration)
	}

	/// Sends the `Alert` command to the server
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	/// This function will panic if the client thread is closed.
	pub fn alert(
		&mut self,
		alert_type: AlertType,
		alert_code: u32,
		alert_name: String,
		alert_description: String,
	) -> EdgeClientResult<()> {
		self.base
			.alert(
				Some(self.camera_id),
				alert_type,
				alert_code,
				alert_name,
				alert_description,
			)
	}

	/// Sends the `SetNameCamera` command to the server
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	/// This function will panic if the client thread is closed.
	pub fn set_name_camera(
		&mut self,
		name: String,
	) -> EdgeClientResult<()> {
		self.base.set_name_camera(self.camera_id, name)
	}
}
