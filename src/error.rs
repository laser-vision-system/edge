use serde_json::error::Error as SerdeError;

use thiserror::Error;

/// Specialised [`Result`] type for the Edge tier.
///
/// Shorthand for a result with the [`EdgeError`] error type.
///
/// [`Result`]: std::result::Result
pub type Result<T> = std::result::Result<T, EdgeError>;

/// The error type for all operations in the Edge tier.
///
/// Any errors from underlying crates should be translated to this immediately.
#[derive(Debug, Error)]
pub enum EdgeError {
	#[error("{reason}")]
	General { reason: String },
	#[error("The server could not be reached when trying initial connection")]
	AutoConnect,
	#[error("Fail connect: {reason}")]
	AutoConnectUrl { reason: String },
	#[error("The WebSocket connection could not be created")]
	WebSocketConnection,
	#[error("The server could not be reached")]
	ServerConnection,
	#[error("Nokhwa does not have a backend to use")]
	NokhwaNoBackend,
	#[error("The nokhwa camera could not be initialised at index `{index}`: '{reason}'")]
	NokhwaInitialisation { index: String, reason: String },
	#[error("The nokhwa camera stream could not be started: '{reason}'")]
	StreamOpen { reason: String },
	#[error("The frame was not able to be read by nokhwa")]
	FrameRead,
	#[error("The frame could not be decoded because it is invalid")]
	FrameDecodeCorruption,
	#[error("The frame could not be decoded because not more memory could be allocated")]
	FrameDecodeOutOfMemory,
	#[error("The control could not be set on a camera")]
	SetControl,
	#[error("Failed to decode the controls sent by the server: {reason}")]
	DecodeControls { reason: SerdeError },
	#[error("Attempted to access a camera that does not exist")]
	CameraNotPresent,
	#[error("Failed to unpack data from the websocket")]
	WebSocketUnpack,
	#[error("Error while finding the default interface")]
	IdentifyNetwork,
	#[error("Failed to get a response: {reason}")]
	ReqwestError { reason: String },
	#[error("This device connection could not be established at any of the indices")]
	InvalidDeviceConnection,
	#[error("The connection status was not correct for this request")]
	IncorrectConnectionStatus,
	#[error("Attempting to operate on an unknown camera id")]
	UnknownCameraId,
}

impl Clone for EdgeError {
	fn clone(&self) -> Self {
		match self {
			Self::General { reason } => Self::General {
				reason: reason.clone(),
			},
			Self::AutoConnect => Self::AutoConnect,
			Self::AutoConnectUrl { reason } => Self::AutoConnectUrl {
				reason: reason.clone(),
			},
			Self::WebSocketConnection => Self::WebSocketConnection,
			Self::ServerConnection => Self::ServerConnection,
			Self::NokhwaNoBackend => Self::NokhwaNoBackend,
			Self::NokhwaInitialisation { index, reason } => Self::NokhwaInitialisation {
				index: index.clone(),
				reason: reason.clone(),
			},
			Self::StreamOpen { reason } => Self::StreamOpen {
				reason: reason.clone(),
			},
			Self::FrameRead => Self::FrameRead,
			Self::FrameDecodeCorruption => Self::FrameDecodeCorruption,
			Self::FrameDecodeOutOfMemory => Self::FrameDecodeOutOfMemory,
			Self::SetControl => Self::SetControl,
			// Down-convert this to a general error because serde_json's error doesn't implement clone.
			Self::DecodeControls { reason } => Self::General {
				reason: format!("Failed to decode the controls sent by the server: {reason}",),
			},
			Self::CameraNotPresent => Self::CameraNotPresent,
			Self::WebSocketUnpack => Self::WebSocketUnpack,
			Self::IdentifyNetwork => Self::IdentifyNetwork,
			Self::ReqwestError { reason } => Self::ReqwestError {
				reason: reason.clone(),
			},
			Self::InvalidDeviceConnection => Self::InvalidDeviceConnection,
			Self::IncorrectConnectionStatus => Self::IncorrectConnectionStatus,
			Self::UnknownCameraId => Self::UnknownCameraId,
		}
	}
}
