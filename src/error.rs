#![expect(
	clippy::module_name_repetitions,
	reason = "Distinct error type for clarity"
)]
use serde_json::error::Error as SerdeError;

/// Specialised [`Result`] type for the Edge tier.
///
/// Shorthand for a result with the [`EdgeError`] error type.
///
/// [`Result`]: std::result::Result
pub type Result<T> = std::result::Result<T, EdgeError>;

/// The error type for all operations in the Edge tier.
///
/// Any errors from underlying crates should be translated to this immediately.
#[derive(Debug, thiserror::Error)]
pub enum EdgeError {
	/// Non-specific error type, please create a dedicated variant instead.
	#[error("{reason}")]
	General { reason: String },
	#[error(transparent)]
	CameraDeviceError(#[from] CameraDeviceError),
	#[error(transparent)]
	NetworkError(#[from] NetworkError),
	#[error(transparent)]
	PersistenceError(#[from] PersistenceError),
	#[error("Failed to decode the controls sent by the server: {reason}")]
	DecodeControls {
		#[from]
		reason: SerdeError,
	},
}

impl Clone for EdgeError {
	/// May return a duplicate of this error.
	///
	/// This method is destruct to the [`DecodeControls`] variant due to the
	/// inner type not supporting clone.
	///
	/// [`DecodeControls`]: Self::DecodeControls
	fn clone(&self) -> Self {
		match self {
			// Down-convert this to a general error because serde_json's error
			// doesn't implement clone.
			Self::DecodeControls { reason } => Self::General {
				reason: format!(
					"Failed to decode the controls sent by the server: {reason}",
				),
			},
			other => other.clone(),
		}
	}
}

impl From<String> for EdgeError {
	fn from(value: String) -> Self {
		EdgeError::General { reason: value }
	}
}

/// Errors specific to the [`CameraDevice`](crate::tcam_interface::CameraDevice)
/// type.
// NOTE: Maybe this should be in that module?
#[derive(Clone, Debug, thiserror::Error)]
pub enum CameraDeviceError {
	// TODO: This variant is not used, find a place for it or remove.
	#[error("Nokhwa does not have a backend to use")]
	NokhwaNoBackend,
	#[error(
		"The nokhwa camera could not be initialised at index `{index}`: '{reason}'"
	)]
	NokhwaInitialisation { index: String, reason: String },
	#[error("The nokhwa camera stream could not be started: '{reason}'")]
	StreamOpen { reason: String },
	#[error("The frame was not able to be read by nokhwa")]
	FrameRead,
	#[error("The frame could not be decoded because it is invalid")]
	FrameDecodeCorruption,
	// TODO: This variant is not used, find a place for it or remove.
	#[error(
		"The frame could not be decoded because not more memory could be allocated"
	)]
	FrameDecodeOutOfMemory,
	#[error("The control could not be set on a camera")]
	SetControl,
	#[error("Attempted to access a camera that does not exist")]
	CameraNotPresent,
	#[error("Attempting to set an invalid control value")]
	InvalidControlValue,
}

/// Errors specific to networking operations.
#[derive(Clone, Debug, thiserror::Error)]
pub enum NetworkError {
	#[error("The server could not be reached when trying initial connection")]
	AutoConnect,
	#[error("Fail connect: {reason}")]
	AutoConnectUrl { reason: String },
	#[error("The WebSocket connection could not be created")]
	WebSocketConnection,
	#[error("The server could not be reached")]
	ServerConnection,
	#[error("Failed to unpack data from the websocket")]
	WebSocketUnpack,
	// TODO: This variant is not used, find a place for it or remove.
	#[error("Error while finding the default interface")]
	IdentifyNetwork,
	#[error("Failed to get a response: {reason}")]
	ReqwestError { reason: String },
}

/// Errors specific to camera persistence logic.
#[derive(Clone, Debug, thiserror::Error)]
pub enum PersistenceError {
	#[error("Attempting to operate on an unknown camera id")]
	UnknownCameraId,
	#[error(
		"This device connection could not be established at any of the indices"
	)]
	InvalidDeviceConnection,
	#[error("The connection status was not correct for this request")]
	IncorrectConnectionStatus,
}
