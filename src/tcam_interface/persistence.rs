use std::{
	collections::HashMap,
	ops::{Deref, DerefMut},
};

use super::{Camera, Identifier, Metadata, Status};

/// The internal mapping type.
type __Map = HashMap<Identifier, Metadata>;

/// Persistence data for camera devices.
///
/// This type holds details about all currently and previously connected camera devices.
pub(crate) struct Persistence {
	inner: __Map,
}

impl Deref for Persistence {
	type Target = __Map;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Persistence {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl Persistence {
	/// Look for and add new cameras.
	///
	/// Returns a list of information about newly added devices.
	///
	/// The return type should be made into an explicit type.
	pub(crate) fn poll_for_new_cameras(&mut self) -> HashMap<Identifier, (String, Status)> {
		let all_devices = Camera::query_for_available_cameras();

		all_devices
			.into_iter()
			.filter_map(|info| {
				let index = info.index().clone().as_index().unwrap();
				let is_meta = info.is_metadata();
				// Create identifier.
				let identifier = Identifier::from(info);

				// TODO: Save to log.

				// Contrast against Persistence.
				// Is the entry occupied?
				let metadata = self.inner.entry(identifier.clone()).or_insert_with(|| {
					// TODO: Save to log.
					// Create new metadata struct.
					// Insert metadata into Persistence.
					Metadata::default()
				});

				// Does Metadata contain the index?
				// Add index to Metadata.
				if metadata.try_insert_index(index) {
					return None;
				}

				if is_meta {
					// TODO: Save to log.
					return None;
				}

				// TODO: Initialise the camera.
				let cam_res = Camera::new(index);

				match cam_res {
					Err(_why) => {
						// TODO: Save to logs.
						metadata.status = Status::Invalid;
						return None;
					},
					Ok(cam) => {
						// TODO: Save to logs.
						metadata.status = Status::Active;
						metadata.add_camera(cam);
					},
				};

				Some((identifier, (metadata.name.clone(), metadata.status)))
			})
			.collect::<HashMap<_, _>>()
	}
}
