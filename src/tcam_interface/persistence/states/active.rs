use std::{
	borrow::Cow,
	ops::{Deref, DerefMut},
};

use image::RgbImage;
use server_edge_datatypes::shared::{CameraId, CameraStatus};

use crate::{
	error::Result,
	logger::logger,
	tcam_interface::{
		CameraDevice, PerpetualInfo,
		persistence::{
			ErrAccType, OccupiedIndices,
			polling::{AccessPoint, DeviceConnection},
		},
	},
	vision_processor::BatchItem,
};

use super::{Disconnected, IndexOccupier, Limbo};

/// A connection in an active state.
///
/// It represents a usable connection in normal operation.
///
/// It can be created from:
/// - A new connection
/// - Disconnected
/// - Limbo
///
/// It can change state to:
/// - Broken
/// - Disconnected
/// - Limbo
pub struct Active {
	pub occupied_indices: OccupiedIndices,
	pub data: PerpetualInfo,
	/// A user configurable name for this camera.
	///
	/// It is not required to be unique, but is preferrable.
	pub name: String,
	pub camera: CameraDevice,
}

// Constructors.
impl Active {
	pub fn new(camera: CameraDevice, occupied_indices: OccupiedIndices, name: String) -> Self {
		Self {
			occupied_indices,
			data: PerpetualInfo::default(),
			name,
			camera,
		}
	}

	pub fn from_disconnected(
		disconnected: Disconnected,
		camera: CameraDevice,
		occupied_indices: OccupiedIndices,
	) -> Self {
		Self {
			occupied_indices,
			data: disconnected.data,
			name: disconnected.name,
			camera,
		}
	}

	pub fn from_limbo(limbo: Limbo, camera: CameraDevice) -> Self {
		Self {
			occupied_indices: limbo.occupied_indices,
			data: limbo.data,
			name: limbo.name,
			camera,
		}
	}
}

// Accessors.
impl Active {
	/// Get the name of this device.
	pub fn get_name(&self) -> Cow<'_, str> {
		Cow::Borrowed(&self.name)
	}

	/// Update the name of this device.
	pub fn set_name(&mut self, name: String) {
		self.name = name
	}

	/// Get a reference to the current camera.
	pub fn get_camera(&self) -> &CameraDevice {
		&self.camera
	}

	/// Get a mutable reference to the current camera.
	pub fn get_camera_mut(&mut self) -> &mut CameraDevice {
		&mut self.camera
	}

	/// Map this type to a [`CameraStatus`].
	pub fn get_status(&self) -> CameraStatus {
		CameraStatus::Active
	}
}

// Operational.
impl Active {
	pub fn get_image(&mut self, camera_err_decrement: ErrAccType) -> Result<RgbImage> {
		let frame = match self.get_camera_mut().get_frame() {
			Ok(frame) => frame,
			Err(why) => {
				logger().lock().unwrap().log_error("", &why);
				return Err(why);
			},
		};

		logger()
			.lock()
			.unwrap()
			.log_info("Successfully read frame from camera");
		self.data.reduce_error(camera_err_decrement);

		let image = match self.get_camera_mut().decode_image(frame) {
			Ok(image) => image,
			Err(why) => {
				// TODO: If out of memory, send log and panic.
				logger().lock().unwrap().log_error("", &why);
				return Err(why);
			},
		};

		Ok(image)
	}

	pub fn make_batch_item(&mut self, image: RgbImage, id: CameraId) -> BatchItem<'_> {
		self.data.create_batch_item(image, id)
	}
}

impl AsRef<OccupiedIndices> for Active {
	fn as_ref(&self) -> &OccupiedIndices {
		&self.occupied_indices
	}
}

impl AsMut<OccupiedIndices> for Active {
	fn as_mut(&mut self) -> &mut OccupiedIndices {
		&mut self.occupied_indices
	}
}

impl Deref for Active {
	type Target = PerpetualInfo;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl DerefMut for Active {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

impl IndexOccupier for Active {}
