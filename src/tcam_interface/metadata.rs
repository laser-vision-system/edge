use std::collections::HashSet;

use image::RgbImage;
use server_edge_datatypes::{
	edge::{NewCameraInfo, Settings},
	shared::{CameraId, CameraStatus},
};

use crate::{
	error::Result,
	logger::{self, logger},
	vision_processor::{BatchItem, VisionState},
};

use super::Camera;

type ErrAccType = u16;
// IDEA: Use nokhwa's internal `CameraIndex` type.
type IndexType = u32;

/// Named type for clarity.
pub type InsertWasSuccessful = bool;
/// Named type for clarity.
type InitialiseWasSuccessful = bool;

/// Information about a camera device that persists after the device is
/// disconnected.
pub struct Metadata {
	// ==== Persistent data ====
	/// A user configurable name for this camera.
	///
	/// It is not required to be unique, but it is best if it is.
	///
	/// If side-effects are needed when this value is changed, make it private
	/// and use accessor methods.
	pub name: String,
	/// The current status of the camera.
	pub status: CameraStatus,
	/// Whether this camera is sending the images to the server.
	pub currently_viewed: bool,
	/// An error accumulation value. If this value gets too high, the camera
	/// [`CameraStatus`] should be changed to [`Broken`](CameraStatus::Broken) TODO: Error
	/// threshold.
	err_accumulation: ErrAccType,
	/// Whether to change the setpoint to the immediate next measurement.
	/// TODO: Use.
	calibrate: bool,

	// ==== Temporary data ====
	/// All of the indicies that the camera is occupying.
	indices: Option<HashSet<IndexType>>,
	/// The camera device where images are pulled from.
	camera: Option<Camera>,
	/// Local holding of the vision processor state.
	/// TODO: Use.
	vision_state: Option<VisionState>,
}

impl Metadata {
	/// Get the current status of this camera.
	pub fn status(&self) -> CameraStatus {
		self.status
	}

	/// Get the current error accumulation value.
	pub fn err_accumulation(&self) -> ErrAccType {
		self.err_accumulation
	}

	/// Mark that this camera has encountered an error.
	pub fn add_error(&mut self, increment: ErrAccType) {
		self.err_accumulation += increment;
	}

	/// Reduce the error by up to an amount.
	pub fn reduce_error(&mut self, amount: ErrAccType) {
		if self.err_accumulation > amount {
			self.err_accumulation -= amount;
		} else if self.err_accumulation > 0 {
			self.err_accumulation = 0;
		}
	}

	pub fn request_calibration(&mut self) {
		self.calibrate = true;
	}

	/// Get the list of all device indices that this camera is occupying.
	pub fn indices(&self) -> Option<&HashSet<IndexType>> {
		self.indices.as_ref()
	}

	/// Check whether an index is present.
	///
	/// Returns false if the [`Option`] is [`None`].
	pub fn indices_has(&self, index: IndexType) -> bool {
		self.indices().is_some_and(|ind| ind.contains(&index))
	}

	/// Attempt to add an index that a camera is determined to be occupying.
	///
	/// If this fails, it's because the device is already initialised and can be
	/// ignored.
	pub fn try_insert_index(&mut self, index: IndexType) -> InsertWasSuccessful {
		self.indices.get_or_insert_default().insert(index)
	}

	/// Get a reference to the current camera (if connected).
	pub fn camera(&self) -> Option<&Camera> {
		self.camera.as_ref()
	}

	/// Get a mutable reference to the current camera (if connected).
	pub fn camera_mut(&mut self) -> Option<&mut Camera> {
		self.camera.as_mut()
	}

	pub fn get_image(&mut self, settings: Settings) -> Result<RgbImage> {
		let frame = match self
			.camera_mut()
			.expect("camera expected by status being active")
			.get_frame()
		{
			Ok(frame) => frame,
			Err(why) => {
				logger().lock().unwrap().log_error("", &why);
				self.remove_camera();
				return Err(why);
			},
		};

		logger()
			.lock()
			.unwrap()
			.log_info("Successfully read frame from camera");
		self.reduce_error(settings.camera_err_decrement);

		let image = match self
			.camera_mut()
			.expect("camera expected by status being active")
			.decode_image(frame)
		{
			Ok(image) => image,
			Err(why) => {
				// TODO: If out of memory, send log and panic.
				logger().lock().unwrap().log_error("", &why);
				return Err(why);
			},
		};

		Ok(image)
	}

	/// Add a camera instance to this this structure.
	///
	/// This will drop an existing camera. Should it check first?
	fn add_camera(&mut self, camera: Camera) {
		self.camera.replace(camera);
	}

	/// Set up a camera for use.
	///
	/// TODO: Finish implementing; refer to edge flow logic diagram.
	fn initialise_camera(&mut self, index: u32) -> InitialiseWasSuccessful {
		let cam_res = Camera::new(index);
		match cam_res {
			Err(_why) => {
				logger::logger()
					.lock()
					.unwrap()
					.log_error("Could not initialise camera", &_why);
				self.status = CameraStatus::Invalid;
				false
			},
			Ok(camera) => {
				logger::logger()
					.lock()
					.unwrap()
					.log_info("Camera successfully initialised");
				self.status = CameraStatus::Active;
				self.add_camera(camera);
				true
			},
		}
	}

	/// Attempt to add a device to this instance.
	///
	/// Returns [`None`] if:
	///
	/// - The index is already registered
	/// - This is a metadata device
	/// - The camera failed to intialise
	pub fn try_register_device(
		&mut self,
		index: IndexType,
		is_metadata: bool,
	) -> Option<NewCameraInfo> {
		// Does Metadata contain the index?
		// Add index to Metadata.
		if self.try_insert_index(index) {
			return None;
		}

		// Stop now if this is a metadata device.
		if is_metadata {
			logger::logger().lock().unwrap().log_warn(&format!(
				"This index ({}) represents a metadata device",
				index
			));
			return None;
		}

		if !self.initialise_camera(index) {
			return None;
		}

		let device = NewCameraInfo::new(self.name.clone(), self.status);
		Some(device)
	}

	/// Drop the current camera and all of its indices.
	pub fn remove_camera(&mut self) {
		self.camera.take();
		self.indices.take();
	}

	pub fn make_batch_item(&mut self, image: RgbImage, id: CameraId) -> BatchItem<'_> {
		let state = self
			.vision_state
			.as_mut()
			.expect("vision state expected while status is active");
		let calibrate = self.calibrate;
		self.calibrate = false;
		BatchItem::new(id, state, image, calibrate, self.currently_viewed)
	}
}

impl Default for Metadata {
	fn default() -> Self {
		Self {
			name: String::from("unnamed camera"),
			status: Default::default(),
			currently_viewed: false,
			err_accumulation: 0,
			calibrate: false,
			indices: None,
			camera: None,
			vision_state: None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_values_as_expected() {
		let meta = Metadata::default();
		assert_eq!(meta.name, String::from("unnamed camera"));
		assert_eq!(meta.status, CameraStatus::Uninitialised);
		assert_eq!(meta.currently_viewed, false);
		assert_eq!(meta.err_accumulation, 0);
		assert!(meta.indices.is_none());
		assert!(meta.camera.is_none());
	}

	#[test]
	fn name_set_and_read() {
		let mut meta = Metadata::default();
		assert_eq!(meta.name, String::from("unnamed camera"));
		meta.name = String::from("test_name");
		assert_eq!(meta.name, String::from("test_name"));
	}

	#[test]
	fn status_set_and_read() {
		let mut meta = Metadata::default();
		assert_eq!(meta.status, CameraStatus::Uninitialised);
		meta.status = CameraStatus::Active;
		assert_eq!(meta.status(), CameraStatus::Active);
	}

	#[test]
	fn currently_viewed_set_and_read() {
		let mut meta = Metadata::default();
		assert_eq!(meta.currently_viewed, false);
		meta.currently_viewed = true;
		assert_eq!(meta.currently_viewed, true);
	}

	#[test]
	fn err_accumulation_add() {
		let mut meta = Metadata::default();
		assert_eq!(meta.err_accumulation, 0);
		meta.add_error(2);
		meta.add_error(2);
		assert_eq!(meta.err_accumulation, 4);
	}

	#[test]
	fn indices_check_and_insert() {
		let mut meta = Metadata::default();
		assert!(meta.indices.is_none());
		assert!(!meta.indices_has(0));
		assert!(meta.try_insert_index(0));
		assert!(meta.indices_has(0));
		assert!(!meta.try_insert_index(0));
		assert!(meta.try_insert_index(2));
		assert!(meta.indices_has(2));
	}

	#[test]
	#[ignore = "requires a camera device to be connected"]
	fn camera_add() {
		let mut meta = Metadata::default();
		assert!(meta.camera.is_none());
		let camera = Camera::new(0).expect("a valid camera device must exist at index 0");
		meta.add_camera(camera);
		assert!(meta.camera().is_some());
	}

	#[test]
	#[ignore = "requires a camera device to be connected"]
	fn camera_remove() {
		let mut meta = Metadata::default();
		assert!(meta.camera.is_none());
		let camera = Camera::new(0).expect("a valid camera device must exist at index 0");
		meta.add_camera(camera);
		assert!(meta.camera().is_some());
		assert!(meta.try_insert_index(0));
		meta.remove_camera();
		assert!(meta.indices.is_none());
		assert!(meta.camera.is_none());
	}
}
