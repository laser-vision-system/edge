//! Defines error handling types for the [edge-cameras](crate) crate.

use edge_core::error::EdgeCoreError;
use nokhwa::{
	NokhwaError,
	utils::KnownCameraControl,
};

/// Specialised [`Result`] type for the [edge-cameras](crate) crate.
///
/// Shorthand for a result with the [`EdgeCamerasError`] error type.
///
/// [`Result`]: std::result::Result
#[expect(clippy::absolute_paths, reason = "Single use aliasing")]
pub type EdgeCamerasResult<T> = std::result::Result<T, EdgeCamerasError>;

#[derive(Clone, Debug, thiserror::Error)]
pub enum EdgeCamerasError {
	#[error(transparent)]
	EdgeCore(#[from] EdgeCoreError),

	#[error("{0}")]
	General(String),

	// ===== CameraDevice Errors =====

	// TODO: This variant is not used, find a place for it or remove.
	#[error("{0}")]
	NokhwaBackend(#[source] NokhwaError),
	#[error(
		"The nokhwa camera could not be initialised at index `{index}`: '{source}'"
	)]
	NokhwaInitialisation {
		index: String,
		#[source]
		source: NokhwaError,
	},
	#[error("We do not currently have support for the '{cam_type}' type")]
	UnsupportedCameraType { cam_type: String },
	#[error("{0}")]
	StreamOpen(#[source] NokhwaError),
	#[error("{0}")]
	FrameRead(#[source] NokhwaError),
	#[error("Failed to set control {id} to {value}: {source}")]
	SetControl {
		id: KnownCameraControl,
		value: String,
		#[source]
		source: NokhwaError,
	},
	#[error("Attempting to set a control of an unsupported type: {0}")]
	SetControlType(KnownCameraControl),
	#[error("{0}")]
	CannotGetControls(#[source] NokhwaError),
	#[error("Unable to get control of type '{0}'")]
	GetControl(KnownCameraControl),
	#[error("Attempting to get a control of an unsupported type: {0}")]
	GetControlType(KnownCameraControl),
	#[error("Attempting to set an invalid control value")]
	InvalidControlValue,

	// ===== Persistence Errors =====
	#[error("Attempting to operate on an unknown camera id")]
	UnknownCameraId,
	#[error(
		"This device connection could not be established at any of the indices"
	)]
	InvalidDeviceConnection,
	#[error("The connection status was not correct for this request")]
	IncorrectConnectionStatus,
}
