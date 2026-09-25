//! Provides connection, identification, management, and configuration for
//! camera devices.

mod camera_device;
pub mod controls;
mod identifier;
mod persistence;

pub use camera_device::CameraDevice;
pub use identifier::Identifier;
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
	#[error("Nokhwa does not have a backend to use")]
	NokhwaNoBackend,
	#[error(
		"The nokhwa camera could not be initialised at index `{index}`: '{reason}'"
	)]
	NokhwaInitialisation { index: String, reason: String },
	#[error("The nokhwa camera stream could not be started: '{reason}'")]
	StreamOpen { reason: String },
	#[error("The frame was not able to be read by nokhwa")]
	FrameRead,
	#[error("The frame could not be decoded because it is invalid")]
	FrameDecodeCorruption,
	// TODO: This variant is not used, find a place for it or remove.
	#[error(
		"The frame could not be decoded because not more memory could be allocated"
	)]
	FrameDecodeOutOfMemory,
	#[error("The control could not be set on a camera")]
	SetControl,
	#[error("Attempted to access a camera that does not exist")]
	CameraNotPresent,
	#[error("Attempting to set an invalid control value")]
	InvalidControlValue,
}

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
