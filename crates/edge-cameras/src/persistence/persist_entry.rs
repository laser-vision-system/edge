//! A device entry in the [`Persistence`] map.
//!
//! [`Persistence`]: super::Persistence

use std::{
	borrow::Cow,
	collections::HashSet,
};

use api::shared::{
	CameraId,
	CameraStatus,
};
use edge_core::{
	image::Image,
	logger::logger,
};
use edge_vision::batch_item::BatchItem;
use nokhwa::utils::CameraIndex;

use super::{
	OccupiedIndices,
	polling::AccessPoint,
	states::{
		Active,
		Broken,
		ConnectionStatus,
		Disconnected,
		Invalid,
		Limbo,
	},
};
use crate::{
	camera_device::{
		CameraDevice,
		CameraLike,
	},
	error::EdgeCamerasError,
	identifier::Identifier,
};

/// The reason why an [`Image`] couldn't be retrieved from a camera.
#[derive(Clone, Debug)]
pub enum WhyNo {
	/// No [`Image`] because the camera is broken.
	Broken,
	/// No [`Image`] because the camera is disconnected.
	Disconnected,
}

/// The result from trying to retrieve an [`Image`] from a camera.
pub enum MaybeImage {
	/// An [`Image`] was successfully retrieved.
	Image(Image),
	/// No [`Image`] could be retireved.
	WhyNo(WhyNo),
}

impl From<Image> for MaybeImage {
	fn from(image: Image) -> Self {
		Self::Image(image)
	}
}

impl From<WhyNo> for MaybeImage {
	fn from(whyno: WhyNo) -> Self {
		Self::WhyNo(whyno)
	}
}

/// This is the value associated with an [`Identifier`].
pub struct PersistEntry {
	/// Wrapped in option to allow status switching.
	/// Never access this directly, or the program may panic.
	///
	/// Use [`PersistEntry::as_inner`] instead!
	inner: Option<ConnectionStatus>,
}

/// Constructors.
impl PersistEntry {
	/// Create a new [`PersistEntry`] instance.
	pub fn new(
		points: &HashSet<AccessPoint>,
		name: String,
	) -> Self {
		let occupied_indices: HashSet<CameraIndex> =
			points.iter().map(|point| point.index.clone()).collect();

		let res = Self::try_make_camera_from_points(points);
		if let Some(camera) = res {
			let new =
				ConnectionStatus::new_active(camera, occupied_indices, name);
			logger().log_info("Camera successfully initialised");
			let inner = Some(new);
			Self {
				inner,
			}
		} else {
			let err = EdgeCamerasError::InvalidDeviceConnection;
			logger().log_error(&err);
			let new = ConnectionStatus::new_invalid(occupied_indices);
			let inner = Some(new);
			Self {
				inner,
			}
		}
	}

	fn try_make_camera_from_points(
		points: &HashSet<AccessPoint>
	) -> Option<CameraDevice> {
		for point in points {
			if point.is_metadata {
				continue;
			}
			let maybe_camera = CameraDevice::try_new_open(point.index.clone());
			match maybe_camera {
				Ok(camera) => {
					return Some(camera);
				},
				// TODO: This might be better suited as a warn.
				Err(why) => logger().log_error(&why),
			}
		}
		None
	}
}

/// Variant checkers.
impl PersistEntry {
	/// Check if the current status is [`Active`].
	///
	/// [`Active`]: CameraStatus::Active
	pub fn is_active(&self) -> bool {
		self.get_status() == CameraStatus::Active
	}

	/// Check if the current status is [`Broken`].
	///
	/// [`Broken`]: CameraStatus::Broken
	pub fn is_broken(&self) -> bool {
		self.get_status() == CameraStatus::Broken
	}

	/// Check if the current status is [`Disconnected`].
	///
	/// [`Disconnected`]: CameraStatus::Disconnected
	pub fn is_disconnected(&self) -> bool {
		self.get_status() == CameraStatus::Disconnected
	}

	/// Check if the current status is [`Invalid`].
	///
	/// [`Invalid`]: CameraStatus::Invalid
	pub fn is_invalid(&self) -> bool {
		self.get_status() == CameraStatus::Invalid
	}

	/// Check if the current status is [`Limbo`].
	///
	/// [`Limbo`]: CameraStatus::Limbo
	pub fn is_limbo(&self) -> bool {
		self.get_status() == CameraStatus::Limbo
	}
}

/// Inner operations.
#[expect(clippy::expect_used, reason = "Tightly controlled inner mutability")]
impl PersistEntry {
	fn as_inner(&self) -> &ConnectionStatus {
		self.inner.as_ref().expect(
			"the inner value should only be None for brief switching operations",
		)
	}

	fn as_inner_mut(&mut self) -> &mut ConnectionStatus {
		self.inner.as_mut().expect(
			"the inner value should only be None for brief switching operations",
		)
	}

	/// Remove and return the inner value for use in conversion.
	///
	/// This is the only case where the inner value will become [`None`].
	/// Replace it with a new value ASAP.
	fn take_inner(&mut self) -> ConnectionStatus {
		self.inner.take().expect(
			"the inner value should only be None for brief switching operations",
		)
	}
}

/// Switching operations.
impl PersistEntry {
	/// Change from [`Active`] to [`Broken`].
	pub fn active_to_broken(&mut self) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Active(active) => {
				ConnectionStatus::from(Broken::from(active))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	/// Change from [`Active`] to [`Disconnected`].
	pub fn active_to_disconnected(&mut self) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Active(active) => {
				ConnectionStatus::from(Disconnected::from(active))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	/// Change from [`Active`] to [`Limbo`].
	pub fn active_to_limbo(&mut self) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Active(active) => {
				ConnectionStatus::from(Limbo::from(active))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	/// Change from [`Broken`] to [`Disconnected`].
	pub fn broken_to_disconnected(&mut self) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Broken(broken) => {
				ConnectionStatus::from(Disconnected::from(broken))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	/// Change from [`Disconnected`] to [`Active`].
	pub fn disconnected_to_active<T>(
		&mut self,
		camera: T,
		occupied_indices: OccupiedIndices,
	) where
		T: CameraLike + 'static,
	{
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Disconnected(disconnected) => {
				// TODO
				ConnectionStatus::from(Active::from_disconnected(
					disconnected,
					camera,
					occupied_indices,
				))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	/// Change from [`Disconnected`] to [`Limbo`].
	pub fn disconnected_to_limbo(
		&mut self,
		occupied_indices: OccupiedIndices,
	) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Disconnected(disconnected) => {
				ConnectionStatus::from(Limbo::from_disconnected(
					disconnected,
					occupied_indices,
				))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	/// Change from [`Limbo`] to [`Active`].
	pub fn limbo_to_active<T>(
		&mut self,
		camera: T,
		occupied_indices: OccupiedIndices,
	) where
		T: CameraLike + 'static,
	{
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Limbo(limbo) => {
				// TODO
				ConnectionStatus::from(Active::from_limbo(
					limbo,
					camera,
					occupied_indices,
				))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	/// Change from [`Limbo`] to [`Broken`].
	pub fn limbo_to_broken(&mut self) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Limbo(limbo) => {
				ConnectionStatus::from(Broken::from(limbo))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	/// Change from [`Limbo`] to [`Disconnected`].
	pub fn limbo_to_disconnected(&mut self) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Limbo(limbo) => {
				ConnectionStatus::from(Disconnected::from(limbo))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}
}

/// Accessors.
impl PersistEntry {
	/// Try to access this entry as [`Active`].
	pub fn get_active(&self) -> Option<&Active> {
		match self.as_inner() {
			ConnectionStatus::Active(active) => Some(active),
			_ => None,
		}
	}

	/// Try to mutably access this entry as [`Active`].
	pub fn get_active_mut(&mut self) -> Option<&mut Active> {
		match self.as_inner_mut() {
			ConnectionStatus::Active(active) => Some(active),
			_ => None,
		}
	}

	/// Try to access this entry as [`Broken`].
	pub fn get_broken(&self) -> Option<&Broken> {
		match self.as_inner() {
			ConnectionStatus::Broken(broken) => Some(broken),
			_ => None,
		}
	}

	/// Try to mutably access this entry as [`Broken`].
	pub fn get_broken_mut(&mut self) -> Option<&mut Broken> {
		match self.as_inner_mut() {
			ConnectionStatus::Broken(broken) => Some(broken),
			_ => None,
		}
	}

	/// Try to access this entry as [`Disconnected`].
	pub fn get_disconnected(&self) -> Option<&Disconnected> {
		match self.as_inner() {
			ConnectionStatus::Disconnected(disconnected) => Some(disconnected),
			_ => None,
		}
	}

	/// Try to mutably access this entry as [`Disconnected`].
	pub fn get_disconnected_mut(&mut self) -> Option<&mut Disconnected> {
		match self.as_inner_mut() {
			ConnectionStatus::Disconnected(disconnected) => Some(disconnected),
			_ => None,
		}
	}

	/// Try to access this entry as [`Invalid`].
	pub fn get_invalid(&self) -> Option<&Invalid> {
		match self.as_inner() {
			ConnectionStatus::Invalid(invalid) => Some(invalid),
			_ => None,
		}
	}

	/// Try to mutably access this entry as [`Invalid`].
	pub fn get_invalid_mut(&mut self) -> Option<&mut Invalid> {
		match self.as_inner_mut() {
			ConnectionStatus::Invalid(invalid) => Some(invalid),
			_ => None,
		}
	}

	/// Try to access this entry as [`Limbo`].
	pub fn get_limbo(&self) -> Option<&Limbo> {
		match self.as_inner() {
			ConnectionStatus::Limbo(limbo) => Some(limbo),
			_ => None,
		}
	}

	/// Try to mutably access this entry as [`Limbo`].
	pub fn get_limbo_mut(&mut self) -> Option<&mut Limbo> {
		match self.as_inner_mut() {
			ConnectionStatus::Limbo(limbo) => Some(limbo),
			_ => None,
		}
	}

	/// Map this type to a [`CameraStatus`].
	pub fn get_status(&self) -> CameraStatus {
		self.as_inner().get_status()
	}

	/// Get the name, retuning [`None`] if it is [`Invalid`].
	pub fn try_get_name(&self) -> Option<Cow<'_, str>> {
		match self.as_inner() {
			ConnectionStatus::Active(active) => Some(active.get_name()),
			ConnectionStatus::Broken(broken) => Some(broken.get_name()),
			ConnectionStatus::Disconnected(disconnected) => {
				Some(disconnected.get_name())
			},
			ConnectionStatus::Limbo(limbo) => Some(limbo.get_name()),
			ConnectionStatus::Invalid(_) => None,
		}
	}
}

/// Setters.
impl PersistEntry {
	/// Change the name for this entry.
	pub fn set_name(
		&mut self,
		name: String,
	) {
		match self.as_inner_mut() {
			ConnectionStatus::Active(active) => active.set_name(name),
			ConnectionStatus::Broken(broken) => broken.set_name(name),
			ConnectionStatus::Disconnected(disconnected) => {
				disconnected.set_name(name);
			},
			ConnectionStatus::Limbo(limbo) => limbo.set_name(name),
			ConnectionStatus::Invalid(_) => {
				logger().log_warn(
					"Attempted to change name of an invalid connection",
				);
			},
		}
	}

	/// Change whether this entry is current being viewed.
	pub fn set_currently_viewed(
		&mut self,
		state: bool,
	) {
		match self.as_inner_mut() {
			ConnectionStatus::Active(active) => {
				active.set_currently_viewed(state);
			},

			// TODO: Technically, these two should be a logic error.
			ConnectionStatus::Disconnected(disconnected) => {
				disconnected.set_currently_viewed(state);
			},
			ConnectionStatus::Limbo(limbo) => {
				limbo.set_currently_viewed(state);
			},

			ConnectionStatus::Broken(_) | ConnectionStatus::Invalid(_) => {
				logger().log_warn(
					"Attempted to change currently_viewed of an invalid connection",
				);
			},
		}
	}

	/// Ask for this entry to calibrate the setpoint when next processed.
	pub fn request_calibration(&mut self) {
		match self.as_inner_mut() {
			ConnectionStatus::Active(active) => active.set_calibrate(true),

			// TODO: Technically, these two should be a logic error.
			ConnectionStatus::Disconnected(disconnected) => {
				disconnected.set_calibrate(true);
			},
			ConnectionStatus::Limbo(limbo) => limbo.set_calibrate(true),

			ConnectionStatus::Broken(_) | ConnectionStatus::Invalid(_) => {
				logger().log_warn(
					"Attempted to set calibrate of an invalid connection",
				);
			},
		}
	}
}

/// Operations.
impl PersistEntry {
	/// Attempt to retrieve an image from this entry.
	///
	/// Will return [`None`] if the current [`ConnectionStatus`] is not
	/// [`Active`].
	pub fn try_get_image(
		&mut self,
		camera_err_increment: u16,
		camera_err_threshold: u16,
		camera_err_decrement: u16,
	) -> Option<MaybeImage> {
		let active = self.get_active_mut()?;

		let maybeimage = active.get_image(camera_err_decrement).map_or_else(
			|why| {
				logger().log_error(&why);
				active.add_error(camera_err_increment);

				if active.err_accumulation() < camera_err_threshold {
					logger().log_warn("disconnetcted");
					MaybeImage::WhyNo(WhyNo::Disconnected)
				} else {
					logger().log_warn("bokrne");
					MaybeImage::WhyNo(WhyNo::Broken)
				}
			},
			MaybeImage::from,
		);

		Some(maybeimage)
	}

	/// Has to be a separate method, so that the failures can be dropped before
	/// creating the batch item.
	pub fn try_make_batch_item_with(
		&mut self,
		ident: &Identifier,
		name: &str,
		image: Image,
	) -> Option<BatchItem<'_>> {
		let active = self.get_active_mut()?;

		let err_acc = active.err_accumulation();
		if err_acc > 0 {
			logger().log_warn(&format!(
				"Camera at id `{ident}` with name '{name}' has an err accumulation of `{err_acc}`"
			));
		}

		let batch =
			active.make_batch_item(image, CameraId::from(ident.clone()));

		Some(batch)
	}

	/// Compare this entry to the current access points, updating the state as
	/// required.
	pub fn update(
		&mut self,
		points: &HashSet<AccessPoint>,
	) -> Option<CameraStatus> {
		// todo!("First, check if main index is gone. Then update and log extra
		// indices.");
		match self.as_inner_mut() {
			ConnectionStatus::Active(active) => {
				// Check if it needs to go to limbo.
				let occupied_indices: HashSet<_> =
					points.iter().map(|point| &point.index).collect();
				let index = &active.get_camera().get_index();
				(!occupied_indices.contains(index)).then(|| {
					// Camera has disconnected.
					self.active_to_disconnected();
					CameraStatus::Disconnected
				})
			},
			ConnectionStatus::Disconnected(_) => {
				let camera = Self::try_make_camera_from_points(points)?;
				// Succesful reconnection.
				let occupied_indices: HashSet<_> =
					points.iter().map(|point| point.index.clone()).collect();
				self.disconnected_to_active(camera, occupied_indices);
				Some(CameraStatus::Active)
			},
			ConnectionStatus::Limbo(_) => {
				let camera = Self::try_make_camera_from_points(points)?;
				// Succesful reconnection.
				let occupied_indices: HashSet<_> =
					points.iter().map(|point| point.index.clone()).collect();
				self.limbo_to_active(camera, occupied_indices);
				Some(CameraStatus::Active)
			},
			ConnectionStatus::Broken(_) | ConnectionStatus::Invalid(_) => {
				// Do nothing.
				None
			},
		}
	}
}
