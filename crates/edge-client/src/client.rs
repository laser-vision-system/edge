//! Defines the behavior of the websocket thread.
mod auto_connect;
mod persistent_edge_id;

use std::collections::{
    HashMap,
};
use std::collections::hash_map::Entry;
use std::time::Duration;
use api::{
    edge::{
        Alert,
        EdgeRequests,
        ImageAnalysis,
        InitialiseConnection,
        NewCameraInfo,
        RegisterCameras,
        ServerRequests,
        SetNameCamera,
        Settings,
    },
    shared::{
        AlertType,
        CameraId,
        CameraStatus,
        Configuration,
        ConnId,
        Deviation,
        ForgetCameraRequest,
        VisionTechnique,
    },
};
use edge_core::{
    logger::logger,
};
use futures_util::{
    stream::FusedStream as _,
    SinkExt as _,
    StreamExt as _,
};
use serde::Serialize;
use tokio::{
    net::TcpStream,
    sync::mpsc::{
        UnboundedReceiver,
        UnboundedSender,
    },
    time::interval,
};
use tokio_tungstenite::{
    tungstenite::{
        Bytes,
        Error,
        Message,
        Utf8Bytes,
    },
    MaybeTlsStream,
    WebSocketStream,
};
use regex::Regex;
use crate::{
	client::{
        auto_connect::get_ws_stream,
        persistent_edge_id::get_mac_address,
    },
	error::{
        EdgeClientError,
        EdgeClientResult,
    },
	interface::{
        Command,
		CameraRequest,
		PersistenceRequest,
    },
};
use crate::handle::ClientHandle;
use crate::interface::{CameraRx, CameraTx, ListenerRx, ListenerTx, PersistenceRx, PersistenceTx, StatusRx, StatusTx, VisionRx, VisionTx};

pub const API: &str = "/edge_ws";
pub const PORT: &str = "8080";
pub const AUTO_CONNECT_DELAY: Duration = Duration::from_secs(5);
pub const WS_KEEP_ALIVE: Duration = Duration::from_secs(6);
type WebsocketType = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// The main websocket client_2 implementation.
#[derive(Debug)]
pub struct Client {
	/// Channel for receiving instructions from other threads.
	cmd_rx: UnboundedReceiver<Command>,

	/// List of transmitters for receivers listening to the websocket status.
	status_listeners: Vec<StatusTx>,

	/// Dedicated status transmitter for the conductor. If this closes,
	/// this thread will panic.
	status_tx: StatusTx,

	/// Dedicated transmitter for vision model selection. If this closes,
	/// this thread will panic.
	vision_tx: VisionTx,

	/// Dedicated transmitter for commands destined for the `Persistence` struct.
	/// If this closes, this thread will panic
	persistence_tx: PersistenceTx,

	/// Dynamic list of camera command receivers with associated camera IDs.
	cameras: HashMap<CameraId, CameraTx>,

	/// Dynamic list of arbitrary listeners. This field should be depreciated
	/// once the other transmitters are in proper use.
	listeners: Vec<ListenerTx>,

	/// The url of the currently connected server.
	url: Option<String>,
}

/// Methods for the clients overall process loop.
impl Client {
	/// Removes the protocol and endpoint from a url.
	///
	/// # Panics
	/// This function will panic if given a value not it protocol://*.*.*.*:****/edge_ws format, or if
	/// the internal regex is invalid
	fn convert_url(
		url: String,
	) -> String {
		let regex = Regex::new(r"^[A-z]+://([0-9.]+:[0-9]+)/edge_ws").expect("invalid regex");

		let captures = regex.captures(&url);

		let capture = captures
			.expect("Invalid url")
			.get(1)
			.expect("Invalid url");

		capture.as_str().to_string()
	}

	/// Loops until a websocket is successfully connected.
	async fn connect_ws(&mut self) -> (WebsocketType, String) {
		let mut interval = interval(AUTO_CONNECT_DELAY);
		loop {
			interval.tick().await;
			if let (Ok(stream), url) = get_ws_stream().await {
				return (stream, url);
			}
		}
	}

	/// Sends our MAC address to the server as a `ConnId` initializing the connection.
	///
	/// This should always be the first thing we do after connecting to a websocket.
	///
	/// # Panics
	/// If we are unable to acquire our own MAC address, this panics.
	async fn send_persistent_edge_id(
		&mut self,
		websocket: &mut WebsocketType,
	) {
		let request_enum =
			ServerRequests::InitialiseConnection(InitialiseConnection {
				edge_id: get_mac_address().expect("missing MAC address"),
			});
		self.send_text(websocket, request_enum).await.unwrap_or(());
	}

	/// Sends an arbitrary serializable struct to the server.
	///
	/// # Errors
	/// Returns a [`EdgeClientError::WebsocketClosed`] if the text does not send
	async fn send_text<T>(
		&mut self,
		websocket: &mut WebsocketType,
		data: T,
	) -> EdgeClientResult<()>
	where
		T: Serialize,
	{
		let result = websocket
			.send(Message::Text(Utf8Bytes::from(&serde_json::to_string(
				&data,
			).expect("Unable to serialize struct"))))
			.await;

		self.process_ws_err(result)
	}

	/// Converts `tungstenite::error` into an [`EdgeClientError`]
	fn process_ws_err(
		&mut self,
		result: Result<(), Error>,
	) -> EdgeClientResult<()> {
		if result.is_err() {
			return match result.err() {
				None => {
					logger().lock().unwrap().log_warn(
						"Could not send text through websocket for unknown reason",
					);
					Ok(())
				},
				Some(err) => {
					match err {
						Error::ConnectionClosed | Error::AlreadyClosed => {
							Err(EdgeClientError::WebsocketClosed)
						},

						err => {
							logger().lock().unwrap().log_warn(
								format!(
									"Could not send text through websocket due to: {err}"
								)
								.as_str(),
							);
							Ok(())
						},
					}
				},
			};
		}
		Ok(())
	}

	/// Forwards the SetProcessingType request to `vision_rx`
	fn set_processing_type(
		&mut self,
		vision: VisionTechnique,
	) {
		self.vision_tx.send(vision).expect("Vision channel closed");
	}

	/// Forwards non-vision-processing requests to their respective camera,
	/// logs an error if the request uses an invalid camera ID.
	fn camera_request(
		&mut self,
		request: CameraRequest,
		camera: CameraId,
	) {
		let result = self
			.cameras
			.entry(camera)
			.and_modify(|entry| entry.send(request).unwrap_or(()) );

		match result {
			Entry::Occupied(_) => {}
			Entry::Vacant(_) => {
				logger().lock().unwrap().log_warn(&format!(
					"Command received for invalid camera ID: {:?} ",
					camera
				));
			}
		}
	}

	/// Forwards requests to the `persistence` struct.
	fn persistence_request(
		&mut self,
		request: PersistenceRequest,
	) {
		self.persistence_tx
			.send(request)
			.expect("Persistence channel closed");
	}

	/// Forwards all requests to miscellaneous listeners. In place for migration
	/// purposes.
	fn forward_request(
		&mut self,
		request: EdgeRequests,
	) {
		self.listeners
			.iter()
			.for_each(|listener|
				listener.send(request.clone()).unwrap_or(())
			);
	}


	/// Forwards the websockets status to relevant listeners.
	fn status_update(
		&mut self,
		status: bool,
	) {
		self.status_tx.send(status).expect(
			"Status receiver dropped before websocket thread was closed",
		);
		self.status_listeners
			.iter()
			.for_each(|listener|
				listener.send(status).unwrap_or(())
			);
	}

	/// Removes any closed listeners from storage.
	fn check_listeners(&mut self) {
		self.listeners.retain(|listener| !listener.is_closed());
		self.cameras.retain(|_, listener| !listener.is_closed());
		self.status_listeners
			.retain(|listener| !listener.is_closed());
	}

	/// Listens for incoming commands and websocket requests.
	///
	/// Every interval, it checks if the websocket is closed,
	/// sends a keepalive, and checks the status of listeners from other threads
	///
	/// Exits when the websocket closes
	async fn main_loop(
		&mut self,
		websocket: &mut WebsocketType,
	) {
		let mut interval = interval(WS_KEEP_ALIVE);
		loop {
			tokio::select! {
				Some(cmd)  = self.cmd_rx.recv() => {
					if let Err(e) = self.process_command(cmd, websocket).await {

					}
				}

				Some(Ok(msg)) = websocket.next() => {
					if let Err(e) = self.process_ws(msg).await {

					}
				}

				_ = interval.tick() => {
					if websocket.is_terminated() {
						break
					}
					websocket.send(Message::Ping(Bytes::from("ping"))).await.unwrap_or(());
					self.check_listeners();
				}
			}
		}
		websocket.send(Message::Close(None)).await.unwrap_or(());
	}

	/// Processes Commands send by the client_2 handle.
	///
	/// # Panics
	/// This will panic if the return channels close before it returns,
	/// which should not happen unless the main thread panics.
	async fn process_command(
		&mut self,
		command: Command,
		websocket: &mut WebsocketType,
	) -> EdgeClientResult<()> {
		match command {
			Command::SubscribeNewCamera {
				camera_id,
				res_tx,
			} => {
				let result = self.subscribe_new_camera(camera_id);
				res_tx
					.send(result)
					.expect("Return channel closed before returning");
			},

			Command::SubscribeNewListener {
				res_tx,
			} => {
				let result = self.subscribe_listener();
				res_tx
					.send(result)
					.expect("Return channel closed before returning");
			},

			Command::GetNewStatusChannel {
				res_tx,
			} => {
				let result = self.get_new_status_channel();
				res_tx
					.send(result)
					.expect("Return channel closed before returning");
			},

			Command::RegisterCameras {
				cameras,
			} => {
				self.register_cameras(websocket, cameras).await?;
			},

			Command::UpdateCameraStatus {
				camera_id,
				status,
			} => {
				self.update_camera_status(websocket, camera_id, status)
					.await?;
			},

			Command::ImageAnalysis {
				camera_id,
				deviation,
			} => {
				self.image_analysis(websocket, camera_id, deviation).await?;
			},

			Command::Configuration {
				conn_id,
				camera_id,
				configuration,
			} => {
				self.configuration(
					websocket,
					conn_id,
					camera_id,
					configuration,
				)
				.await?;
			},

			Command::ForgetCameraRequest {
				conn_id,
				camera_id,
				confirm,
			} => {
				self.forget_camera_request(
					websocket, conn_id, camera_id, confirm,
				)
				.await?;
			},

			Command::Alert {
				camera_id,
				alert_code,
				alert_type,
				alert_name,
				alert_description,
			} => {
				self.alert(
					websocket,
					camera_id,
					alert_type,
					alert_code,
					alert_name,
					alert_description,
				)
				.await?;
			},

			Command::SetNameCamera {
				camera_id,
				name,
			} => {
				self.set_name_camera(websocket, camera_id, name).await?;
			},

			Command::SendImage {
				camera_id,
				frame,
			} => self.send_image(websocket, camera_id, &mut frame.clone()).await?,

			Command::GetDefaultSettings {
				res_tx,
			} => {
				let result = self.get_default_settings().await;
				res_tx
					.send(result)
					.expect("Return channel closed before returning");
			},

			Command::GetServerURL {
				res_tx,
			} => {
				let result = self.get_server_url();
				res_tx
					.send(result)
					.expect("Return channel closed before returning");
			},
		}
		Ok(())
	}

	/// Processes a websocket message, returning an error on close.
	async fn process_ws(
		&mut self,
		msg: Message,
	) -> EdgeClientResult<()> {
		match msg {
			Message::Text(text) => {
				self.process_message(&text).await;
			},

			Message::Close(_) => return Err(EdgeClientError::WebsocketClosed),

			_ => {},
		}
		Ok(())
	}

	/// Parses websocket requests and calls their respective functions.
	async fn process_message(
		&mut self,
		text: &str,
	) {
		let msg = text.trim();

		let data_unchecked: serde_json::error::Result<EdgeRequests> =
			serde_json::from_str(msg);

		let Ok(request) = data_unchecked else {
			logger().lock().unwrap().log_warn(&format!(
				"Could not deserialize websocket message due to: {:?}",
				data_unchecked.err().unwrap().to_string()
			));
			return
		};

		self.forward_request(request.clone());

		match request {
			EdgeRequests::SetNameCamera(data) => {
				let request = CameraRequest::SetNameCamera(data.name);
				self.camera_request(request, data.camera_id);
			},

			EdgeRequests::ToggleViewingCamera(data) => {
				let request =
					CameraRequest::ToggleViewingCamera(data.start_viewing);
				self.camera_request(request, data.camera_id);
			},

			EdgeRequests::Configuration(data) => {
				let request = CameraRequest::Configuration(data.configuration);
				self.camera_request(request, data.camera_id);
			},

			EdgeRequests::ViewCameraConfig(data) => {
				let request = CameraRequest::ViewCameraConfig(data.conn_id);
				self.camera_request(request, data.camera_id);
			},

			EdgeRequests::Setpoint(data) => {
				let request = CameraRequest::Setpoint;
				self.camera_request(request, data.camera_id);
			},

			EdgeRequests::SetProcessingType(data) => {
				self.set_processing_type(data.processing_type);
			},

			EdgeRequests::ForceRetryCamera(data) => {
				let request =
					PersistenceRequest::ForceRetryCamera(data.camera_id);
				self.persistence_request(request);
			},
			EdgeRequests::ForgetCameraRequest(data) => {
				let request = PersistenceRequest::ForgetCameraRequest(
					data.conn_id,
					data.camera_id,
				);
				self.persistence_request(request);
			},

			_ => {},
		}
	}
}

/// Methods for handling commands from the handle.
impl Client {
	/// Adds a new camera receiver to the internal camera list, and returns the
	/// associated receiver
	fn subscribe_new_camera(
		&mut self,
		camera_id: CameraId,
	) -> CameraRx {
		let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
		self.cameras.insert(camera_id, tx);
		rx
	}

	/// Adds a new listener to the internal list, returning the
	/// associated receiver.
	///
	/// TO BE DEPRECIATED
	fn subscribe_listener(
		&mut self
	) -> ListenerRx {
		let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
		self.listeners.insert(0, tx);
		rx
	}

	/// Creates a new status channel, returning the
	/// associated receiver.
	fn get_new_status_channel(
		&mut self
	) -> StatusRx {
		let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
		self.status_listeners.insert(0, tx);
		rx
	}

	/// Sends the `RegisterCameras` command to the server
	async fn register_cameras(
		&mut self,
		websocket: &mut WebsocketType,
		cameras: HashMap<CameraId, NewCameraInfo>,
	) -> EdgeClientResult<()> {
		let message = ServerRequests::RegisterCameras(RegisterCameras {
			cameras,
		});
		self.send_text(websocket, message).await
	}

	/// Sends the `UpdateCameraStatus` command to the server
	async fn update_camera_status(
		&mut self,
		websocket: &mut WebsocketType,
		camera_id: CameraId,
		status: CameraStatus,
	) -> EdgeClientResult<()> {
		let message =
			ServerRequests::UpdateCameraStatus(api::edge::UpdateCameraStatus {
				camera_id,
				status,
			});
		self.send_text(websocket, message).await
	}

	/// Sends the `ImageAnalysis` command to the server
	async fn image_analysis(
		&mut self,
		websocket: &mut WebsocketType,
		camera_id: CameraId,
		deviation: Deviation,
	) -> EdgeClientResult<()> {
		let message = ServerRequests::ImageAnalysis(ImageAnalysis {
			camera_id,
			deviation,
		});
		self.send_text(websocket, message).await
	}

	/// Sends the `Configuration` command to the server
	async fn configuration(
		&mut self,
		websocket: &mut WebsocketType,
		conn_id: ConnId,
		camera_id: CameraId,
		configuration: String,
	) -> EdgeClientResult<()> {
		let message = ServerRequests::Configuration(Configuration {
			conn_id,
			camera_id,
			configuration,
		});
		self.send_text(websocket, message).await
	}

	/// Sends the `ForgetCameraRequest` command to the server
	async fn forget_camera_request(
		&mut self,
		websocket: &mut WebsocketType,
		conn_id: ConnId,
		camera_id: CameraId,
		confirm: bool,
	) -> EdgeClientResult<()> {
		let message =
			ServerRequests::ForgetCameraRequest(ForgetCameraRequest {
				conn_id,
				camera_id,
				confirm,
			});
		self.send_text(websocket, message).await
	}

	/// Sends the `Alert` command to the server
	async fn alert(
		&mut self,
		websocket: &mut WebsocketType,
		camera_id: Option<CameraId>,
		alert_type: AlertType,
		alert_code: u32,
		alert_name: String,
		alert_description: String,
	) -> EdgeClientResult<()> {
		let message = ServerRequests::Alert(Alert {
			camera_id,
			alert_type,
			alert_code,
			alert_name,
			alert_description,
		});
		self.send_text(websocket, message).await
	}

	/// Sends the `SetNameCamera` command to the server
	async fn set_name_camera(
		&mut self,
		websocket: &mut WebsocketType,
		camera_id: CameraId,
		name: String,
	) -> EdgeClientResult<()> {
		let message = ServerRequests::SetNameCamera(SetNameCamera {
			camera_id,
			name,
		});
		self.send_text(websocket, message).await
	}

	/// Sends the `SendImage` command to the server
	///
	/// # Errors
	/// Returns an error if the websocket was closed
	async fn send_image(
		&mut self,
		websocket: &mut WebsocketType,
		camera_id: u64,
		frame: &mut Vec<u8>,
	) -> EdgeClientResult<()> {
		let mut msg = camera_id.to_be_bytes().to_vec();
		msg.append(frame);
		let result = websocket.send(Message::Binary(Bytes::from(msg))).await;
		self.process_ws_err(result)
	}

	/// Gets the default settings from the server.
	async fn get_default_settings(&mut self) -> EdgeClientResult<Settings> {
		auto_connect::get_default_settings(
			self
				.url
				.clone()
				.expect("websocket URL not set while websocket connected")
				.as_str(),
		)
		.await
	}

	/// Returns the stored server url.
	fn get_server_url(&mut self) -> String {
		self
			.url
			.clone()
			.expect("websocket URL not set while websocket connected")
	}
}

impl Client {
	pub async fn new() -> (
		StatusRx,
		VisionRx,
		PersistenceRx,
		ClientHandle,
		Self
	) {
		let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel();
		let (status_tx, status_rx) = tokio::sync::mpsc::unbounded_channel();
		let (internal_status_tx, internal_status_rx) = tokio::sync::mpsc::unbounded_channel();
		let (vision_tx, vision_rx) = tokio::sync::mpsc::unbounded_channel();
		let (persistence_tx, persistence_rx) = tokio::sync::mpsc::unbounded_channel();
		let mut status_listeners = Vec::new();
		status_listeners.insert(0, internal_status_tx);
		(
			status_rx,
			vision_rx,
			persistence_rx,
			ClientHandle::new(
				cmd_tx,
				internal_status_rx,
			),
			Client {
				cmd_rx,
				status_listeners,
				status_tx,
				vision_tx,
				persistence_tx,
				cameras: HashMap::new(),
				listeners: Vec::new(),
				url: None,
			},
		)
	}


	pub async fn run(mut self) -> EdgeClientResult<()> {
		loop {
			let (mut websocket, url) = self.connect_ws().await;

			self.send_persistent_edge_id(&mut websocket).await;

			self.url = Some(Self::convert_url(url)); // Saving the url here so eventually we can implement a get_default_settings method

			self.status_update(true);

			logger().lock().unwrap().log_info("Websocket connected");

			self.main_loop(&mut websocket).await;

			logger().lock().unwrap().log_warn("Websocket disconnected");

			self.status_update(false);
		}
		Ok(())
	}
}
