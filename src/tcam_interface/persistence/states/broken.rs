use std::borrow::Cow;

use server_edge_datatypes::shared::CameraStatus;

use super::{
	Active,
	IndexOccupier,
	Limbo,
};
use crate::tcam_interface::persistence::OccupiedIndices;

/// A connection in a broken state.
///
/// It represents a connection that can be initialised to a camera, but fails
/// to read in image data.
///
/// It can be created from:
/// - Active
/// - Limbo
///
/// It can change state to:
/// - Disconnected
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Broken {
	pub occupied_indices: OccupiedIndices,
	/// A user configurable name for this camera.
	///
	/// It is not required to be unique, but is preferrable.
	pub name: String,
}

// Accessors.
impl Broken {
	/// Get the name of this device.
	pub fn get_name(&self) -> Cow<'_, str> {
		Cow::Borrowed(&self.name)
	}

	/// Update the name of this device.
	pub fn set_name(
		&mut self,
		name: String,
	) {
		self.name = name;
	}

	/// Map this type to a [`CameraStatus`].
	pub fn get_status() -> CameraStatus {
		CameraStatus::Broken
	}
}

impl AsRef<OccupiedIndices> for Broken {
	fn as_ref(&self) -> &OccupiedIndices {
		&self.occupied_indices
	}
}

impl AsMut<OccupiedIndices> for Broken {
	fn as_mut(&mut self) -> &mut OccupiedIndices {
		&mut self.occupied_indices
	}
}

impl From<Active> for Broken {
	fn from(active: Active) -> Self {
		Self {
			occupied_indices: active.occupied_indices,
			name: active.name,
		}
	}
}

impl From<Limbo> for Broken {
	fn from(limbo: Limbo) -> Self {
		Self {
			occupied_indices: limbo.occupied_indices,
			name: limbo.name,
		}
	}
}

impl IndexOccupier for Broken {
}
