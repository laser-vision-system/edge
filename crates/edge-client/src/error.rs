//! Defines error handling types for the [edge-client](crate) crate.

/// Specialised [`Result`] type for the [edge-client](crate) crate.
///
/// Shorthand for a result with the [`EdgeClientError`] error type.
///
/// [`Result`]: std::result::Result
#[expect(clippy::absolute_paths, reason = "Single use aliasing")]
pub type EdgeClientResult<T> = std::result::Result<T, EdgeClientError>;

/// The error type for all operations in [edge-client](crate).
///
/// Any errors from external crates should be translated to this immediately.
#[derive(Clone, Debug, thiserror::Error)]
pub enum EdgeClientError {
	/// The autoconnect process failed.
	#[error("Fail connect: {0}")]
	AutoConnect(String),
	/// Could not retried the websocket url during autoconnect.
	#[error("The server could not be reached when trying initial connection")]
	AutoConnectUrl,
	/// The WebSocket connection could not be created.
	#[error("The WebSocket connection could not be created")]
	WebSocketConnection,
	/// The server could not be reached.
	#[error("The server could not be reached")]
	ServerConnection,
	/// Failed to unpack data from the websocket.
	#[error("Failed to unpack data from the websocket")]
	WebSocketUnpack,
	/// No clue sorry.
	// TODO: This variant is not used, find a place for it or remove.
	#[error("Error while finding the default interface")]
	IdentifyNetwork,
	/// There was an error with a reqwest message.
	#[error("Failed to get a response: {reason}")]
	ReqwestError {
		/// More details as to why.
		reason: String,
	},
	/// Could not find MAC address.
	#[error("Could not find MAC address")]
	MacAddress,
	/// The websocket connection was closed by the server.
	#[error("Websocket closed by server")]
	WebsocketClosed,
	/// The message could not be sent because the websocket is closed.
	#[error("message could not be sent, as the websocket is currently closed")]
	WebsocketDown,
	/// Tungenite had an issue.
	///
	/// It doesn't implement clone so we can't use it directly.
	#[error("{0}")]
	TungsteniteError(String),
}
