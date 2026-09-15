//! Polling logic for the [`Persistence`] struct.

use std::{
	collections::{HashMap, HashSet},
	fmt,
	hash::RandomState,
};

use nokhwa::utils::{CameraIndex, CameraInfo};
use server_edge_datatypes::{edge::NewCameraInfo, shared::CameraId};

use super::{
	super::{CameraDevice, Identifier},
	KeyType, Persistence, ValType,
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
	pub index: CameraIndex,
	pub is_metadata: bool,
}

impl fmt::Display for AccessPoint {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.identifier)?;
		for point in &self.points {
			write!(f, "\n\t{}", point)?;
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

fn convert_to_intermediaries(devices: Vec<CameraInfo>) -> IntermediaryMap {
	let mut map = HashMap::with_capacity_and_hasher(devices.len(), RandomState::new());

	for device in devices.into_iter() {
		let index = device.index().to_owned();
		let is_metadata = device.is_metadata();
		let accesspoint = AccessPoint { index, is_metadata };
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
	pub fn poll_for_new_cameras(&mut self) -> HashMap<CameraId, NewCameraInfo> {
		let all_devices = CameraDevice::query_for_available_cameras();
		let inter = convert_to_intermediaries(all_devices);
		self.get_new_from_inter(inter)
	}

	fn get_new_from_inter(&mut self, inter: IntermediaryMap) -> HashMap<CameraId, NewCameraInfo> {
		inter
			.into_iter()
			// TODO: Filter for disconnections here.
			.filter_map(|(ident, points)| -> Option<(CameraId, NewCameraInfo)> {
				self.on_each_connection(DeviceConnection::from((ident, points)))
			})
			.collect::<HashMap<_, _>>()

		// Cache a NewCameraInfo of each entry beforehand.
		// Make a new batch of NewCameraInfos.
		// Contrast this to use for register to server.
	}

	/// Attempt to create a new device from a device info entry.
	fn on_each_connection(
		&mut self,
		connection: DeviceConnection,
	) -> Option<(CameraId, NewCameraInfo)> {
		//let index = match info.index() {
		//	CameraIndex::Index(index) => index,
		//	CameraIndex::String(url) => {
		//		logger()
		//			.lock()
		//			.unwrap()
		//			.log_info(&format!("non-index camera found: {url}"));
		//		// TODO: Set this entry to invalid.
		//		return None;
		//	},
		//};

		//let entry = self.contrast(identifier.clone());
		//let entry = self.inner.entry(ident.clone()).or_insert_with_key(|ident| {
		//	logger()
		//		.lock()
		//		.unwrap()
		//		.log_info(&format!("New identifier added: {ident}"));
		//	Self::on_new_entry(connection)
		//});

		let entry = self
			.inner
			.entry(connection.identifier.clone())
			.and_modify(|entry| entry.update(&connection.points))
			.or_insert_with(|| ValType::new(&connection.points));

		//Self::on_all_entries(entry, &connection.points);

		None
	}

	//fn on_all_entries(entry: &mut PersistEntry, points: &HashSet<AccessPoint>)
	//-> Option<(CameraId, NewCameraInfo)>
	//{
	//	let device = entry.try_register_device(index, is_metadata)?;
	//
	//	let hashed_id = identifier.as_ref().to_owned();
	//	Some((hashed_id, device))
	//}

	// Get the entry by the key with logging.
	//
	// Create a new entry if it does not exist.
	//fn contrast(&mut self, ident: KeyType) -> &mut ValType {
	//	self.inner.entry(ident.clone()).or_insert_with(|| {
	//		logger()
	//			.lock()
	//			.unwrap()
	//			.log_info(&format!("New identifier added: {ident}"));
	//		ValType::Invalid
	//	})
	//}
}
