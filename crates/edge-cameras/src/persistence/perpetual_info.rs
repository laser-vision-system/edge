//! Data attached to a valid device that must be retained through state changes.

use edge_vision::{
	vision,
	vision_interface::VisionInterface as _,
	vision_state::VisionState,
};

use super::ErrAccType;

/// Information about a camera that persists after the device is
/// disconnected.
#[derive(Clone, Debug, Default)]
pub struct PerpetualInfo {
	/// Whether this camera is sending the images to the server.
	currently_viewed: bool,
	/// An error accumulation value. If this value gets too high, the camera
	/// [`CameraStatus`] should be changed to [`Broken`].
	///
	/// [`CameraStatus`]: api::shared::CameraStatus
	/// [`Broken`]: api::shared::CameraStatus::Broken
	err_accumulation: ErrAccType,
	/// Whether to change the setpoint to the immediate next measurement.
	calibrate: bool,

	/// Local holding of the vision processor state.
	/// IDEA: Instead of Option, leave it to the default variant.
	vision_state: VisionState,
}

impl PerpetualInfo {
	/// Query if this device is currently being viewed.
	pub fn get_currently_viewed(&self) -> bool {
		self.currently_viewed
	}

	/// Mark if this device should be viewed.
	pub fn set_currently_viewed(
		&mut self,
		state: bool,
	) {
		self.currently_viewed = state;
	}

	/// Get the current error accumulation value.
	pub fn err_accumulation(&self) -> ErrAccType {
		self.err_accumulation
	}

	/// Mark that this camera has encountered an error.
	pub fn add_error(
		&mut self,
		increment: ErrAccType,
	) {
		self.err_accumulation += increment;
	}

	#[expect(
		clippy::else_if_without_else,
		reason = "This should be a no-op when err_accumulation is at 0"
	)]
	/// Reduce the error by up to an amount.
	pub fn reduce_error(
		&mut self,
		amount: ErrAccType,
	) {
		if self.err_accumulation > amount {
			self.err_accumulation -= amount;
		} else if self.err_accumulation > 0 {
			self.err_accumulation = 0;
		}
	}

	/// Query if this device should be calibrated.
	pub fn get_calibrate(&self) -> bool {
		self.calibrate
	}

	/// Mark if this device should be calibrated.
	pub fn set_calibrate(
		&mut self,
		state: bool,
	) {
		self.calibrate = state;
	}

	/// Check if the [`VisionState`] matches the current variant, updating if
	/// not.
	pub fn refresh_vision_state(&mut self) {
		if !self.vision_state.is_valid() {
			self.vision_state = vision().interact(|vision| vision.new_state());
		}
	}

	/// Return a mutable reference to the [`VisionState`].
	pub fn get_vision_state(&mut self) -> &mut VisionState {
		&mut self.vision_state
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_values_as_expected() {
		let meta = PerpetualInfo::default();
		assert!(!meta.currently_viewed);
		assert_eq!(meta.err_accumulation, 0);
	}

	#[test]
	fn currently_viewed_set_and_read() {
		let mut meta = PerpetualInfo::default();
		assert!(!meta.currently_viewed);
		meta.currently_viewed = true;
		assert!(meta.currently_viewed);
	}

	#[test]
	fn err_accumulation_add() {
		let mut meta = PerpetualInfo::default();
		assert_eq!(meta.err_accumulation, 0);
		meta.add_error(2);
		meta.add_error(2);
		assert_eq!(meta.err_accumulation, 4);
	}
}
