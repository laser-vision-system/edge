pub mod active;
pub mod broken;
pub mod disconnected;
pub mod invalid;
pub mod limbo;

pub use active::Active;
use api::shared::CameraStatus;
pub use broken::Broken;
pub use disconnected::Disconnected;
pub use invalid::Invalid;
pub use limbo::Limbo;
use nokhwa::utils::CameraIndex;

use super::OccupiedIndices;
use crate::tcam_interface::CameraLike;

/// Named type for clarity.
pub type InsertWasSuccessful = bool;

/// Additional methods for types that represent occupied indices.
#[expect(
	dead_code,
	reason = "Currently not used, but provides the option if needed"
)]
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
/// Acceptable switches are:
/// - Active -> Broken
/// - Active -> Disconnected
/// - Active -> Limbo
///
/// - Broken -> Disconnected
///
/// - Disconnected -> Active
/// - Disconnected -> Limbo
///
/// - Limbo -> Active
/// - Limbo -> Broken
/// - Limbo -> Disconnected
///
/// Invalid is locked into that state.
pub enum ConnectionStatus {
	Active { active: Active },
	Broken { broken: Broken },
	Disconnected { disconnected: Disconnected },
	Invalid { invalid: Invalid },
	Limbo { limbo: Limbo },
}

// Constructors.
impl ConnectionStatus {
	pub fn new_active<T>(
		camera: T,
		occupied_indices: OccupiedIndices,
		name: String,
	) -> Self
	where
		T: CameraLike + 'static,
	{
		let active = Active::new(camera, occupied_indices, name);
		Self::Active {
			active,
		}
	}

	pub fn new_invalid(occupied_indices: OccupiedIndices) -> Self {
		let invalid = Invalid::new(occupied_indices);
		Self::Invalid {
			invalid,
		}
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
		Self::Active {
			active,
		}
	}
}

impl From<Broken> for ConnectionStatus {
	fn from(broken: Broken) -> Self {
		Self::Broken {
			broken,
		}
	}
}

impl From<Disconnected> for ConnectionStatus {
	fn from(disconnected: Disconnected) -> Self {
		Self::Disconnected {
			disconnected,
		}
	}
}

impl From<Invalid> for ConnectionStatus {
	fn from(invalid: Invalid) -> Self {
		Self::Invalid {
			invalid,
		}
	}
}

impl From<Limbo> for ConnectionStatus {
	fn from(limbo: Limbo) -> Self {
		Self::Limbo {
			limbo,
		}
	}
}
