//! Defines error handling types for the [edge](crate) crate.

/// Specialised [`Result`] type for the Edge tier.
///
/// Shorthand for a result with the [`EdgeError`] error type.
///
/// [`Result`]: std::result::Result
#[expect(clippy::absolute_paths, reason = "Single use aliasing")]
pub type EdgeResult<T = ()> = std::result::Result<T, EdgeError>;

/// The error type for all operations in [edge](crate).
///
/// Any errors from underlying crates should be translated to this immediately.
#[derive(Clone, Debug, thiserror::Error)]
pub enum EdgeError {
	/// An error for when a specific type does not yet exist.
	#[error("{0}")]
	General(String),
}

// Needed because String doesn't auto-convert to `dyn Error`.
impl From<String> for EdgeError {
	fn from(value: String) -> Self {
		EdgeError::General(value)
	}
}
