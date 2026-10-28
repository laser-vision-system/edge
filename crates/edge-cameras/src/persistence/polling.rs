//! Polling logic for the [`Persistence`] struct.
//!
//! This is a separate file to avoid bloating the `persistence.rs` file.
//!
//! Potential inputs:
//! - Camera device.
//! - Camera metadata device.
//! - Invalid device.
//!
//! Potential variants:
//! - Unfaimilar
//! - Familiar, invalid
//! - Familiar, valid: broken
//! - Familiar, valid: disconnected
//! - Familiar, valid: active
//!
//! If this is an unfamiliar device:
//! - Try initialise it.
//! - If successful, add it as an initialised entry.
//! - If unsuccessful, add it as an invalid entry.
//!
//! If this is a familiar device and invalid:
//! - Ignore it.
//!
//! If this is a familiar device and broken:
//! - Ignore it.
//!
//! If this is a familiar device and active:
//! - Make sure all indexes are accounted for.
//!
//! If this is a familiar device and disconnected:
//! - Reconnect it.

use std::{
	collections::{
		HashMap,
		HashSet,
	},
	fmt,
	hash::RandomState,
};

use api::{
	edge::NewCameraInfo,
	shared::{
		CameraId,
		CameraStatus,
	},
};
use edge_core::logger::logger;
use nokhwa::utils::{
	CameraIndex,
	CameraInfo,
};

use super::{
	KeyType,
	Persistence,
	ValType,
};
use crate::{
	camera_device::query_for_available_cameras,
	identifier::Identifier,
};

/// Representative data for a device index.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct AccessPoint {
	/// Use the full [`nokhwa`] index type so that any error from an incorrect
	/// index type are handled when creating the camera.
	pub index: CameraIndex,
	/// Whether v4l2 reports this index as belonging to a metadata device.
	pub is_metadata: bool,
}

impl fmt::Display for AccessPoint {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "{}", self.index)?;
		if self.is_metadata {
			write!(f, " (metadata device)")?;
		}
		Ok(())
	}
}

type IntermediaryMap = HashMap<Identifier, HashSet<AccessPoint>>;

/// Intermediary type to map a number of [`AccessPoint`]s to a single
/// [`Identifier`].
#[derive(Clone, Debug)]
pub struct DeviceConnection {
	identifier: Identifier,
	points: HashSet<AccessPoint>,
}

impl fmt::Display for DeviceConnection {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "{}", self.identifier)?;
		for point in &self.points {
			write!(f, "\n\t{point}")?;
		}
		Ok(())
	}
}

impl From<(Identifier, HashSet<AccessPoint>)> for DeviceConnection {
	fn from(value: (Identifier, HashSet<AccessPoint>)) -> Self {
		Self {
			identifier: value.0,
			points: value.1,
		}
	}
}

/// Details about a change to the current [`Persistence`] map.
pub enum ConnectionDetail {
	/// A new device connection.
	New(NewCameraInfo),
	/// An update to an existing device connection.
	Update(CameraStatus),
}

fn convert_to_intermediaries(devices: Vec<CameraInfo>) -> IntermediaryMap {
	let mut map =
		HashMap::with_capacity_and_hasher(devices.len(), RandomState::new());

	for device in devices {
		let index = device.index().to_owned();
		let is_metadata = device.is_metadata();
		let accesspoint = AccessPoint {
			index,
			is_metadata,
		};
		let identifier = KeyType::from(device);
		let entry: &mut HashSet<_> = map.entry(identifier).or_default();
		entry.insert(accesspoint);
	}

	map
}

// Polling logic.
impl Persistence {
	/// Poll all devices to instantiate newly connected cameras.
	///
	/// Returns a list of information about newly added cameras.
	pub fn poll_for_new_cameras(
		&mut self
	) -> HashMap<CameraId, ConnectionDetail> {
		let all_devices = query_for_available_cameras();
		let inter = convert_to_intermediaries(all_devices);
		self.get_new_from_inter(inter)
	}

	fn get_new_from_inter(
		&mut self,
		inter: IntermediaryMap,
	) -> HashMap<CameraId, ConnectionDetail> {
		inter
			.into_iter()
			// TODO: Filter for disconnections here.
			.filter_map(
				|(ident, points)| -> Option<(CameraId, ConnectionDetail)> {
					self.on_each_connection(&DeviceConnection::from((
						ident, points,
					)))
				},
			)
			.collect::<HashMap<_, _>>()

		// Cache a NewCameraInfo of each entry beforehand.
		// Make a new batch of NewCameraInfos.
		// Contrast this to use for register to server.
	}

	/// Attempt to create a new device from a device info entry.
	fn on_each_connection(
		&mut self,
		connection: &DeviceConnection,
	) -> Option<(CameraId, ConnectionDetail)> {
		let mut before = CameraStatus::Invalid;
		let mut update = None;
		let mut new = false;

		let entry = self
			.inner
			.entry(connection.identifier.clone())
			.and_modify(|entry| {
				before = entry.get_status();
				update = entry.update(&connection.points)
			})
			.or_insert_with(|| {
				logger().log_info(&format!(
					"New identifier added: {}",
					connection.identifier
				));
				new = true;
				ValType::new(
					&connection.points,
					connection.identifier.as_name().to_string(),
				)
			});

		if new {
			match entry.get_active() {
				Some(active) => {
					let id = connection.identifier.as_ref();
					let name = active.get_name().to_string();
					let info = NewCameraInfo::new(name, CameraStatus::Active);
					let new = ConnectionDetail::New(info);
					Some((*id, new))
				},
				None => None,
			}
		} else {
			match update {
				Some(after) => {
					(before != after).then(|| {
						let id = connection.identifier.as_ref();
						let status = after;
						let update = ConnectionDetail::Update(status);
						(*id, update)
					})
				},
				None => None,
			}
		}
	}
}
