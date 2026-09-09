use std::collections::HashMap;

use nokhwa::utils::CameraInfo;

use super::{Camera, Identifier, Metadata, NewCameraInfo};

/// The internal mapping type.
type MapType = HashMap<Identifier, Metadata>;

/// Persistence data for camera devices.
///
/// This type holds details about all currently and previously connected camera devices.
#[derive(Default)]
pub struct Persistence {
	inner: MapType,
}

impl Persistence {
	/// Iterate over a list of devices info entries, returning newly instantiated cameras.
	///
	/// Separated out to make testing easier.
	fn get_new_from_all(&mut self, devices: Vec<CameraInfo>) -> HashMap<Identifier, NewCameraInfo> {
		devices
			.into_iter()
			.filter_map(|info: CameraInfo| -> Option<(Identifier, NewCameraInfo)> {
				self.try_create_device(info)
			})
			.collect::<HashMap<Identifier, NewCameraInfo>>()
	}

	/// Attempt to create a new device from a device info entry.
	fn try_create_device(&mut self, info: CameraInfo) -> Option<(Identifier, NewCameraInfo)> {
		// Create identifier.
		let identifier = Identifier::from(info.clone());
		// TODO: Save to log.

		// Contrast against Persistence.
		let metadata = self.contrast(identifier.clone());
		let index = info
			.index()
			.clone()
			.as_index()
			.expect("index should always be a number");
		let is_metadata = info.is_metadata();
		let device = metadata.try_register_device(index, is_metadata)?;
		Some((identifier, device))
	}

	/// Get the [`Metadata`] for a device by its identifier.
	///
	/// Create a new entry if it does not exist.
	fn contrast(&mut self, ident: Identifier) -> &mut Metadata {
		// Is the entry occupied?
		self.inner.entry(ident).or_insert_with(|| {
			// TODO: Save to log.
			// Create new metadata struct.
			// Insert metadata into Persistence.
			Metadata::default()
		})
	}

	pub fn inner(&mut self) -> &mut MapType {
		&mut self.inner
	}

	/// Poll all devices to instantiate newly connected cameras.
	///
	/// Returns a list of information about newly added cameras.
	///
	/// IDEA: The return type could be made into an explicit type.
	pub fn poll_for_new_cameras(&mut self) -> HashMap<Identifier, NewCameraInfo> {
		let all_devices = Camera::query_for_available_cameras();
		self.get_new_from_all(all_devices)
	}

	/// Filter peristence with an arbitrary function.
	pub fn filter_by(&mut self, filter: Box<dyn Fn(&Metadata) -> bool>) -> Vec<&mut Metadata> {
		self.inner
			.values_mut()
			.filter(|metadata| filter(metadata))
			.collect()
	}
}
