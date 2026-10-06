// TODO: Remove all unwraps and then this.
#![expect(clippy::unwrap_used, reason = "WIP development")]
#![expect(clippy::inline_modules, reason = "Simple unit tests")]

mod perpetual_info;
mod persist_entry;
pub mod polling;
mod states;

use std::{
	borrow::{Borrow, BorrowMut},
	collections::{HashMap, HashSet},
	hash::RandomState,
};

use nokhwa::utils::CameraIndex;
use server_edge_datatypes::{
	edge::{
		NewCameraInfo, RegisterCameras, ServerRequests, SetNameCamera,
		Setpoint, StartViewingCamera, StopViewingCamera,
	},
	shared::{CameraId, Configuration, ViewCameraConfig},
};

pub use self::{
	perpetual_info::PerpetualInfo,
	persist_entry::{MaybeImage, PersistEntry, WhyNo},
};
use super::{Identifier, PersistenceError};
use crate::{error::EdgeError, vision_processor::BatchItem};

pub type ErrAccType = u16;

// Internal mapping types.
type KeyType = Identifier;
type ValType = PersistEntry;
type MapType = HashMap<KeyType, ValType>;
type OccupiedIndices = HashSet<CameraIndex>;

pub enum MaybeBatch<'a> {
	Batch(BatchItem<'a>),
	WhyNo(WhyNo),
}

pub struct StatusChange {
	pub id: CameraId,
	pub why: WhyNo,
}

impl StatusChange {
	fn new(id: CameraId, why: WhyNo) -> Self {
		Self { id, why }
	}
}

/// Persistence data for camera devices.
///
/// This type holds details about all currently and previously connected camera
/// devices.
pub struct Persistence {
	inner: MapType,
}

// Access methods.
impl Persistence {
	/// Filter peristence with an arbitrary function.
	pub fn filter_by(
		&mut self,
		filter: &dyn Fn(&PersistEntry) -> bool,
	) -> Vec<&PersistEntry> {
		self.inner.values().filter(|entry| filter(entry)).collect()
	}

	/// Filter peristence with an arbitrary function.
	pub fn filter_by_mut(
		&mut self,
		filter: &dyn Fn(&PersistEntry) -> bool,
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

// Setters.
impl Persistence {
	// TODO: Server should send a single request to change viewing state.
	fn set_currently_viewed(&mut self, id: CameraId, state: bool) {
		if let Some(entry) = self.get_mut(id) {
			entry.set_currently_viewed(state);
		} else {
			EdgeError::from(PersistenceError::UnknownCameraId).log();
		}
	}

	pub fn set_entry_name(&mut self, request: SetNameCamera) {
		if let Some(entry) = self.get_mut(request.camera_id) {
			entry.set_name(request.name);
		} else {
			EdgeError::from(PersistenceError::UnknownCameraId).log();
		}
	}

	#[expect(
		clippy::needless_pass_by_value,
		reason = "The request should not be used after this method call"
	)]
	pub fn start_viewing_camera(&mut self, request: StartViewingCamera) {
		self.set_currently_viewed(request.camera_id, true);
	}

	#[expect(
		clippy::needless_pass_by_value,
		reason = "The request should not be used after this method call"
	)]
	pub fn stop_viewing_camera(&mut self, request: StopViewingCamera) {
		self.set_currently_viewed(request.camera_id, false);
	}

	#[expect(
		clippy::needless_pass_by_value,
		reason = "The request should not be used after this method call"
	)]
	pub fn set_configuration(&mut self, request: Configuration) {
		let Some(entry) = self.get_mut(request.camera_id) else {
			EdgeError::from(PersistenceError::UnknownCameraId).log();
			return;
		};
		let Some(active) = entry.get_active_mut() else {
			//"Not active"
			EdgeError::from(PersistenceError::IncorrectConnectionStatus).log();
			return;
		};
		match serde_json::from_str(&request.configuration) {
			Ok(map) => {
				if let Err(why) = active.get_camera_mut().set_controls(map) {
					// TODO: Error type for "Failed to apply request controls to
					// the camera"
					why.log();
				}
			},
			Err(why) => {
				EdgeError::from(why).log();
			},
		}
	}

	pub fn get_configuration(
		&mut self,
		request: ViewCameraConfig,
	) -> Option<Configuration> {
		let Some(entry) = self.get_mut(request.camera_id) else {
			EdgeError::from(PersistenceError::UnknownCameraId).log();
			return None;
		};
		let Some(active) = entry.get_active_mut() else {
			// TODO: Error type for "Not active"
			EdgeError::from(PersistenceError::IncorrectConnectionStatus).log();
			return None;
		};
		let controls = active.get_camera_mut().get_controls();
		let important_controls = match controls {
			Ok(controls) => controls.stripped(),
			Err(why) => {
				why.log();
				return None;
			},
		};
		let configuration = Configuration::build_response(
			request,
			important_controls.serialise(),
		);
		Some(configuration)
	}

	#[expect(
		clippy::needless_pass_by_value,
		reason = "The request should not be used after this method call"
	)]
	pub fn request_calibration(&mut self, request: Setpoint) {
		if let Some(entry) = self.get_mut(request.camera_id) {
			entry.request_calibration();
		} else {
			EdgeError::from(PersistenceError::UnknownCameraId).log();
		}
	}

	pub fn get_current_cameras(&mut self) -> ServerRequests {
		let camera_list: HashMap<CameraId, NewCameraInfo> = self
			.inner
			.iter()
			.filter_map(|(id, entry)| {
				let name = entry
					.try_get_name()
					.map(std::borrow::Cow::into_owned)
					.unwrap_or(id.to_string());
				let camera_id = id.as_ref();
				let camera_status = NewCameraInfo {
					name,
					status: entry.get_status(),
				};
				Some((*camera_id, camera_status))
			})
			.collect();
		ServerRequests::RegisterCameras(RegisterCameras {
			cameras: camera_list,
		})
	}
}

impl Persistence {
	pub fn poll_active_entries(
		&mut self,
		camera_err_increment: u16,
		camera_err_threshold: u16,
		camera_err_decrement: u16,
	) -> (Vec<BatchItem<'_>>, Vec<StatusChange>) {
		let (maybe_batch_items, maybe_status_changes): (Vec<_>, Vec<_>) = self
			.inner
			.iter_mut()
			.filter_map(|(ident, entry)| {
				let maybe_image = entry.try_get_image(
					camera_err_increment,
					camera_err_threshold,
					camera_err_decrement,
				)?;

				// Non-destructively read the result and adjust entry state.
				if let MaybeImage::WhyNo(whynot) = &maybe_image {
					match whynot {
						WhyNo::Broken => {
							entry.active_to_broken();
						},
						WhyNo::Disconnected => {
							entry.active_to_disconnected();
						},
					}
				}

				match maybe_image {
					MaybeImage::Image(image) => {
						let name = entry
							.try_get_name()
							.expect("Pls not inavlid")
							.into_owned();
						let batch_item = entry
							.try_make_batch_item_with(ident, &name, image)?;
						Some((Some(batch_item), None))
					},
					MaybeImage::WhyNo(whyno) => Some((
						None,
						Some(StatusChange::new(
							CameraId::from(ident.clone()),
							whyno,
						)),
					)),
				}
			})
			.unzip();

		let batch_items = maybe_batch_items.into_iter().flatten().collect();
		let status_changes =
			maybe_status_changes.into_iter().flatten().collect();

		(batch_items, status_changes)
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
	use super::*;
	use crate::tcam_interface::identifier::*;

	#[test]
	fn matching_entries() {
		let mut persist_map = Persistence::default();
		let points = HashSet::new();

		let first_key =
			Identifier::from(IdentMetaInfo::new("name", "description", "misc"));
		let first_entry = PersistEntry::new(&points);
		persist_map.inner.insert(first_key, first_entry);

		let second_key =
			Identifier::from(IdentMetaInfo::new("name", "description", "misc"));
		let second_entry = PersistEntry::new(&points);
		persist_map.inner.insert(second_key, second_entry);

		assert_eq!(persist_map.inner.len(), 1);
	}
}
