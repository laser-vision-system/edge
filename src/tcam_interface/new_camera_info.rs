use super::Status;

/// Information about a newly instantiated device, to be sent to the server.
// IDEA: This might be better suited in the server-edge-datatypes crate
#[derive(Clone, Debug)]
pub struct NewCameraInfo {
	pub name: String,
	pub status: Status,
}

impl NewCameraInfo {
	pub fn new(name: String, status: Status) -> Self {
		Self { name, status }
	}
}

impl From<(String, Status)> for NewCameraInfo {
	fn from(value: (String, Status)) -> Self {
		NewCameraInfo {
			name: value.0,
			status: value.1,
		}
	}
}
