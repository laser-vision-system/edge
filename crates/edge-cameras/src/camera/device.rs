//! A physical camera device, as made available by [`nokhwa`].

mod controls;
#[cfg(test)]
mod tests;

use edge_core::{
	logger::logger,
	settings::CAMERA_BUFFER_COUNT,
};
pub use nokhwa::utils::{
	CameraIndex,
	CameraInfo,
	ControlValueSetter,
	KnownCameraControl,
	KnownCameraControlFlag,
};
use nokhwa::{
	OpenRequest,
	OpenedCamera,
	StreamCamera,
	session::open_with_buffers,
};

use super::CameraLike as _;
use crate::error::{
	EdgeCamerasError,
	EdgeCamerasResult,
};

/// A physical camera device wrapping a Nokhwa stream camera.
///
/// Prefer using [`CameraLike`](super::CameraLike) when defining type bounds.
#[must_use]
pub struct CameraDevice {
	inner: StreamCamera,
}

impl CameraDevice {
	/// Initialise a physical camera at the given index.
	///
	/// # Errors
	///
	/// If Nokhwa cannot initialise the camera or reports an unsupported
	/// shutter.
	pub fn try_new(index: CameraIndex) -> EdgeCamerasResult<Self> {
		let index_string = index.to_string();
		let opened =
			open_with_buffers(index, OpenRequest::any(), CAMERA_BUFFER_COUNT)
				.map_err(|source| {
				EdgeCamerasError::NokhwaInitialisation {
					index: index_string,
					source,
				}
			})?;
		let inner = match opened {
			OpenedCamera::Stream(stream) => stream,
			OpenedCamera::Shutter(_) => {
				return Err(EdgeCamerasError::UnsupportedCameraType {
					cam_type: "Shutter".into(),
				});
			},
			OpenedCamera::Hybrid(hybrid) => {
				logger().log_warn(
					"HybridCamera detected, converting to StreamCamera",
				);
				hybrid.into()
			},
		};
		Ok(Self {
			inner,
		})
	}

	/// Initialise and immediately open a camera.
	///
	/// # Errors
	///
	/// If initialisation or stream opening fails.
	pub fn try_new_open(index: CameraIndex) -> EdgeCamerasResult<Self> {
		Self::try_new(index)?.open()
	}

	/// Initialise and open a camera, then apply reported control defaults.
	///
	/// # Errors
	///
	/// If initialisation, opening, or a supported control write fails.
	pub fn try_new_open_with_default_controls(
		index: CameraIndex
	) -> EdgeCamerasResult<Self> {
		Self::try_new_open(index)?.with_default_controls()
	}

	/// Open the camera stream.
	///
	/// # Errors
	///
	/// If Nokhwa cannot open the stream.
	pub fn open(mut self) -> EdgeCamerasResult<Self> {
		self.inner.open().map_err(EdgeCamerasError::StreamOpen)?;
		Ok(self)
	}

	/// Apply available control defaults and return the camera.
	///
	/// Controls without defaults are skipped.
	///
	/// # Errors
	///
	/// If reading or setting controls fails.
	pub fn with_default_controls(mut self) -> EdgeCamerasResult<Self> {
		self.reset_controls_to_default()?;
		Ok(self)
	}

	/// Set auto controls to manual.
	///
	/// # Errors
	///
	/// If the control has an unexpected type or cannot be set.
	pub fn with_manual_controls(mut self) -> EdgeCamerasResult<Self> {
		self.set_auto_control_manual(KnownCameraControl::AutoExposure)?;
		self.set_auto_control_manual(KnownCameraControl::AutoWhiteBalance)?;
		Ok(self)
	}

	/// Try to set the camera controls to default.
	///
	/// # Errors
	///
	/// If reading or setting any control fails.
	pub fn try_set_default_controls(&mut self) -> EdgeCamerasResult {
		self.reset_controls_to_default()
	}

	/// Try to set auto control to manual.
	///
	/// # Errors
	///
	/// If any contol has an unexpected type or cannot be set.
	pub fn try_set_manual_controls(&mut self) -> EdgeCamerasResult {
		self.set_auto_control_manual(KnownCameraControl::AutoExposure)?;
		self.set_auto_control_manual(KnownCameraControl::AutoWhiteBalance)?;
		Ok(())
	}
}
