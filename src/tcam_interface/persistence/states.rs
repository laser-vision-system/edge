pub mod active;
pub mod broken;
pub mod disconnected;
pub mod invalid;
pub mod limbo;

use nokhwa::utils::CameraIndex;
use server_edge_datatypes::shared::CameraStatus;

use crate::tcam_interface::CameraDevice;

use super::OccupiedIndices;

pub use active::Active;
pub use broken::Broken;
pub use disconnected::Disconnected;
pub use invalid::Invalid;
pub use limbo::Limbo;

/// Named type for clarity.
pub type InsertWasSuccessful = bool;

/// Additional methods for types that represent occupied indices.
pub trait IndexOccupier: AsRef<OccupiedIndices> + AsMut<OccupiedIndices> {
	/// Check whether an index is present.
	fn indices_has(&self, index: CameraIndex) -> bool {
		self.as_ref().contains(&index)
	}

	/// Attempt to add an index that a camera is determined to be occupying.
	///
	/// Returns whether the value was newly inserted.
	fn try_insert_index(&mut self, index: CameraIndex) -> InsertWasSuccessful {
		self.as_mut().insert(index)
	}
}

pub enum ConnectionStatus {
	// HACK: To work around camera not being cloneable.
	Active { active: Option<Active> },
	Broken { broken: Broken },
	Disconnected { disconnected: Disconnected },
	Invalid { invalid: Invalid },
	Limbo { limbo: Limbo },
}

// Constructors.
impl ConnectionStatus {
	pub fn new_active(
		camera: CameraDevice,
		occupied_indices: OccupiedIndices,
		name: String,
	) -> Self {
		let active = Active::new(camera, occupied_indices, name);
		Self::Active {
			active: Some(active),
		}
	}

	pub fn new_invalid(occupied_indices: OccupiedIndices) -> Self {
		let invalid = Invalid::new(occupied_indices);
		Self::Invalid { invalid }
	}
}

impl ConnectionStatus {
	/// Map this type to a [`CameraStatus`].
	pub fn get_status(&self) -> CameraStatus {
		match self {
			ConnectionStatus::Active { .. } => CameraStatus::Active,
			ConnectionStatus::Broken { .. } => CameraStatus::Broken,
			ConnectionStatus::Disconnected { .. } => CameraStatus::Broken,
			ConnectionStatus::Invalid { .. } => CameraStatus::Invalid,
			ConnectionStatus::Limbo { .. } => CameraStatus::Limbo,
		}
	}
}

impl From<Active> for ConnectionStatus {
	fn from(active: Active) -> Self {
		Self::Active {
			active: Some(active),
		}
	}
}

impl From<Broken> for ConnectionStatus {
	fn from(broken: Broken) -> Self {
		Self::Broken { broken }
	}
}

impl From<Disconnected> for ConnectionStatus {
	fn from(disconnected: Disconnected) -> Self {
		Self::Disconnected { disconnected }
	}
}

impl From<Invalid> for ConnectionStatus {
	fn from(invalid: Invalid) -> Self {
		Self::Invalid { invalid }
	}
}

impl From<Limbo> for ConnectionStatus {
	fn from(limbo: Limbo) -> Self {
		Self::Limbo { limbo }
	}
}
