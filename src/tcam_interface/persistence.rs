mod perpetual_info;
mod persist_entry;
pub mod polling;
mod states;

use std::{
	borrow::{Borrow, BorrowMut},
	collections::{HashMap, HashSet},
	hash::RandomState,
};

use image::RgbImage;
use nokhwa::utils::CameraIndex;
use server_edge_datatypes::shared::CameraId;

use crate::{error::EdgeError, logger::logger, vision_processor::BatchItem};

use self::states::Active;
pub use self::{perpetual_info::PerpetualInfo, persist_entry::PersistEntry};

use super::{Identifier, WhyNo};

pub type ErrAccType = u16;

// Internal mapping types.
type KeyType = Identifier;
type ValType = PersistEntry;
type MapType = HashMap<KeyType, ValType>;
type OccupiedIndices = HashSet<CameraIndex>;

/// Persistence data for camera devices.
///
/// This type holds details about all currently and previously connected camera devices.
pub struct Persistence {
	inner: MapType,
}

// Access methods.
impl Persistence {
	/// Filter peristence with an arbitrary function.
	pub fn filter_by(&mut self, filter: Box<dyn Fn(&PersistEntry) -> bool>) -> Vec<&PersistEntry> {
		self.inner.values().filter(|entry| filter(entry)).collect()
	}

	/// Filter peristence with an arbitrary function.
	pub fn filter_by_mut(
		&mut self,
		filter: Box<dyn Fn(&PersistEntry) -> bool>,
	) -> Vec<&mut PersistEntry> {
		self.inner
			.values_mut()
			.filter(|entry| filter(entry))
			.collect()
	}

	pub fn get(&self, ident: CameraId) -> Option<&PersistEntry> {
		self.inner.get(&ident)
	}

	pub fn get_mut(&mut self, ident: CameraId) -> Option<&mut PersistEntry> {
		self.inner.get_mut(&ident)
	}
}

impl Persistence {
	pub fn run_on_active(
		&mut self,
		camera_err_increment: u16,
		camera_err_threshold: u16,
		camera_err_decrement: u16,
	) -> Vec<std::result::Result<BatchItem<'_>, (EdgeError, WhyNo)>> {
		let batch: Vec<_> = self
			.inner
			.iter_mut()
			.filter_map(|(ident, entry)| {
				if let Some(active) = entry.get_active_mut() {
					Some(Self::on_active_camera(
						CameraId::from(ident.clone()),
						active,
						camera_err_increment,
						camera_err_threshold,
						camera_err_decrement,
					))
				} else {
					None
				}
			})
			.collect();

		batch
	}

	fn on_active_camera(
		id: CameraId,
		entry: &mut Active,
		camera_err_increment: u16,
		camera_err_threshold: u16,
		camera_err_decrement: u16,
	) -> std::result::Result<BatchItem<'_>, (EdgeError, WhyNo)> {
		let image = match Self::imahe_glgo(
			id,
			entry,
			camera_err_increment,
			camera_err_threshold,
			camera_err_decrement,
		) {
			Ok(image) => image,
			Err(why) => return Err(why),
		};

		logger().lock().unwrap().log_info("YAY!!!!");

		logger().lock().unwrap().log_warn(&format!(
			"Camera at id: {} has {} errors",
			id,
			entry.err_accumulation()
		));

		let batch = entry.make_batch_item(image, id);

		Ok(batch)
	}

	fn imahe_glgo(
		id: CameraId,
		entry: &mut Active,
		camera_err_increment: u16,
		camera_err_threshold: u16,
		camera_err_decrement: u16,
	) -> std::result::Result<RgbImage, (EdgeError, WhyNo)> {
		match entry.get_image(camera_err_decrement) {
			Ok(image) => Ok(image),
			Err(why) => Err((
				why,
				Self::on_error(id, entry, camera_err_increment, camera_err_threshold),
			)),
		}
	}

	fn on_error(
		id: CameraId,
		entry: &mut Active,
		camera_err_increment: u16,
		camera_err_threshold: u16,
	) -> WhyNo {
		entry.add_error(camera_err_increment);

		if entry.err_accumulation() < camera_err_threshold {
			//entry.status = CameraStatus::Disconnected;
			logger().lock().unwrap().log_warn("disconnetcted");
			return WhyNo::Disconnected(id);
		} else {
			//entry.status = CameraStatus::Broken;
			logger().lock().unwrap().log_warn("bokrne");
			return WhyNo::Broken(id);
		}
	}
}

impl AsRef<MapType> for Persistence {
	fn as_ref(&self) -> &MapType {
		&self.inner
	}
}

impl AsMut<MapType> for Persistence {
	fn as_mut(&mut self) -> &mut MapType {
		&mut self.inner
	}
}

impl Borrow<MapType> for Persistence {
	fn borrow(&self) -> &MapType {
		&self.inner
	}
}

impl BorrowMut<MapType> for Persistence {
	fn borrow_mut(&mut self) -> &mut MapType {
		&mut self.inner
	}
}

impl Default for Persistence {
	fn default() -> Self {
		Self {
			inner: HashMap::with_hasher(RandomState::new()),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::tcam_interface::identifier::*;

	use super::*;

	#[test]
	fn matching_entries() {
		let mut persist_map = Persistence::default();
		let points = HashSet::new();

		let first_key = Identifier::from(IdentMetaInfo::new("name", "description", "misc"));
		let first_entry = PersistEntry::new(&points);
		persist_map.inner.insert(first_key, first_entry);

		let second_key = Identifier::from(IdentMetaInfo::new("name", "description", "misc"));
		let second_entry = PersistEntry::new(&points);
		persist_map.inner.insert(second_key, second_entry);

		assert_eq!(persist_map.inner.len(), 1)
	}
}
