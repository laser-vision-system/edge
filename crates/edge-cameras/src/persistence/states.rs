//! The many different states that a device may present as.

pub mod active;
pub mod broken;
pub mod disconnected;
pub mod invalid;
pub mod limbo;

use api::shared::CameraStatus;
use nokhwa::utils::CameraIndex;

pub use self::{
	active::Active,
	broken::Broken,
	disconnected::Disconnected,
	invalid::Invalid,
	limbo::Limbo,
};
use super::OccupiedIndices;
use crate::camera_device::CameraLike;

/// Named type for clarity.
pub type InsertWasSuccessful = bool;

/// Additional methods for types that represent occupied indices.
pub trait IndexOccupier:
	AsRef<OccupiedIndices> + AsMut<OccupiedIndices>
{
	/// Check whether an index is present.
	fn indices_has(
		&self,
		index: CameraIndex,
	) -> bool {
		self.as_ref().contains(&index)
	}

	/// Attempt to add an index that a camera is determined to be occupying.
	///
	/// Returns whether the value was newly inserted.
	fn try_insert_index(
		&mut self,
		index: CameraIndex,
	) -> InsertWasSuccessful {
		self.as_mut().insert(index)
	}
}

/// The current status of a known connection.
///
/// New connections must become either [`Active`] or [`Invalid`].
///
/// Acceptable switches are:
/// - [`Active`] -> [`Broken`]
/// - [`Active`] -> [`Disconnected`]
/// - [`Active`] -> [`Limbo`]
///
/// - [`Broken`] -> [`Disconnected`]
///
/// - [`Disconnected`] -> [`Active`]
/// - [`Disconnected`] -> [`Limbo`]
///
/// - [`Limbo`] -> [`Active`]
/// - [`Limbo`] -> [`Broken`]
/// - [`Limbo`] -> [`Disconnected`]
///
/// [`Invalid`] is locked into that state.
pub enum ConnectionStatus {
	/// A connected and functioning camera device.
	Active(Active),
	/// A connected and non-functional camera device.
	Broken(Broken),
	/// A camera that is not connected.
	Disconnected(Disconnected),
	/// A device that occupies indices but does not offer camera capabilities.
	Invalid(Invalid),
	/// A camera that has reconnected but is non-functional.
	Limbo(Limbo),
}

// Constructors.
impl ConnectionStatus {
	/// Create a new instance of [`Active`].
	pub fn new_active<T>(
		camera: T,
		occupied_indices: OccupiedIndices,
		name: String,
	) -> Self
	where
		T: CameraLike + 'static,
	{
		let active = Active::new(camera, occupied_indices, name);
		Self::Active(active)
	}

	/// Create a new instance of [`Invalid`].
	pub fn new_invalid(occupied_indices: OccupiedIndices) -> Self {
		let invalid = Invalid::new(occupied_indices);
		Self::Invalid(invalid)
	}
}

impl ConnectionStatus {
	/// Map this type to a [`CameraStatus`].
	pub fn get_status(&self) -> CameraStatus {
		match self {
			ConnectionStatus::Active {
				..
			} => CameraStatus::Active,
			ConnectionStatus::Broken {
				..
			} => CameraStatus::Broken,
			ConnectionStatus::Disconnected {
				..
			} => CameraStatus::Disconnected,
			ConnectionStatus::Invalid {
				..
			} => CameraStatus::Invalid,
			ConnectionStatus::Limbo {
				..
			} => CameraStatus::Limbo,
		}
	}
}

impl From<Active> for ConnectionStatus {
	fn from(active: Active) -> Self {
		Self::Active(active)
	}
}

impl From<Broken> for ConnectionStatus {
	fn from(broken: Broken) -> Self {
		Self::Broken(broken)
	}
}

impl From<Disconnected> for ConnectionStatus {
	fn from(disconnected: Disconnected) -> Self {
		Self::Disconnected(disconnected)
	}
}

impl From<Invalid> for ConnectionStatus {
	fn from(invalid: Invalid) -> Self {
		Self::Invalid(invalid)
	}
}

impl From<Limbo> for ConnectionStatus {
	fn from(limbo: Limbo) -> Self {
		Self::Limbo(limbo)
	}
}
