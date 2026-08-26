use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum EdgeError {
	#[error("The server could not be reached when trying initial connection")]
	AutoConnect,
	#[error("The WebSocket connection could not be created")]
	WebSocketConnection,
	#[error("The server could not be reached")]
	ServerConnection,
	#[error("Nokhwa does not have a backend to use")]
	NokhwaNoBackend,
	#[error("The nokhwa camera could not be initialised at index `{index}`: '{reason}'")]
	NokhwaInitialisation { index: u32, reason: String },
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
	#[error("An error that does not yet have a specific implementation")]
	General(String),
}
