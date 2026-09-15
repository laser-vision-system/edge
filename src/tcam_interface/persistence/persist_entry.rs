use std::collections::HashSet;

use nokhwa::utils::CameraIndex;

use crate::{error::EdgeError, logger::logger, tcam_interface::CameraDevice};

use super::{
	polling::AccessPoint,
	states::{Active, Broken, ConnectionStatus, Disconnected, Invalid, Limbo},
};

#[allow(unused_imports, reason = "Needed for docstring")]
use crate::tcam_interface::Identifier;

/// Named type for clarity.
pub type InsertWasSuccessful = bool;
/// Named type for clarity.
type InitialiseWasSuccessful = bool;

/// This is the value associated with an [`Identifier`].
pub struct PersistEntry {
	inner: ConnectionStatus,
}

// Constructors.
impl PersistEntry {
	pub fn new(points: &HashSet<AccessPoint>) -> Self {
		let occupied_indices: HashSet<CameraIndex> =
			points.iter().map(|point| point.index.clone()).collect();

		let res = Self::try_make_camera_from_points(&points);
		match res {
			Some(camera) => {
				let name = camera.get_info().to_string();
				let new = ConnectionStatus::new_active(camera, occupied_indices, name);
				Self { inner: new }
			},
			None => {
				// Use warn because this isn't an explicit error case.
				let err = EdgeError::InvalidDeviceConnection;
				logger()
					.lock()
					.unwrap()
					.log_warn(&format!("Invalid device found: {}", err));
				let new = ConnectionStatus::new_invalid(occupied_indices);
				Self { inner: new }
			},
		}
	}

	fn try_make_camera_from_points(points: &HashSet<AccessPoint>) -> Option<CameraDevice> {
		for point in points {
			if point.is_metadata {
				continue;
			}
			let res = CameraDevice::new(point.index.clone());
			if let Ok(mut camera) = res {
				camera.reset_controls_to_default();
				return Some(camera);
			}
		}
		None
	}
}

// Accessors.
impl PersistEntry {
	pub fn get_active(&self) -> Option<&Active> {
		match self.inner {
			ConnectionStatus::Active { ref active } => active.as_ref(),
			_ => None,
		}
	}

	pub fn get_active_mut(&mut self) -> Option<&mut Active> {
		match self.inner {
			ConnectionStatus::Active { ref mut active } => active.as_mut(),
			_ => None,
		}
	}

	pub fn get_broken(&self) -> Option<&Broken> {
		match self.inner {
			ConnectionStatus::Broken { ref broken } => Some(broken),
			_ => None,
		}
	}

	pub fn get_broken_mut(&mut self) -> Option<&mut Broken> {
		match self.inner {
			ConnectionStatus::Broken { ref mut broken } => Some(broken),
			_ => None,
		}
	}

	pub fn get_disconnected(&self) -> Option<&Disconnected> {
		match self.inner {
			ConnectionStatus::Disconnected { ref disconnected } => Some(disconnected),
			_ => None,
		}
	}

	pub fn get_disconnected_mut(&mut self) -> Option<&mut Disconnected> {
		match self.inner {
			ConnectionStatus::Disconnected {
				ref mut disconnected,
			} => Some(disconnected),
			_ => None,
		}
	}

	pub fn get_invalid(&self) -> Option<&Invalid> {
		match self.inner {
			ConnectionStatus::Invalid { ref invalid } => Some(invalid),
			_ => None,
		}
	}

	pub fn get_invalid_mut(&mut self) -> Option<&mut Invalid> {
		match self.inner {
			ConnectionStatus::Invalid { ref mut invalid } => Some(invalid),
			_ => None,
		}
	}

	pub fn get_limbo(&self) -> Option<&Limbo> {
		match self.inner {
			ConnectionStatus::Limbo { ref limbo } => Some(limbo),
			_ => None,
		}
	}

	pub fn get_limbo_mut(&mut self) -> Option<&mut Limbo> {
		match self.inner {
			ConnectionStatus::Limbo { ref mut limbo } => Some(limbo),
			_ => None,
		}
	}
}

// Operations.
impl PersistEntry {
	pub fn update(&mut self, points: &HashSet<AccessPoint>) {
		//todo!("First, check if main index is gone. Then update and log extra indices.");
		match &mut self.inner {
			ConnectionStatus::Active { active } => {
				// Check if it needs to go to limbo.
				let occupied_indices: HashSet<_> =
					points.iter().map(|point| &point.index).collect();
				let index = &active
					.as_ref()
					.expect("active status should never remain None")
					.get_camera()
					.get_index();
				if !occupied_indices.contains(index) {
					// Camera has disconnected.
					let temp = active
						.take()
						.expect("active camera should never remain None");
					let disconnected = Disconnected::from(temp);
					self.inner = ConnectionStatus::Disconnected { disconnected }
				}
			},
			ConnectionStatus::Broken { broken: _ } => {
				// Do nothing.
			},
			ConnectionStatus::Disconnected { disconnected } => {
				let res = Self::try_make_camera_from_points(points);
				match res {
					Some(camera) => {
						// Succesful reconnection.
						let occupied_indices: HashSet<_> =
							points.iter().map(|point| point.index.clone()).collect();
						let active = Active::from_disconnected(
							disconnected.clone(),
							camera,
							occupied_indices,
						);
						self.inner = ConnectionStatus::Active {
							active: Some(active),
						};
					},
					None => {
						// Unsucessful reconnection.
					},
				}
			},
			ConnectionStatus::Invalid { invalid: _ } => {
				// Do nothing.
			},
			ConnectionStatus::Limbo { limbo } => {
				let res = Self::try_make_camera_from_points(points);
				match res {
					Some(camera) => {
						// Succesful reconnection.
						let active = Active::from_limbo(limbo.clone(), camera);
						self.inner = ConnectionStatus::Active {
							active: Some(active),
						};
					},
					None => {
						// Unsucessful reconnection.
					},
				}
			},
		}
	}

	// Attempt to add a device to this instance.
	//
	// Returns [`None`] if:
	//
	// - The index is already registered
	// - This is a metadata device
	// - The camera failed to intialise
	//pub fn try_register_device(
	//	&mut self,
	//	index: CameraIndex,
	//	is_metadata: bool,
	//) -> Option<NewCameraInfo> {
	//	if self.try_insert_index(index) {
	//		return None;
	//	}

	//	// Stop now if this is a metadata device.
	//	if is_metadata {
	//		logger().lock().unwrap().log_warn(&format!(
	//			"This index ({}) represents a metadata device",
	//			index
	//		));
	//		return None;
	//	}

	//	if !self.initialise_camera(index) {
	//		return None;
	//	}

	//	let name = self.get_name().to_owned().to_string();
	//	let status = self.status();
	//	let device = NewCameraInfo::new(name, status);
	//	Some(device)
	//}
}
