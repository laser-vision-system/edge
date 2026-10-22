use std::{
	borrow::Cow,
	ops::{
		Deref,
		DerefMut,
	},
};

use api::shared::{
	CameraId,
	CameraStatus,
};
use edge_core::image::Image;
use edge_vision::batch_item::BatchItem;

use super::{
	Disconnected,
	IndexOccupier,
	Limbo,
};
use crate::{
	camera_device::CameraLike,
	error::{
		EdgeCamerasError,
		EdgeCamerasResult,
	},
	persistence::{
		ErrAccType,
		OccupiedIndices,
		PerpetualInfo,
	},
};

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
	pub camera: Box<dyn CameraLike>,
}

// Constructors.
impl Active {
	pub fn new<T>(
		camera: T,
		occupied_indices: OccupiedIndices,
		name: String,
	) -> Self
	where
		T: CameraLike + 'static,
	{
		Self {
			occupied_indices,
			data: PerpetualInfo::default(),
			name,
			camera: Box::new(camera),
		}
	}

	pub fn from_disconnected<T>(
		disconnected: Disconnected,
		camera: T,
		occupied_indices: OccupiedIndices,
	) -> Self
	where
		T: CameraLike + 'static,
	{
		Self {
			occupied_indices,
			data: disconnected.data,
			name: disconnected.name,
			camera: Box::new(camera),
		}
	}

	pub fn from_limbo<T>(
		limbo: Limbo,
		camera: T,
		mut occupied_indices: OccupiedIndices,
	) -> Self
	where
		T: CameraLike + 'static,
	{
		// The parameter contains more indices than limbo.
		if !occupied_indices.is_subset(&limbo.occupied_indices) {
			occupied_indices.extend(limbo.occupied_indices);
		}
		Self {
			occupied_indices,
			data: limbo.data,
			name: limbo.name,
			camera: Box::new(camera),
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
	pub fn set_name(
		&mut self,
		name: String,
	) {
		self.name = name;
	}

	/// Get a reference to the current camera.
	pub fn get_camera(&self) -> &dyn CameraLike {
		&*self.camera
	}

	/// Get a mutable reference to the current camera.
	pub fn get_camera_mut(&mut self) -> &mut dyn CameraLike {
		&mut *self.camera
	}

	/// Map this type to a [`CameraStatus`].
	pub fn get_status() -> CameraStatus {
		CameraStatus::Active
	}
}

// Operational.
impl Active {
	/// Poll the camera for an image.
	///
	/// # Errors
	///
	/// If the camera hasn't been opened yet.
	pub fn get_image(
		&mut self,
		camera_err_decrement: ErrAccType,
	) -> EdgeCamerasResult<Image> {
		let frame = self.get_camera_mut().get_frame_buffer()?;
		// TODO: Inspect error reason. If out of memory, send log and panic.
		let image = frame.try_decode().map_err(EdgeCamerasError::from)?;
		self.data.reduce_error(camera_err_decrement);
		Ok(image)
	}

	pub fn make_batch_item(
		&mut self,
		image: Image,
		id: CameraId,
	) -> BatchItem<'_> {
		let calibrate = self.data.get_calibrate();
		self.data.set_calibrate(false);
		let currently_viewed = self.data.get_currently_viewed();
		self.data.refresh_vision_state();
		let state = self.data.get_vision_state();
		BatchItem::new(id, state, image, calibrate, currently_viewed)
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

impl IndexOccupier for Active {
}
