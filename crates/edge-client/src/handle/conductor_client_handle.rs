//! A client handle provided to the conductor.

// TODO: Remove expects and then remove this.
#![expect(clippy::expect_used, reason = "WIP development")]
#![expect(deprecated, reason = "Temporarily used for backwards compatibility")]

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
use tokio::sync::{
	mpsc::UnboundedSender,
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
		ListenerRx,
		StatusRx,
	},
};

/// Interface for other threads to access the websocket client.
///
/// Contains all functions necessary to communicate with the websocket,
/// and to get receivers for messages from the websocket thread.
#[derive(Debug)]
pub struct ConductorClientHandle {
	base: BasicClientHandle,
}

// TODO: Remove all the camera command methods here, once the proper integration
// has been done.
impl ConductorClientHandle {
	/// Create a new [`ConductorClientHandle`] instance.
	///
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
	) -> Self {
		ConductorClientHandle {
			base: BasicClientHandle::new(cmd_tx, false, internal_status_rx),
		}
	}

	/// Gets the event receiver for a new camera.
	///
	/// These receivers can be dropped without issue.
	///
	/// # Panics
	///
	/// This function will panic if the client thread is closed, or closes while
	/// waiting to respond.
	///
	/// # Depreciating
	///
	/// This is replaced by `subscribe_new_camera` on `PersistenceClientHandle`.
	#[deprecated]
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

	/// Gets a generic listener for handling any websocket request.
	///
	/// Will be phased out as the other receivers are put into place.
	///
	/// These receivers can be dropped without issue.
	///
	/// # Panics
	///
	/// This function will panic if the client thread is closed, or closes while
	/// waiting to respond.
	///
	/// # Depreciating
	///
	/// This function should not be used.
	#[deprecated]
	pub async fn subscribe_new_listener(&mut self) -> ListenerRx {
		let (res_tx, res_rx) = oneshot::channel();

		self.base
			.send(Command::SubscribeNewListener {
				res_tx,
			})
			.expect("Client thread closed");

		res_rx.await.expect("Return handle dropped")
	}

	/// Sleeps until the websocket is up.
	pub async fn assert_connection(&mut self) {
		if self.status() {
			return;
		}
		while !self.base.await_status_change().await {}
	}

	/// Sends the `RegisterCameras` command to the server.
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	///
	/// This function will panic if the client thread is closed.
	///
	/// # Depreciating
	///
	/// This is replaced by `register_cameras` on `PersistenceClientHandle`.
	#[deprecated]
	pub fn register_cameras(
		&mut self,
		cameras: HashMap<CameraId, NewCameraInfo>,
	) -> EdgeClientResult<()> {
		self.base.send_ws_request(Command::RegisterCameras {
			cameras,
		})
	}

	/// Gets the current status of the websocket.
	pub fn status(&mut self) -> bool {
		self.base.status()
	}

	/// Sends the `UpdateCameraStatus` command to the server.
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	///
	/// This function will panic if the client thread is closed.
	///
	/// # Depreciating
	///
	/// This is replaced by `update_camera_status` on `CameraClientHandle`.
	#[deprecated]
	pub fn update_camera_status(
		&mut self,
		camera_id: CameraId,
		status: CameraStatus,
	) -> EdgeClientResult<()> {
		self.base.send_ws_request(Command::UpdateCameraStatus {
			camera_id,
			status,
		})
	}

	/// Sends the `ImageAnalysis` command to the server.
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	///
	/// This function will panic if the client thread is closed.
	///
	/// # Depreciating
	///
	/// This is replaced by `image_analysis` on `CameraClientHandle`.
	#[deprecated]
	pub fn image_analysis(
		&mut self,
		camera_id: CameraId,
		deviation: Deviation,
	) -> EdgeClientResult<()> {
		self.base.send_ws_request(Command::ImageAnalysis {
			camera_id,
			deviation,
		})
	}

	/// Sends the `Configuration` command to the server.
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	///
	/// This function will panic if the client thread is closed.
	///
	/// # Depreciating
	///
	/// This is replaced by configuration on `CameraClientHandle`.
	#[deprecated]
	pub fn configuration(
		&mut self,
		conn_id: ConnId,
		camera_id: CameraId,
		configuration: String,
	) -> EdgeClientResult<()> {
		self.base.send_ws_request(Command::Configuration {
			conn_id,
			camera_id,
			configuration,
		})
	}

	/// Sends the `Alert` command to the server.
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	///
	/// This function will panic if the client thread is closed.
	pub fn alert(
		&mut self,
		camera_id: Option<CameraId>,
		alert_type: AlertType,
		alert_code: u32,
		alert_name: String,
		alert_description: String,
	) -> EdgeClientResult<()> {
		self.base.send_ws_request(Command::Alert {
			camera_id,
			alert_type,
			alert_code,
			alert_name,
			alert_description,
		})
	}

	/// Sends an image to the server.
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	///
	/// This function will panic if the client thread is closed.
	///
	/// # Depreciating
	///
	/// This is replaced by `send_image` on `CameraClientHandle`.
	#[deprecated]
	pub fn send_image(
		&mut self,
		camera_id: CameraId,
		frame: Vec<u8>,
	) -> EdgeClientResult<()> {
		self.base.send_ws_request(Command::SendImage {
			camera_id,
			frame,
		})
	}

	/// Sends the `SetNameCamera` command to the server.
	///
	/// # Errors
	///
	/// Returns a `WebsocketDown` error if the status is false.
	///
	/// # Panics
	///
	/// This function will panic if the client thread is closed.
	///
	/// # Depreciating
	///
	/// This is replaced by `set_name_camera` on [`CameraClientHandle`].
	#[deprecated]
	pub fn set_name_camera(
		&mut self,
		camera_id: CameraId,
		name: String,
	) -> EdgeClientResult<()> {
		self.base.send_ws_request(Command::SetNameCamera {
			camera_id,
			name,
		})
	}

	/// Gets the default settings from the server.
	///
	/// # Errors
	///
	/// No clue, sorry.
	///
	/// # Panics
	///
	/// This function will panic if the client thread is closed, or closes while
	/// waiting to respond.
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
	///
	/// This function will panic if the client thread is closed, or closes while
	/// waiting to respond.
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
