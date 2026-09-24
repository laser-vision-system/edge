#![expect(
	clippy::module_name_repetitions,
	reason = "Distinct error type for clarity"
)]
use serde_json::error::Error as SerdeError;

use crate::{
	network::NetworkError,
	tcam_interface::{CameraDeviceError, PersistenceError},
};

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
