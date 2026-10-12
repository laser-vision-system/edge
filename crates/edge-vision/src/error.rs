//! Defines error handling types for the [edge-vision](crate) crate.

use crate::vision_variant::cnn::CnnError;

/// Specialised [`Result`] type for the [edge-vision](crate) crate.
///
/// Shorthand for a result with the [`EdgeVisionError`] error type.
///
/// [`Result`]: std::result::Result
#[expect(clippy::absolute_paths, reason = "Single use aliasing")]
pub type EdgeVisionResult<T> = std::result::Result<T, EdgeVisionError>;

#[derive(Debug, thiserror::Error)]
pub enum EdgeVisionError {
	#[error(transparent)]
	Cnn(#[from] CnnError),
}
