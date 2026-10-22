use std::collections::HashMap;
use api::{
	edge::{
		NewCameraInfo,
		Settings,
	},
	shared::{
		AlertType,
		CameraId,
		CameraStatus,
		ConnId,
		Deviation,
	},
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use crate::{
	error::EdgeClientResult,
	handle::camera_client_handle::CameraClientHandle,
	interface::commands::Command,
};
use crate::handle::basic_client_handle::BasicClientHandle;
use crate::interface::types::{ListenerRx, PersistenceRx, StatusRx, VisionRx};

/// Interface for other threads to access the websocket client.
///
/// Contains all functions necessary to communicate with the websocket,
/// and to get receivers for messages from the websocket thread.
pub struct ClientHandle {
	base: BasicClientHandle,
}


// TODO: Remove all the camera command methods here, once the proper integration has been done.
impl ClientHandle {
	/// Starts a websocket client on a separate thread.
	/// Returns itself, for communications with the thread,
	/// as well as a receiver which sends true when the websocket comes online,
	/// a receiver to indicate vision technique change requests,
	/// and a receiver for persistence specific requests.
	///
	/// The client expects these receivers to never be dropped,
	/// and will panic upon sending data to them if they are.
	pub fn new(
		cmd_tx: UnboundedSender<Command>,
		internal_status_rx: StatusRx,
	) -> Self
	{
		ClientHandle {
			base: BasicClientHandle::new(
				cmd_tx,
				false,
				internal_status_rx,
			),
		}
	}

	/// Gets the event receiver for a new camera.
	///
	/// These receivers can be dropped without issue.
	///
	/// # Panics
	/// This function will panic if the client thread is closed, or closes while waiting to respond
	pub async fn subscribe_new_camera(
		&mut self,
		camera_id: CameraId,
	) -> CameraClientHandle
	{
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
			status_receiver
		)
	}

	/// Gets a generic listener for handling any websocket request.
	///
	/// Will be phased out as the other receivers are put into place.
	///
	/// These receivers can be dropped without issue.
	///
	/// # Panics
	/// This function will panic if the client thread is closed, or closes while waiting to respond
	pub async fn subscribe_new_listener(
		&mut self
	) -> ListenerRx {
		let (res_tx, res_rx) = oneshot::channel();

		self.base
			.send(Command::SubscribeNewListener {
				res_tx,
			})
			.expect("Client thread closed");

		res_rx.await.expect("Return handle dropped")
	}

	/// Sleeps until the websocket is up
	pub async fn assert_connection(&mut self) {
		if self.status() {
			return;
		}
		while !self.base.await_status_change().await {

		}
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
		self.base
			.send_ws_request(Command::RegisterCameras {
				cameras,
			})
	}

	/// Gets the current status of the websocket.
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
		camera_id: CameraId,
		status: CameraStatus,
	) -> EdgeClientResult<()> {
		self.base.update_camera_status(camera_id, status)
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
		camera_id: CameraId,
		deviation: Deviation,
	) -> EdgeClientResult<()> {
		self.base.image_analysis(camera_id, deviation)
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
		camera_id: CameraId,
		configuration: String,
	) -> EdgeClientResult<()> {
		self.base
			.configuration(conn_id, camera_id, configuration)
	}

	/// Sends the `ConfirmForgetCamera` command to the server
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	/// This function will panic if the client thread is closed.
	pub fn forget_camera(
		&mut self,
		conn_id: ConnId,
		camera_id: CameraId,
		confirm: bool,
	) -> EdgeClientResult<()> {
		self.base
			.confirm_forget_camera(conn_id, camera_id, confirm)
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
		camera_id: Option<CameraId>,
		alert_type: AlertType,
		alert_code: u32,
		alert_name: String,
		alert_description: String,
	) -> EdgeClientResult<()> {
		self.base.alert(
			camera_id,
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
		camera_id: CameraId,
		image: Vec<u8>,
	) -> EdgeClientResult<()>  {
		self.base.send_image(camera_id, image)
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
		camera_id: CameraId,
		name: String,
	) -> EdgeClientResult<()> {
		self.base.set_name_camera(camera_id, name)
	}

	/// Gets the default settings from the server.
	///
	/// # Panics
	/// This function will panic if the client thread is closed, or closes while waiting to respond
    pub async fn get_default_setting(&mut self) -> EdgeClientResult<Settings> {
        let (res_tx, res_rx) = oneshot::channel();

        self.base
            .send(Command::GetDefaultSettings {
                res_tx,
            })
            .expect("Client thread closed");

        res_rx.await.expect("Return handle dropped")
    }

	/// Gets the url currently being used for the websocket.
	///
	/// # Panics
	/// This function will panic if the client thread is closed, or closes while waiting to respond
    pub async fn get_server_url(&mut self) -> String {
        let (res_tx, res_rx) = oneshot::channel();

        self.base
            .send(Command::GetServerURL {
                res_tx,
            })
            .expect("Client thread closed");

        res_rx.await.expect("Return handle dropped")
    }
}
