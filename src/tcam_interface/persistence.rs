mod perpetual_info;

use std::collections::HashMap;

use image::RgbImage;
use nokhwa::utils::CameraInfo;
use server_edge_datatypes::{
	edge::{NewCameraInfo, Settings},
	shared::{CameraId, CameraStatus},
};

use crate::{error::EdgeError, logger::logger, vision_processor::BatchItem};

pub use self::perpetual_info::PerpetualInfo;

use super::{CameraDevice, Identifier, WhyNo};

/// The internal mapping type.
type MapType = HashMap<Identifier, PerpetualInfo>;

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
	fn get_new_from_all(&mut self, devices: Vec<CameraInfo>) -> HashMap<CameraId, NewCameraInfo> {
		devices
			.into_iter()
			.filter_map(|info: CameraInfo| -> Option<(CameraId, NewCameraInfo)> {
				self.try_create_device(info)
			})
			.collect::<HashMap<CameraId, NewCameraInfo>>()
	}

	/// Attempt to create a new device from a device info entry.
	fn try_create_device(&mut self, info: CameraInfo) -> Option<(CameraId, NewCameraInfo)> {
		let identifier = Identifier::from(info.clone());
		// TODO: Save to log.

		let entry = self.contrast(identifier.clone());
		// New devices will be the `Uninitialised` variant here.
		let index = info
			.index()
			.clone()
			.as_index()
			.expect("index should always be a number");
		let is_metadata = info.is_metadata();
		let device = entry.try_register_device(index, is_metadata)?;
		let hashed_id: CameraId = identifier.calculate_hash();
		Some((hashed_id, device))
	}

	/// Get the [`PerpetualInfo`] for a device by its identifier.
	///
	/// Create a new entry if it does not exist.
	fn contrast(&mut self, ident: Identifier) -> &mut PerpetualInfo {
		self.inner.entry(ident).or_insert_with(|| {
			// TODO: Save to log.
			PerpetualInfo::default()
		})
	}

	pub fn inner(&mut self) -> &mut MapType {
		&mut self.inner
	}

	/// Poll all devices to instantiate newly connected cameras.
	///
	/// Returns a list of information about newly added cameras.
	pub fn poll_for_new_cameras(&mut self) -> HashMap<CameraId, NewCameraInfo> {
		let all_devices = CameraDevice::query_for_available_cameras();
		self.get_new_from_all(all_devices)
	}

	/// Filter peristence with an arbitrary function.
	pub fn filter_by(
		&mut self,
		filter: Box<dyn Fn(&PerpetualInfo) -> bool>,
	) -> Vec<&mut PerpetualInfo> {
		self.inner
			.values_mut()
			.filter(|entry| filter(entry))
			.collect()
	}

	pub fn get_with_id(&mut self, ident: CameraId) -> Option<&mut PerpetualInfo> {
		self.inner
			.iter_mut()
			.filter_map(|(key, val)| if *key == ident { Some(val) } else { None })
			.last()
	}

	pub fn run_on_active(
		&mut self,
		settings: Settings,
	) -> Vec<std::result::Result<BatchItem<'_>, (EdgeError, WhyNo)>> {
		let batch: Vec<_> = self
			.inner
			.iter_mut()
			.filter(|(_, entry)| entry.status() == CameraStatus::Active)
			.map(|(ident, active_entry)| {
				let id = ident.calculate_hash();
				Self::on_active_camera(id, active_entry, settings.clone())
			})
			.collect();

		batch
	}

	fn on_active_camera(
		id: CameraId,
		entry: &mut PerpetualInfo,
		settings: Settings,
	) -> std::result::Result<BatchItem<'_>, (EdgeError, WhyNo)> {
		let image = match Self::imahe_glgo(id, entry, settings) {
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
		entry: &mut PerpetualInfo,
		settings: Settings,
	) -> std::result::Result<RgbImage, (EdgeError, WhyNo)> {
		match entry.get_image(settings.clone()) {
			Ok(image) => Ok(image),
			Err(why) => Err((why, Self::on_error(id, entry, settings))),
		}
	}

	fn on_error(id: CameraId, entry: &mut PerpetualInfo, settings: Settings) -> WhyNo {
		entry.add_error(settings.camera_err_increment);

		if entry.err_accumulation() < settings.camera_err_threshold {
			entry.status = CameraStatus::Disconnected;
			logger().lock().unwrap().log_warn("disconnetcted");
			return WhyNo::Disconnected(id);
		} else {
			entry.status = CameraStatus::Broken;
			logger().lock().unwrap().log_warn("bokrne");
			return WhyNo::Broken(id);
		}
	}
}
