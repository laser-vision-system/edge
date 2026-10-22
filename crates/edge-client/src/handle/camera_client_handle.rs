use api::shared::{
	AlertType,
	CameraId,
	CameraStatus,
	ConnId,
	Deviation,
};
use tokio::sync::mpsc::{
	UnboundedReceiver,
	UnboundedSender,
};
use tokio::sync::mpsc::error::TryRecvError;
use crate::{
	error::EdgeClientResult,
	interface::commands::Command,
};
use crate::handle::basic_client_handle::BasicClientHandle;
use crate::interface::types::CameraRx;
use crate::interface::websocket_requests::CameraRequest;

pub struct CameraClientHandle {
	camera_id: CameraId,
	request_receiver: CameraRx,
	base: BasicClientHandle,
}

/// Stripped down version of `ClientHandler`, with an internal `CameraId` and
/// access to camera-related websocket commands.
impl CameraClientHandle {
	pub fn new(
		camera_id: CameraId,
		cmd_tx: UnboundedSender<Command>,
		request_receiver: CameraRx,
		status: bool,
		internal_status_rx: UnboundedReceiver<bool>,
	) -> Self {
		CameraClientHandle {
			camera_id,
			request_receiver,
			base: BasicClientHandle::new(cmd_tx, status, internal_status_rx),
		}
	}

	/// Tries to receive the next value for this receiver.
	///
	/// # Errors
	///
	/// This method returns the `Empty` error if the channel is currently empty,
	/// but there are still outstanding senders or permits.
	///
	/// This method returns the `Disconnected` error if the channel is
	/// currently empty, and there are no outstanding senders or permits.
	pub fn try_recv(&mut self) -> Result<CameraRequest, TryRecvError>{
		self.request_receiver.try_recv()
	}

	/// Receives the next value for this receiver.
	///
	/// This method returns `None` if the channel has been closed and there are no remaining messages
	/// in the channel's buffer. This indicates that no further values can ever be received from
	/// this Receiver. The channel is closed when all senders have been dropped,
	/// or when close is called.
	///
	/// If there are no messages in the channel's buffer, but the
	/// channel has not yet been closed, this method will sleep
	/// until a message is sent or the channel is closed.
	pub async fn recv(&mut self) -> Option<CameraRequest> {
		self.request_receiver.recv().await
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
			.configuration(conn_id, self.camera_id, configuration)
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

	/// Sends an image to the server
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	/// This function will panic if the client thread is closed.
	pub fn send_image(
		&mut self,
		frame: Vec<u8>,
	) -> EdgeClientResult<()> {
		self.base.send_image(self.camera_id, frame)
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
