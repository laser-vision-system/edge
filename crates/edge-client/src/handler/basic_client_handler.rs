//! Base client handler struct, defining methods for all other client handler structs to use.
use api::{
    shared::{
        AlertType,
        CameraId,
        CameraStatus,
        ConnId,
        Deviation,
    },
};
use tokio::sync::{
    mpsc::{
        UnboundedReceiver,
        UnboundedSender,
    },
};

use crate::{
    error::{
        EdgeClientError,
        EdgeClientResult,
    },
    interface::commands::Command,
};

/// Base client handler, providing common methods and data for other client
/// handlers to use.
pub struct BasicClientHandler {
	/// Transmission channel to the client thread.
	pub cmd_tx: UnboundedSender<Command>,

	/// Last known status of the websocket.
	status: bool,

	/// Channel for receiving status updates
	internal_status_rx: UnboundedReceiver<bool>,
}

impl BasicClientHandler {
	pub fn new(
		cmd_tx: UnboundedSender<Command>,
		status: bool,
		internal_status_rx: UnboundedReceiver<bool>,
	) -> Self {
		BasicClientHandler {
			cmd_tx,
			status,
			internal_status_rx,
		}
	}

	/// Updates the status, and returns it.
	pub fn status(&mut self) -> bool {
		let _ = self.update_status();

		self.status
	}

	/// Updates 'self.status' to the current state of the websocket.
	///
	/// # Errors:
	/// Returns a `WebsocketDown` error if the status is false.
	pub fn update_status(&mut self) -> EdgeClientResult<()> {
		while let Ok(status) = self.internal_status_rx.try_recv() {
			self.status = status;
		}

		if self.status {
			return Ok(());
		}
		Err(EdgeClientError::WebsocketDown)
	}

	/// Sends the `UpdateCameraStatus` command to the server.
	///
	/// # Errors:
	/// Returns a `WebsocketDown` error if the websocket is down.
	pub fn update_camera_status(
		&mut self,
		camera_id: CameraId,
		status: CameraStatus,
	) -> EdgeClientResult<()> {
		self.update_status()?;

		self.cmd_tx
			.send(Command::UpdateCameraStatus {
				camera_id,
				status,
			})
			.expect("Client thread closed");

		Ok(())
	}

	/// Sends the `ImageAnalysis` command to the server.
	///
	/// # Errors:
	/// Returns a `WebsocketDown` error if the websocket is down.
	pub fn image_analysis(
		&mut self,
		camera_id: CameraId,
		deviation: Deviation,
	) -> EdgeClientResult<()> {
		self.update_status()?;

		self.cmd_tx
			.send(Command::ImageAnalysis {
				camera_id,
				deviation,
			})
			.expect("Client thread closed");

		Ok(())
	}

	/// Sends the `Configuration` command to the server.
	///
	/// # Errors:
	/// Returns a `WebsocketDown` error if the websocket is down.
	pub fn configuration(
		&mut self,
		conn_id: ConnId,
		camera_id: CameraId,
		configuration: String,
	) -> EdgeClientResult<()> {
		self.update_status()?;

		self.cmd_tx
			.send(Command::Configuration {
				conn_id,
				camera_id,
				configuration,
			})
			.expect("Client thread closed");

		Ok(())
	}

	/// Sends the `ConfirmForgetCamera` command to the server.
	///
	/// # Errors:
	/// Returns a `WebsocketDown` error if the websocket is down.
	pub fn confirm_forget_camera(
		&mut self,
		conn_id: ConnId,
		camera_id: CameraId,
		confirm: bool,
	) -> EdgeClientResult<()> {
		self.update_status()?;

		self.cmd_tx
			.send(Command::ForgetCameraRequest {
				conn_id,
				camera_id,
				confirm,
			})
			.expect("Client thread closed");

		Ok(())
	}

	/// Sends the `Alert` command to the server.
	///
	/// # Errors:
	/// Returns a `WebsocketDown` error if the websocket is down.
	pub fn alert(
		&mut self,
		camera_id: Option<CameraId>,
		alert_type: AlertType,
		alert_code: u32,
		alert_name: String,
		alert_description: String,
	) -> EdgeClientResult<()> {
		self.update_status()?;

		self.cmd_tx
			.send(Command::Alert {
				camera_id,
				alert_type,
				alert_code,
				alert_name,
				alert_description,
			})
			.expect("Client thread closed");

		Ok(())
	}

	/// Sends the `SetNameCamera` command to the server.
	///
	/// # Errors:
	/// Returns a `WebsocketDown` error if the websocket is down.
	pub fn set_name_camera(
		&mut self,
		camera_id: CameraId,
		name: String,
	) -> EdgeClientResult<()> {
		self.update_status()?;

		self.cmd_tx
			.send(Command::SetNameCamera {
				camera_id,
				name,
			})
			.expect("Client thread closed");

		Ok(())
	}


}
