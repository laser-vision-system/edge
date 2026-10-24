//! Defines error handling types for the [edge-core](crate) crate.

use std::env::VarError;

use nokhwa::NokhwaError;

/// Specialised [`Result`] type for the [edge-core](crate) crate.
///
/// Shorthand for a result with the [`EdgeCoreError`] error type.
///
/// [`Result`]: std::result::Result
#[expect(clippy::absolute_paths, reason = "Single use aliasing")]
pub type EdgeCoreResult<T> = std::result::Result<T, EdgeCoreError>;

/// The error type for all operations in [edge-core](crate).
///
/// Any errors from external crates should be translated to this immediately.
#[derive(Clone, Debug, thiserror::Error)]
pub enum EdgeCoreError {
	#[error("Error reading {name} environment variable: {source}")]
	EnvVar {
		name: String,
		#[source]
		source: VarError,
	},
	#[error(
		"Could not parse {name} environment variable to the expected type: {why}"
	)]
	EnvVarParse { name: String, why: String },
	#[error("{0}")]
	SelectDecodeFormat(#[source] NokhwaError),
	#[error("The frame could not be decoded because it is invalid: {0}")]
	FrameDecodeCorruption(#[source] NokhwaError),
	// TODO: This variant is not used, find a place for it or remove.
	#[error(
		"The frame could not be decoded because not more memory could be allocated: {0}"
	)]
	FrameDecodeOutOfMemory(#[source] NokhwaError),
}
