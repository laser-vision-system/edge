use std::collections::HashMap;

use api::{
	edge::{
		EdgeRequests,
		NewCameraInfo,
		Settings,
	},
	shared::{
		AlertType,
		CameraId,
		CameraStatus,
		ConnId,
		Deviation,
		VisionTechnique,
	},
};
use tokio::sync::{
	mpsc::{
		UnboundedReceiver,
		UnboundedSender,
	},
	oneshot,
};

use crate::error::EdgeClientResult;

/// Receiver for the status of the websocket, with true
/// indicating the websocket has come up, and false indicating
/// the websocket has gone down. Any iteration through the
/// `.next()` method will be in chronological order.
pub type StatusRx = UnboundedReceiver<bool>;

pub type StatusTx = UnboundedSender<bool>;

pub type VisionRx = UnboundedReceiver<VisionTechnique>;
pub type VisionTx = UnboundedSender<VisionTechnique>;

#[deprecated]
pub type ListenerRx = UnboundedReceiver<EdgeRequests>;
#[deprecated]
pub type ListenerTx = UnboundedSender<EdgeRequests>;

pub type PersistenceRx = UnboundedReceiver<PersistenceRequest>;
pub type PersistenceTx = UnboundedSender<PersistenceRequest>;

pub type CameraRx = UnboundedReceiver<CameraRequest>;
pub type CameraTx = UnboundedSender<CameraRequest>;

#[derive(Debug)]
pub enum Command {
	// Thread commands:
	SubscribeNewCamera {
		camera_id: CameraId,
		res_tx: oneshot::Sender<CameraRx>,
	},

	#[deprecated]
	SubscribeNewListener {
		res_tx: oneshot::Sender<ListenerRx>,
	},

	GetNewStatusChannel {
		res_tx: oneshot::Sender<StatusRx>,
	},

	// Websocket commands:
	RegisterCameras {
		cameras: HashMap<CameraId, NewCameraInfo>,
	},

	UpdateCameraStatus {
		camera_id: CameraId,
		status: CameraStatus,
	},

	ImageAnalysis {
		camera_id: CameraId,
		deviation: Deviation,
	},

	Configuration {
		conn_id: ConnId,
		camera_id: CameraId,
		configuration: String,
	},

	ForgetCameraRequest {
		conn_id: ConnId,
		camera_id: CameraId,
		confirm: bool,
	},

	Alert {
		camera_id: Option<CameraId>,
		alert_type: AlertType,
		alert_code: u32,
		alert_name: String,
		alert_description: String,
	},

	SetNameCamera {
		camera_id: CameraId,
		name: String,
	},

	SendImage {
		camera_id: CameraId,
		frame: Vec<u8>,
	},

	// Miscellaneous commands
	GetDefaultSettings {
		res_tx: oneshot::Sender<EdgeClientResult<Settings>>,
	},

	GetServerURL {
		res_tx: oneshot::Sender<String>,
	},
}

/// Requests sent to individual cameras by the client thread.
#[derive(Debug)]
pub enum CameraRequest {
	/// Sets the name of the camera.
	SetNameCamera(String),

	/// Tells the camera to start(with a value of true) or
	/// stop(with a value of false) sending images through the client.
	ToggleViewingCamera(bool),

	/// Updates the configuration of the camera.
	Configuration(String),

	/// Requests the configuration of the camera. The attached `ConnId` should
	/// be included when sending the configuration.
	ViewCameraConfig(ConnId),

	/// Tells the camera to take a setpoint in its analysis method.
	Setpoint,
}

/// Requests sent to the Persistence struct by the client thread.
#[derive(Debug)]
pub enum PersistenceRequest {
	/// Indicates a camera should be set to active, regardless of current
	/// structure.
	ForceRetryCamera(CameraId),

	/// Indicates a camera should be removed from the persistence struct.
	ForgetCameraRequest(ConnId, CameraId),
}
