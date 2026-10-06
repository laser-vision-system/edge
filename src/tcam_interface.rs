//! Provides connection, identification, management, and configuration for
//! camera devices.

mod camera_device;
pub mod controls;
mod identifier;
mod persistence;

pub use camera_device::CameraDevice;
pub use identifier::Identifier;
use nokhwa::{NokhwaError, utils::KnownCameraControl};
pub use persistence::{
	MaybeBatch, MaybeImage, PerpetualInfo, PersistEntry, Persistence, WhyNo,
	polling,
};

/// Errors specific to the [`CameraDevice`](crate::tcam_interface::CameraDevice)
/// type.
// NOTE: Maybe this should be in that module?
#[derive(Clone, Debug, thiserror::Error)]
pub enum CameraDeviceError {
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
	#[error(transparent)]
	SelectDecodeFormat(#[from] SelectDecodeFormat),
	#[error("The frame could not be decoded because it is invalid")]
	FrameDecodeCorruption(#[from] FrameDecodeCorruption),
	// TODO: This variant is not used, find a place for it or remove.
	#[error(
		"The frame could not be decoded because not more memory could be allocated"
	)]
	FrameDecodeOutOfMemory(#[from] FrameDecodeOutOfMemory),
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
pub struct SelectDecodeFormat(#[from] NokhwaError);

#[derive(Clone, Debug, thiserror::Error)]
#[error(transparent)]
pub struct FrameDecodeCorruption(#[from] NokhwaError);

#[derive(Clone, Debug, thiserror::Error)]
#[error(transparent)]
pub struct FrameDecodeOutOfMemory(#[from] NokhwaError);

#[derive(Clone, Debug, thiserror::Error)]
#[error(transparent)]
pub struct SetControl(#[from] NokhwaError);

#[derive(Clone, Debug, thiserror::Error)]
#[error(transparent)]
pub struct CannotGetControls(#[from] NokhwaError);

/// Errors specific to camera persistence logic.
#[derive(Clone, Debug, thiserror::Error)]
pub enum PersistenceError {
	#[error("Attempting to operate on an unknown camera id")]
	UnknownCameraId,
	#[error(
		"This device connection could not be established at any of the indices"
	)]
	InvalidDeviceConnection,
	#[error("The connection status was not correct for this request")]
	IncorrectConnectionStatus,
}
