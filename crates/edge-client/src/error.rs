//! Defines error handling types for the [edge-client](crate) crate.

/// Specialised [`Result`] type for the [edge-client](crate) crate.
///
/// Shorthand for a result with the [`EdgeClientError`] error type.
///
/// [`Result`]: std::result::Result
#[expect(clippy::absolute_paths, reason = "Single use aliasing")]
pub type EdgeClientResult<T> = std::result::Result<T, EdgeClientError>;

#[derive(Clone, Debug, thiserror::Error)]
pub enum EdgeClientError {
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
	#[error("Could not find mac address")]
	MacAddress,
}
