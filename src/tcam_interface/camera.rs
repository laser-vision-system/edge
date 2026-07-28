//! # Camera
//!
//! An abstraction over the nokhwa camera objects.
//!
//! Nokhwa's callback camera literally just wraps the regular camera and executes a loop on a spawned thread. There seems to be no benefit to using it.
//!
//! ## TODO
//!
//! The `frame()` function seems to copy the data into a buffer, and then calls decode image which will copy the data again. It may be a good idea to use `frame_raw()` to avoid pointless copy operations.

use image::RgbImage;
use nokhwa::{
	self, NokhwaError, pixel_format,
	utils::{CameraIndex, RequestedFormat, RequestedFormatType},
};

/// An abstraction over the camera provided by Nokhwa.
#[must_use]
pub struct Camera(Option<nokhwa::Camera>);

impl Camera {
	pub fn new(index: u32) -> Result<Self, NokhwaError> {
		let format = RequestedFormat::new::<pixel_format::RgbFormat>(
			RequestedFormatType::AbsoluteHighestFrameRate,
		);

		// TODO: Handle potential error cases.
		let mut camera = nokhwa::Camera::new(CameraIndex::Index(index), format)?;
		camera.open_stream()?;

		Ok(Camera(Some(camera)))
	}

	/// Request an image of the current camera frame.
	pub fn get_image(&mut self) -> Result<RgbImage, NokhwaError> {
		let Some(cam) = self.0.as_mut() else {
			// HACK: Temporarily use nokhwa's error type.
			return Err(NokhwaError::GeneralError(String::from(
				"camera doesn't exist",
			)));
		};
		// TODO: Handle potential error cases.
		let frame = cam.frame()?;
		let decoded = frame.decode_image::<pixel_format::RgbFormat>()?;
		Ok(decoded)
	}

	// report configurations

	// apply configurations
}
