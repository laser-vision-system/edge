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

/// The error type for all operations in [edge-cameras](crate).
///
/// Any errors from external crates should be translated to this immediately.
#[derive(Clone, Debug, thiserror::Error)]
pub enum EdgeCamerasError {
	/// A translation of an error from the core crate.
	#[error(transparent)]
	EdgeCore(#[from] EdgeCoreError),

	/// An error for when a specific type does not yet exist.
	#[error("{0}")]
	General(String),

	// ===== CameraDevice Errors =====
	/// There was an issue with the selected backend, or lack-thereof.
	// TODO: This variant is not used, find a place for it or remove.
	#[error("{0}")]
	NokhwaBackend(#[source] NokhwaError),
	/// An issue occured when trying to initialise a camera.
	#[error(
		"The nokhwa camera could not be initialised at index `{index}`: '{source}'"
	)]
	NokhwaInitialisation {
		/// The index of the device that attempted to initialise.
		index: String,
		/// The soruce error type.
		#[source]
		source: NokhwaError,
	},
	/// Tried to initialise a camera of an unsupported type.
	#[error("We do not currently have support for the '{cam_type}' type")]
	UnsupportedCameraType {
		/// The type name of the
		/// [`OpenedCamera`](nokhwa::session::OpenedCamera) variant.
		cam_type: String,
	},
	/// The camera's stream could not be opened.
	#[error("{0}")]
	StreamOpen(#[source] NokhwaError),
	/// A frame could not be read from the camera.
	#[error("{0}")]
	FrameRead(#[source] NokhwaError),
	/// Unable to set a camera control.
	#[error("Failed to set control {id} to {value}: {source}")]
	SetControl {
		/// The ID of the camera control.
		id: KnownCameraControl,
		/// The value that it was attempted to be set to.
		value: String,
		/// The soruce error type.
		#[source]
		source: NokhwaError,
	},
	/// A control description type is not currently supported.
	#[error("Attempting to set a control of an unsupported type: {0}")]
	SetControlType(KnownCameraControl),
	/// The camera offers up no controls at all.
	#[error("{0}")]
	CannotGetControls(#[source] NokhwaError),
	/// Unable to get a camera control with the provided ID.
	#[error("Unable to get control of type '{0}'")]
	GetControl(KnownCameraControl),
	/// A control description type is not currently supported.
	#[error("Attempting to get a control of an unsupported type: {0}")]
	GetControlType(KnownCameraControl),
	/// Tried to set a control to an invalid value.
	#[error("Attempting to set an invalid control value")]
	InvalidControlValue,

	// ===== Persistence Errors =====
	/// Tried to access persistence with an unknown camera id.
	#[error("Attempting to operate on an unknown camera id")]
	UnknownCameraId,
	/// A device was found that could not be initialised as a camera.
	#[error(
		"This device connection could not be established at any of the indices"
	)]
	InvalidDeviceConnection,
	/// Attempted to make a request on an entry with the wrong status.
	#[error("The connection status was not correct for this request")]
	IncorrectConnectionStatus,
}
