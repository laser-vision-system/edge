use std::collections::HashMap;

use api::{
	edge::NewCameraInfo,
	shared::CameraId,
};
use tokio::sync::{
	mpsc::{
		UnboundedReceiver,
		UnboundedSender,
		error::TryRecvError,
	},
	oneshot,
};

use crate::{
	error::EdgeClientResult,
	handle::{
		basic_client_handle::BasicClientHandle,
		camera_client_handle::CameraClientHandle,
	},
	interface::{
		Command,
		PersistenceRequest,
		PersistenceRx,
	},
};

#[derive(Debug)]
pub struct PersistenceClientHandle {
	request_receiver: PersistenceRx,
	base: BasicClientHandle,
}

impl PersistenceClientHandle {
	pub fn new(
		cmd_tx: UnboundedSender<Command>,
		request_receiver: PersistenceRx,
		status: bool,
		internal_status_rx: UnboundedReceiver<bool>,
	) -> Self {
		PersistenceClientHandle {
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
	pub fn try_recv(&mut self) -> Result<PersistenceRequest, TryRecvError> {
		self.request_receiver.try_recv()
	}

	/// Receives the next value for this receiver.
	///
	/// This method returns `None` if the channel has been closed and there are
	/// no remaining messages in the channel's buffer. This indicates that
	/// no further values can ever be received from this Receiver. The
	/// channel is closed when all senders have been dropped, or when close
	/// is called.
	///
	/// If there are no messages in the channel's buffer, but the
	/// channel has not yet been closed, this method will sleep
	/// until a message is sent or the channel is closed.
	pub async fn recv(&mut self) -> Option<PersistenceRequest> {
		self.request_receiver.recv().await
	}

	pub fn status(&mut self) -> bool {
		self.base.status()
	}

	/// Gets the event receiver for a new camera.
	///
	/// These receivers can be dropped without issue.
	///
	/// # Panics
	/// This function will panic if the client thread is closed, or closes while
	/// waiting to respond
	pub async fn subscribe_new_camera(
		&mut self,
		camera_id: CameraId,
	) -> CameraClientHandle {
		let (res_receiver_tx, res_receiver_rx) = oneshot::channel();
		let (res_status_tx, res_status_rx) = oneshot::channel();

		self.base
			.send(Command::SubscribeNewCamera {
				camera_id,
				res_tx: res_receiver_tx,
			})
			.expect("Client thread closed");

		self.base
			.send(Command::GetNewStatusChannel {
				res_tx: res_status_tx,
			})
			.expect("Client thread closed");

		let receiver = res_receiver_rx.await.expect("Return handle dropped");
		let status_receiver =
			res_status_rx.await.expect("Return handle dropped");

		CameraClientHandle::new(
			camera_id,
			self.base.get_sender(),
			receiver,
			self.base.status(),
			status_receiver,
		)
	}

	/// Sends the `RegisterCameras` command to the server
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	/// This function will panic if the client thread is closed.
	pub fn register_cameras(
		&mut self,
		cameras: HashMap<CameraId, NewCameraInfo>,
	) -> EdgeClientResult<()> {
		self.base.send_ws_request(Command::RegisterCameras {
			cameras,
		})
	}
}
