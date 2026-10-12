use std::fmt;

use api::shared::CameraId;
use edge_core::image::Image;

use crate::vision_state::VisionState;

/// A single record that is passed into `process_frame`.
///
/// Currently holds a reference to the vision state, but should probably be
/// swapped to using an [`mpsc`](std::sync::mpsc) for multithreading. The
/// [`VisionState`] would be sent back through it.
pub struct BatchItem<'a> {
	pub camera_id: CameraId,
	pub state: &'a mut VisionState,
	pub image: Image,
	pub calibrate: bool,
	pub send_to_server: bool,
}

impl<'a> BatchItem<'a> {
	pub fn new(
		camera_id: CameraId,
		state: &'a mut VisionState,
		image: Image,
		calibrate: bool,
		send_to_server: bool,
	) -> Self {
		Self {
			camera_id,
			state,
			image,
			calibrate,
			send_to_server,
		}
	}
}

impl fmt::Display for BatchItem<'_> {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		writeln!(
			f,
			"Batch Item: [\n\tID: {}\n\tState: {}\n\tImage\n\tCalibrate: {}\n\tSend: {}\n]",
			self.camera_id, self.state, self.calibrate, self.send_to_server
		)
	}
}
