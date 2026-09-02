use std::collections::HashSet;

use super::{Camera, Status};

type __ErrAccType = u8;
type __IndexType = u32;

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
	err_accumulation: __ErrAccType,

	// ==== Temporary data ====
	/// All of the indicies that the camera is occupying.
	indices: Option<HashSet<__IndexType>>,
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
	pub(crate) fn err_accumulation(&self) -> __ErrAccType {
		self.err_accumulation
	}

	/// Mark that this camera has encountered an error.
	pub(crate) fn add_error(&mut self) {
		self.err_accumulation += 2;
	}

	/// Get the list of all device indices that this camera is occupying.
	pub(crate) fn indices(&self) -> Option<&HashSet<__IndexType>> {
		self.indices.as_ref()
	}

	/// Attempt to add an index that a camera is determined to be occupying.
	///
	/// If this fails, it's because the device is already initialised and can be ignored.
	pub(crate) fn try_insert_index(&mut self, index: __IndexType) -> InsertWasSuccessful {
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
