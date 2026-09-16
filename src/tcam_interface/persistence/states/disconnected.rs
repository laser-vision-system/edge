use std::{
	borrow::Cow,
	ops::{Deref, DerefMut},
};

use server_edge_datatypes::shared::CameraStatus;

use crate::tcam_interface::PerpetualInfo;

use super::{Active, Broken, Limbo};

/// A connection in a disconnected state.
///
/// It represents a connection that has been lost, but must retain information
/// for the (assumed) eventual reconnect.
///
/// It can be created from:
/// - Active
/// - Broken
/// - Limbo
///
/// It can change state to:
/// - Active
/// - Limbo
#[derive(Clone, Debug, Default)]
pub struct Disconnected {
	pub data: PerpetualInfo,
	/// A user configurable name for this camera.
	///
	/// It is not required to be unique, but is preferrable.
	pub name: String,
}

// Accessors.
impl Disconnected {
	/// Get the name of this device.
	pub fn get_name(&self) -> Cow<'_, str> {
		Cow::Borrowed(&self.name)
	}

	/// Update the name of this device.
	pub fn set_name(&mut self, name: String) {
		self.name = name
	}

	/// Map this type to a [`CameraStatus`].
	pub fn get_status(&self) -> CameraStatus {
		CameraStatus::Active
	}
}

impl From<Active> for Disconnected {
	fn from(active: Active) -> Self {
		Self {
			data: active.data,
			name: active.name,
		}
	}
}

impl From<Broken> for Disconnected {
	fn from(broken: Broken) -> Self {
		Self {
			data: PerpetualInfo::default(),
			name: broken.name,
		}
	}
}

impl From<Limbo> for Disconnected {
	fn from(limbo: Limbo) -> Self {
		Self {
			data: limbo.data,
			name: limbo.name,
		}
	}
}

impl Deref for Disconnected {
	type Target = PerpetualInfo;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl DerefMut for Disconnected {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}
