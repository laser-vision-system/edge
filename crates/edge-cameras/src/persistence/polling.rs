//! Polling logic for the [`Persistence`] struct.

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

// Potential inputs:
// - Camera device.
// - Camera metadata device.
// - Invalid device.

// Potential variants:
// - Unfaimilar
// - Familiar, invalid
// - Familiar, valid: broken
// - Familiar, valid: disconnected
// - Familiar, valid: active

// If this is an unfamiliar device:
// - Try initialise it.
// - If successful, add it as an initialised entry.
// - If unsuccessful, add it as an invalid entry.

// If this is a familiar device and invalid:
// - Ignore it.

// If this is a familiar device and broken:
// - Ignore it.

// If this is a familiar device and active:
// - Make sure all indexes are accounted for.

// If this is a familiar device and disconnected:
// - Reconnect it.

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct AccessPoint {
	/// Use the full [`nokhwa`] index type so that any error from an incorrect
	/// index type are handled when creating the camera.
	pub index: CameraIndex,
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

#[derive(Clone, Debug)]
pub struct DeviceConnection {
	pub identifier: Identifier,
	pub points: HashSet<AccessPoint>,
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

pub enum ConnectionDetail {
	New(NewCameraInfo),
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
			.and_modify(|entry| before = entry.get_status())
			.and_modify(|entry| update = entry.update(&connection.points))
			.or_insert_with(|| {
				logger().log_info(&format!(
					"New identifier added: {}",
					connection.identifier
				));
				new = true;
				ValType::new(&connection.points)
			});

		// Self::on_all_entries(entry, &connection.points);

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

	// fn on_all_entries(entry: &mut PersistEntry, points:
	// &HashSet<AccessPoint>)
	//-> Option<(CameraId, NewCameraInfo)>
	//{
	// 	let device = entry.try_register_device(index, is_metadata)?;
	//
	// 	let hashed_id = identifier.as_ref().to_owned();
	// 	Some((hashed_id, device))
	//}

	// Attempt to add a device to this instance.
	//
	// Returns [`None`] if:
	//
	// - The index is already registered
	// - This is a metadata device
	// - The camera failed to intialise
	// pub fn try_register_device(
	// 	&mut self,
	// 	index: CameraIndex,
	// 	is_metadata: bool,
	//) -> Option<NewCameraInfo> {
	// 	if self.try_insert_index(index) {
	// 		return None;
	// 	}
	//
	// 	// Stop now if this is a metadata device.
	// 	if is_metadata {
	// 		logger().log_warn(&format!(
	// 			"This index ({}) represents a metadata device",
	// 			index
	// 		));
	// 		return None;
	// 	}
	//
	// 	if !self.initialise_camera(index) {
	// 		return None;
	// 	}
	//
	// 	let name = self.get_name().to_owned().to_string();
	// 	let status = self.status();
	// 	let device = NewCameraInfo::new(name, status);
	// 	Some(device)
	//}
}
