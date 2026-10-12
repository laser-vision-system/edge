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
	#[error(transparent)]
	NokhwaBackend(#[from] NokhwaBackend),
	#[error(
		"The nokhwa camera could not be initialised at index `{index}`: '{inner}'"
	)]
	NokhwaInitialisation {
		index: String,
		inner: NokhwaInitialisation,
	},
	#[error("We do not currently have support for the '{cam_type}' type")]
	UnsupportedCameraType { cam_type: String },
	#[error(transparent)]
	StreamOpen(#[from] StreamOpen),
	#[error(transparent)]
	FrameRead(#[from] FrameRead),
	#[error("Failed to set control {id} to {value}: {inner}")]
	SetControl {
		id: KnownCameraControl,
		value: String,
		inner: SetControl,
	},
	#[error("Attempting to set a control of an unsupported type: {0}")]
	SetControlType(KnownCameraControl),
	#[error("Attempted to access a camera that does not exist")]
	CameraNotPresent,
	#[error(transparent)]
	CannotGetControls(#[from] CannotGetControls),
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

#[derive(Clone, Debug, thiserror::Error)]
#[error(transparent)]
pub struct NokhwaBackend(#[from] NokhwaError);

#[derive(Clone, Debug, thiserror::Error)]
#[error(transparent)]
pub struct NokhwaInitialisation(#[from] NokhwaError);

#[derive(Clone, Debug, thiserror::Error)]
#[error(transparent)]
pub struct StreamOpen(#[from] NokhwaError);

#[derive(Clone, Debug, thiserror::Error)]
#[error(transparent)]
pub struct FrameRead(#[from] NokhwaError);

#[derive(Clone, Debug, thiserror::Error)]
#[error(transparent)]
pub struct SetControl(#[from] NokhwaError);

#[derive(Clone, Debug, thiserror::Error)]
#[error(transparent)]
pub struct CannotGetControls(#[from] NokhwaError);
