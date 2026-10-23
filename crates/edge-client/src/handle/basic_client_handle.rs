//! Base client handle struct, defining methods for all other client handle
//! structs to use.
use tokio::sync::mpsc::{
	UnboundedReceiver,
	UnboundedSender,
	error::SendError,
};

use crate::{
	error::{
		EdgeClientError,
		EdgeClientResult,
	},
	interface::Command,
};

/// Base client handle, providing common methods and data for other client
/// handlers to use.
#[derive(Debug)]
pub struct BasicClientHandle {
	/// Transmission channel to the client thread.
	cmd_tx: UnboundedSender<Command>,

	/// Last known status of the websocket.
	status: bool,

	/// Channel for receiving status updates
	internal_status_rx: UnboundedReceiver<bool>,
}

// TODO: Move the camera request methods out of this class into the
// CameraClientHandle class.
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
	pub fn send(
		&mut self,
		cmd: Command,
	) -> Result<(), SendError<Command>> {
		self.cmd_tx.send(cmd)
	}

	/// Creates a clone of the sender TX.
	pub fn get_sender(&mut self) -> UnboundedSender<Command> {
		self.cmd_tx.clone()
	}

	/// Sends a command to the client, on the condition that the websocket
	/// connection is up
	///
	/// # Errors:
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics:
	/// If the client thread is closed, panics.
	pub fn send_ws_request(
		&mut self,
		cmd: Command,
	) -> EdgeClientResult<()> {
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
		self.status = self
			.internal_status_rx
			.recv()
			.await
			.expect("status change channel closed");
		self.status
	}
}
