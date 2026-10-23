use edge_client::error::EdgeClientError;
use edge_core::logger::logger;

/// Specialised [`Result`] type for the Edge tier.
///
/// Shorthand for a result with the [`EdgeError`] error type.
///
/// [`Result`]: std::result::Result
#[expect(clippy::absolute_paths, reason = "Single use aliasing")]
pub type EdgeResult<T> = std::result::Result<T, EdgeError>;

/// The error type for all operations in the Edge tier.
///
/// Any errors from underlying crates should be translated to this immediately.
#[derive(Clone, Debug, thiserror::Error)]
pub enum EdgeError {
	/// Non-specific error type, please create a dedicated variant instead.
	#[error("{reason}")]
	General { reason: String },
	#[error(transparent)]
	EdgeClientError(#[from] EdgeClientError),
}

impl EdgeError {
	pub fn log(self) {
		logger().log_error(&self);
	}
}

impl From<String> for EdgeError {
	fn from(value: String) -> Self {
		EdgeError::General {
			reason: value,
		}
	}
}
