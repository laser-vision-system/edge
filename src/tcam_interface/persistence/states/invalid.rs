use api::shared::CameraStatus;

use super::IndexOccupier;
use crate::tcam_interface::persistence::OccupiedIndices;

/// A connection in an invalid state.
///
/// It represents a connection that was not able to establish a camera device.
///
/// It can be created from:
/// - A new connection
///
/// It cannot change state to anything.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Invalid {
	pub occupied_indices: OccupiedIndices,
}

impl Invalid {
	pub fn new(occupied_indices: OccupiedIndices) -> Self {
		Self {
			occupied_indices,
		}
	}

	/// Map this type to a [`CameraStatus`].
	pub fn get_status() -> CameraStatus {
		CameraStatus::Invalid
	}
}

impl AsRef<OccupiedIndices> for Invalid {
	fn as_ref(&self) -> &OccupiedIndices {
		&self.occupied_indices
	}
}

impl AsMut<OccupiedIndices> for Invalid {
	fn as_mut(&mut self) -> &mut OccupiedIndices {
		&mut self.occupied_indices
	}
}

impl IndexOccupier for Invalid {
}
