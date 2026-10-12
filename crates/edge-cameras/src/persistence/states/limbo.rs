use std::{
	borrow::Cow,
	ops::{
		Deref,
		DerefMut,
	},
};

use api::shared::CameraStatus;

use super::{
	Active,
	Disconnected,
	IndexOccupier,
};
use crate::persistence::{
	OccupiedIndices,
	PerpetualInfo,
};

/// A connection in a limbo state.
///
/// It represents a connection that was previously initialised to a camera, but
/// now only occupies indices. This state should passively accumulate errors
/// until either it reconnects, or is moved to broken.
///
/// It can be created from:
/// - Active
/// - Disconnected
///
/// It can change state to:
/// - Active
/// - Broken
/// - Disconnected
#[derive(Clone, Debug, Default)]
pub struct Limbo {
	pub occupied_indices: OccupiedIndices,
	pub data: PerpetualInfo,
	/// A user configurable name for this camera.
	///
	/// It is not required to be unique, but is preferrable.
	pub name: String,
}

// Constructors.
impl Limbo {
	pub fn from_disconnected(
		disconnected: Disconnected,
		occupied_indices: OccupiedIndices,
	) -> Self {
		Self {
			occupied_indices,
			data: disconnected.data,
			name: disconnected.name,
		}
	}
}

// Accessors.
impl Limbo {
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
		CameraStatus::Limbo
	}
}

impl AsRef<OccupiedIndices> for Limbo {
	fn as_ref(&self) -> &OccupiedIndices {
		&self.occupied_indices
	}
}

impl AsMut<OccupiedIndices> for Limbo {
	fn as_mut(&mut self) -> &mut OccupiedIndices {
		&mut self.occupied_indices
	}
}

impl Deref for Limbo {
	type Target = PerpetualInfo;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl DerefMut for Limbo {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

impl From<Active> for Limbo {
	fn from(active: Active) -> Self {
		Self {
			occupied_indices: active.occupied_indices,
			data: active.data,
			name: active.name,
		}
	}
}

impl IndexOccupier for Limbo {
}
