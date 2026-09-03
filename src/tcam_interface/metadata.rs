use std::collections::HashSet;

use super::{Camera, Status};

type _ErrAccType = u8;
// IDEA: Use nokhwa's internal `CameraIndex` type.
type _IndexType = u32;

/// Named type for clarity.
pub(crate) type InsertWasSuccessful = bool;

/// Information about a camera device that persists after the device is disconnected.
pub(crate) struct Metadata {
	// ==== Persistent data ====
	/// A user configurable name for this camera.
	///
	/// It is not required to be unique, but it is best if it is.
	///
	/// If side-effects are needed when this value is changed, make it private and use accessor methods.
	pub(crate) name: String,
	/// The current status of the camera.
	pub(super) status: Status,
	/// Whether this camera is sending the images to the server.
	pub(super) currently_viewed: bool,
	/// An error accumulation value. If this value gets too high, the camera [`Status`] should be changed to [`Broken`](Status::Broken)
	/// TODO: Error threshold.
	err_accumulation: _ErrAccType,

	// ==== Temporary data ====
	/// All of the indicies that the camera is occupying.
	indices: Option<HashSet<_IndexType>>,
	/// The camera device where images are pulled from.
	camera: Option<Camera>,
}

impl Default for Metadata {
	fn default() -> Self {
		Self {
			name: String::from("unnamed camera"),
			status: Default::default(),
			currently_viewed: false,
			err_accumulation: 0,
			indices: None,
			camera: None,
		}
	}
}

impl Metadata {
	/// Get the current status of this camera.
	pub(crate) fn status(&self) -> Status {
		self.status
	}

	/// Get the current error accumulation value.
	pub(crate) fn err_accumulation(&self) -> _ErrAccType {
		self.err_accumulation
	}

	/// Mark that this camera has encountered an error.
	pub(crate) fn add_error(&mut self) {
		self.err_accumulation += 2;
	}

	/// Get the list of all device indices that this camera is occupying.
	pub(crate) fn indices(&self) -> Option<&HashSet<_IndexType>> {
		self.indices.as_ref()
	}

	/// Check whether an index is present.
	///
	/// Returns false if the [`Option`] is [`None`].
	pub(crate) fn indices_has(&self, index: _IndexType) -> bool {
		self.indices().is_some_and(|ind| ind.contains(&index))
	}

	/// Attempt to add an index that a camera is determined to be occupying.
	///
	/// If this fails, it's because the device is already initialised and can be ignored.
	pub(crate) fn try_insert_index(&mut self, index: _IndexType) -> InsertWasSuccessful {
		self.indices.get_or_insert_default().insert(index)
	}

	/// Get a reference to the current camera (if connected).
	pub(crate) fn camera(&self) -> Option<&Camera> {
		self.camera.as_ref()
	}

	/// Get a mutable reference to the current camera (if connected).
	pub(crate) fn camera_mut(&mut self) -> Option<&mut Camera> {
		self.camera.as_mut()
	}

	/// Add a camera instance to this this structure.
	///
	/// This will drop an existing camera. Should it check first?
	pub(crate) fn add_camera(&mut self, camera: Camera) {
		self.camera.replace(camera);
	}

	/// Drop the current camera value and all indices.
	pub(crate) fn remove_camera(&mut self) {
		self.camera.take();
		self.indices.take();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_values_as_expected() {
		let meta = Metadata::default();
		assert_eq!(meta.name, String::from("unnamed camera"));
		assert_eq!(meta.status, Status::Uninitialised);
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
		assert_eq!(meta.status, Status::Uninitialised);
		meta.status = Status::Active;
		assert_eq!(meta.status(), Status::Active);
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
		meta.add_error();
		meta.add_error();
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
