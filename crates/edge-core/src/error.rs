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
	/// The environment variable was not found.
	#[error("Error reading {name} environment variable: {source}")]
	EnvVar {
		/// The name of the environmnent variable.
		name: String,
		#[source]
		/// The soruce error type.
		source: VarError,
	},
	/// The environment variable was found but failed to parse.
	#[error(
		"Could not parse {name} environment variable to the expected type: {why}"
	)]
	EnvVarParse {
		/// The name of the environmnent variable.
		name: String,
		/// Additional details returned by [`parse`](str::parse).
		why: String,
	},
	/// Attempted to associate the buffer with an incorrect
	/// [`FrameFormat`](nokhwa::utils::FrameFormat).
	#[error("{0}")]
	SelectDecodeFormat(#[source] NokhwaError),
	/// The pixel data is malformed.
	#[error("The frame could not be decoded because it is invalid: {0}")]
	FrameDecodeCorruption(#[source] NokhwaError),
	/// The system ran out of memory while decoding.
	// TODO: This variant is not used, find a place for it or remove.
	#[error(
		"The frame could not be decoded because not more memory could be allocated: {0}"
	)]
	FrameDecodeOutOfMemory(#[source] NokhwaError),
}
