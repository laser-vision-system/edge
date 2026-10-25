//! Defines error handling types for the [edge-vision](crate) crate.

use crate::vision_variant::cnn::CnnError;

/// Specialised [`Result`] type for the [edge-vision](crate) crate.
///
/// Shorthand for a result with the [`EdgeVisionError`] error type.
///
/// [`Result`]: std::result::Result
#[expect(clippy::absolute_paths, reason = "Single use aliasing")]
pub type EdgeVisionResult<T> = std::result::Result<T, EdgeVisionError>;

/// The error type for all operations in [edge-vision](crate).
///
/// Any errors from external crates should be translated to this immediately.
#[derive(Debug, thiserror::Error)]
pub enum EdgeVisionError {
	/// An error occured with the [`CNN`](crate::vision_variant::cnn) variant.
	#[error(transparent)]
	Cnn(#[from] CnnError),
}
