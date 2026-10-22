//! Base client handle struct, defining methods for all other client handle structs to use.
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
use tokio::sync::mpsc::error::SendError;
use crate::{
    error::{
        EdgeClientError,
        EdgeClientResult,
    },
    interface::Command,
};

/// Base client handle, providing common methods and data for other client
/// handlers to use.
pub struct BasicClientHandle {
	/// Transmission channel to the client thread.
	cmd_tx: UnboundedSender<Command>,

	/// Last known status of the websocket.
	status: bool,

	/// Channel for receiving status updates
	internal_status_rx: UnboundedReceiver<bool>,
}

// TODO: Move the camera request methods out of this class into the CameraClientHandle class.
impl BasicClientHandle {
	pub fn new(
		cmd_tx: UnboundedSender<Command>,
		status: bool,
		internal_status_rx: UnboundedReceiver<bool>,
	) -> Self {
		BasicClientHandle {
			cmd_tx,
			status,
			internal_status_rx,
		}
	}

	/// Sends an arbitrary command to the client.
	pub fn send(&mut self, cmd: Command) -> Result<(), SendError<Command>> {
		self.cmd_tx.send(cmd)
	}

	/// Creates a clone of the sender TX.
	pub fn get_sender(&mut self) -> UnboundedSender<Command> {
		self.cmd_tx.clone()
	}

	/// Sends a command to the client, on the condition that the websocket connection is up
	///
	/// # Errors:
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics:
	/// If the client thread is closed, panics.
	pub fn send_ws_request(&mut self, cmd: Command) -> EdgeClientResult<()> {
		self.update_status()?;

		self.cmd_tx.send(cmd).expect("Client thread closed");

		Ok(())
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


	/// Sleeps until the status changes
	pub async fn await_status_change(&mut self) -> bool {
		self.status = self.internal_status_rx.recv().await.expect("status change channel closed");
		self.status
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
		self.send_ws_request(Command::UpdateCameraStatus {
				camera_id,
				status,
			})
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
		self.send_ws_request(Command::ImageAnalysis {
				camera_id,
				deviation,
			})
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
		self.send_ws_request(Command::Configuration {
				conn_id,
				camera_id,
				configuration,
			})
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
		self.send_ws_request(Command::ForgetCameraRequest {
				conn_id,
				camera_id,
				confirm,
			})
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
		self.send_ws_request(Command::Alert {
				camera_id,
				alert_type,
				alert_code,
				alert_name,
				alert_description,
			})
	}

	pub fn send_image(&mut self, camera_id: CameraId, frame: Vec<u8>) -> EdgeClientResult<()> {
		self.send_ws_request( Command::SendImage {
			camera_id,
			frame,
		})
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
		self.send_ws_request(Command::SetNameCamera {
				camera_id,
				name,
			})
	}
}
