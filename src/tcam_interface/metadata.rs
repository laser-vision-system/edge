use super::{Camera, Status};

/// Information about a camera device that persists after the device is disconnected.
pub struct Metadata {
	// ==== Persistent data ====
	/// A user configurable name for this camera.
	///
	/// It is not required to be unique, but it is best if it is.
	///
	/// If side-effects are needed when this value is changed, make it private and use accessor methods.
	pub name: String,
	/// The current status of the camera.
	status: Status,
	/// Whether this camera is sending the images to the server.
	currently_viewed: bool,
	/// An error accumulation value. If this value gets too high, the camera [`Status`] should be changed to [`Broken`](Status::Broken)
	err_accumulation: u8,

	// ==== Temporary data ====
	/// The current device index.
	index: Option<u16>,
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
			index: None,
			camera: None,
		}
	}
}

impl Metadata {
	pub fn status(&self) -> &Status {
		&self.status
	}

	pub fn set_viewed(&mut self, state: bool) {
		self.currently_viewed = state;
	}

	pub fn err_accumulation(&self) -> u8 {
		self.err_accumulation
	}

	pub fn add_error(&mut self) {
		self.err_accumulation += 2;
	}

	pub fn index(&self) -> Option<u16> {
		self.index
	}

	pub fn camera(&self) -> Option<&Camera> {
		self.camera.as_ref()
	}

	pub fn camera_mut(&mut self) -> Option<&mut Camera> {
		self.camera.as_mut()
	}
}
