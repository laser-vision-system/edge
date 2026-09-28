//! Shared network utilities.

pub mod auto_connect;
pub mod websocket;
pub mod persistent_edge_id;

use serde::de::DeserializeOwned;

use crate::error::Result;

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

pub fn unpack<T>(data: &str) -> Result<T>
where
	T: DeserializeOwned,
{
	let data_unchecked: serde_json::Result<T> = serde_json::from_str(data);
	data_unchecked.map_err(|_| NetworkError::WebSocketUnpack.into())
}
