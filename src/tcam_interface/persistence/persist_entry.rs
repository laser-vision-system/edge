use std::collections::HashSet;

use image::RgbImage;
use nokhwa::utils::CameraIndex;
use server_edge_datatypes::shared::{CameraId, CameraStatus};

use super::{
	OccupiedIndices,
	polling::AccessPoint,
	states::{Active, Broken, ConnectionStatus, Disconnected, Invalid, Limbo},
};

use crate::{
	error::EdgeError, logger::logger, tcam_interface::CameraDevice,
	tcam_interface::Identifier, vision_processor::BatchItem,
};

#[derive(Clone, Debug)]
pub enum WhyNo {
	Broken,
	Disconnected,
}

pub enum MaybeImage {
	Image(RgbImage),
	WhyNo(WhyNo),
}

impl From<RgbImage> for MaybeImage {
	fn from(image: RgbImage) -> Self {
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
	// Wrapped in option to allow status switching.
	inner: Option<ConnectionStatus>,
}

// Constructors.
impl PersistEntry {
	pub fn new(points: &HashSet<AccessPoint>) -> Self {
		let occupied_indices: HashSet<CameraIndex> =
			points.iter().map(|point| point.index.clone()).collect();

		let res = Self::try_make_camera_from_points(points);
		if let Some(camera) = res {
			let name = camera.get_info().to_string();
			let new =
				ConnectionStatus::new_active(camera, occupied_indices, name);
			Self { inner: Some(new) }
		} else {
			// Use warn because this isn't an explicit error case.
			let err = EdgeError::InvalidDeviceConnection;
			logger()
				.lock()
				.unwrap()
				.log_warn(&format!("Invalid device found: {err}"));
			let new = ConnectionStatus::new_invalid(occupied_indices);
			Self { inner: Some(new) }
		}
	}

	fn try_make_camera_from_points(
		points: &HashSet<AccessPoint>,
	) -> Option<CameraDevice> {
		for point in points {
			if point.is_metadata {
				continue;
			}
			let res = CameraDevice::new(point.index.clone());
			if let Ok(mut camera) = res {
				camera.reset_controls_to_default();
				return Some(camera);
			}
		}
		None
	}
}

// Variant checkers.
impl PersistEntry {
	pub fn is_active(&self) -> bool {
		self.get_status() == CameraStatus::Active
	}

	pub fn is_broken(&self) -> bool {
		self.get_status() == CameraStatus::Broken
	}

	pub fn is_disconnected(&self) -> bool {
		self.get_status() == CameraStatus::Disconnected
	}

	pub fn is_invalid(&self) -> bool {
		self.get_status() == CameraStatus::Invalid
	}

	pub fn is_limbo(&self) -> bool {
		self.get_status() == CameraStatus::Limbo
	}
}

// Inner operations.
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

	pub fn active_to_broken(&mut self) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Active { active } => {
				ConnectionStatus::from(Broken::from(active))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	pub fn active_to_disconnected(&mut self) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Active { active } => {
				ConnectionStatus::from(Disconnected::from(active))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	pub fn active_to_limbo(&mut self) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Active { active } => {
				ConnectionStatus::from(Limbo::from(active))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	pub fn broken_to_disconnected(&mut self) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Broken { broken } => {
				ConnectionStatus::from(Disconnected::from(broken))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	pub fn disconnected_to_active(
		&mut self,
		camera: CameraDevice,
		occupied_indices: OccupiedIndices,
	) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Disconnected { disconnected } => {
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

	pub fn disconnected_to_limbo(&mut self, occupied_indices: OccupiedIndices) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Disconnected { disconnected } => {
				ConnectionStatus::from(Limbo::from_disconnected(
					disconnected,
					occupied_indices,
				))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	pub fn limbo_to_active(
		&mut self,
		camera: CameraDevice,
		occupied_indices: OccupiedIndices,
	) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Limbo { limbo } => {
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

	pub fn limbo_to_broken(&mut self) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Limbo { limbo } => {
				ConnectionStatus::from(Broken::from(limbo))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}

	pub fn limbo_to_disconnected(&mut self) {
		let inner = self.take_inner();
		let new_inner = match inner {
			ConnectionStatus::Limbo { limbo } => {
				ConnectionStatus::from(Disconnected::from(limbo))
			},
			other => other,
		};
		self.inner.replace(new_inner);
	}
}

// Accessors.
impl PersistEntry {
	pub fn get_active(&self) -> Option<&Active> {
		match self.as_inner() {
			ConnectionStatus::Active { active } => Some(active),
			_ => None,
		}
	}

	pub fn get_active_mut(&mut self) -> Option<&mut Active> {
		match self.as_inner_mut() {
			ConnectionStatus::Active { active } => Some(active),
			_ => None,
		}
	}

	pub fn get_broken(&self) -> Option<&Broken> {
		match self.as_inner() {
			ConnectionStatus::Broken { broken } => Some(broken),
			_ => None,
		}
	}

	pub fn get_broken_mut(&mut self) -> Option<&mut Broken> {
		match self.as_inner_mut() {
			ConnectionStatus::Broken { broken } => Some(broken),
			_ => None,
		}
	}

	pub fn get_disconnected(&self) -> Option<&Disconnected> {
		match self.as_inner() {
			ConnectionStatus::Disconnected { disconnected } => {
				Some(disconnected)
			},
			_ => None,
		}
	}

	pub fn get_disconnected_mut(&mut self) -> Option<&mut Disconnected> {
		match self.as_inner_mut() {
			ConnectionStatus::Disconnected { disconnected } => {
				Some(disconnected)
			},
			_ => None,
		}
	}

	pub fn get_invalid(&self) -> Option<&Invalid> {
		match self.as_inner() {
			ConnectionStatus::Invalid { invalid } => Some(invalid),
			_ => None,
		}
	}

	pub fn get_invalid_mut(&mut self) -> Option<&mut Invalid> {
		match self.as_inner_mut() {
			ConnectionStatus::Invalid { invalid } => Some(invalid),
			_ => None,
		}
	}

	pub fn get_limbo(&self) -> Option<&Limbo> {
		match self.as_inner() {
			ConnectionStatus::Limbo { limbo } => Some(limbo),
			_ => None,
		}
	}

	pub fn get_limbo_mut(&mut self) -> Option<&mut Limbo> {
		match self.as_inner_mut() {
			ConnectionStatus::Limbo { limbo } => Some(limbo),
			_ => None,
		}
	}

	/// Map this type to a [`CameraStatus`].
	pub fn get_status(&self) -> CameraStatus {
		self.as_inner().get_status()
	}
}

// Setters.
impl PersistEntry {
	pub fn set_name(&mut self, name: String) {
		match self.as_inner_mut() {
			ConnectionStatus::Active { active } => active.set_name(name),
			ConnectionStatus::Broken { broken } => broken.set_name(name),
			ConnectionStatus::Disconnected { disconnected } => {
				disconnected.set_name(name);
			},
			ConnectionStatus::Limbo { limbo } => limbo.set_name(name),
			ConnectionStatus::Invalid { .. } => logger()
				.lock()
				.unwrap()
				.log_warn("Attempted to change name of an invalid connection"),
		}
	}

	pub fn set_currently_viewed(&mut self, state: bool) {
		match self.as_inner_mut() {
			ConnectionStatus::Active { active } => {
				active.set_currently_viewed(state);
			},

			// TODO: Technically, these two should be a logic error.
			ConnectionStatus::Disconnected { disconnected } => {
				disconnected.set_currently_viewed(state);
			},
			ConnectionStatus::Limbo { limbo } => {
				limbo.set_currently_viewed(state);
			},

			ConnectionStatus::Broken { .. }
			| ConnectionStatus::Invalid { .. } => logger().lock().unwrap().log_warn(
				"Attempted to change currently_viewed of an invalid connection",
			),
		}
	}

	pub fn request_calibration(&mut self) {
		match self.as_inner_mut() {
			ConnectionStatus::Active { active } => active.set_calibrate(true),

			// TODO: Technically, these two should be a logic error.
			ConnectionStatus::Disconnected { disconnected } => {
				disconnected.set_calibrate(true);
			},
			ConnectionStatus::Limbo { limbo } => limbo.set_calibrate(true),

			ConnectionStatus::Broken { .. }
			| ConnectionStatus::Invalid { .. } => logger().lock().unwrap().log_warn(
				"Attempted to set calibrate of an invalid connection",
			),
		}
	}
}

// Operations.
impl PersistEntry {
	pub fn try_get_image(
		&mut self,
		camera_err_increment: u16,
		camera_err_threshold: u16,
		camera_err_decrement: u16,
	) -> Option<MaybeImage> {
		let active = self.get_active_mut()?;

		let maybeimage = active
			.get_image(camera_err_decrement)
			.inspect_err(|why| logger().lock().unwrap().log_error("", why))
			.map_or_else(
				|_| {
					active.add_error(camera_err_increment);

					if active.err_accumulation() < camera_err_threshold {
						logger().lock().unwrap().log_warn("disconnetcted");
						MaybeImage::WhyNo(WhyNo::Disconnected)
					} else {
						logger().lock().unwrap().log_warn("bokrne");
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
		image: RgbImage,
	) -> Option<BatchItem<'_>> {
		let active = self.get_active_mut()?;

		logger().lock().unwrap().log_warn(&format!(
			"Camera at id `{}` has an err accumulation of `{}`",
			ident,
			active.err_accumulation()
		));

		let batch =
			active.make_batch_item(image, CameraId::from(ident.clone()));

		Some(batch)
	}

	pub fn update(
		&mut self,
		points: &HashSet<AccessPoint>,
	) -> Option<CameraStatus> {
		// todo!("First, check if main index is gone. Then update and log extra
		// indices.");
		match self.as_inner_mut() {
			ConnectionStatus::Active { active } => {
				// Check if it needs to go to limbo.
				let occupied_indices: HashSet<_> =
					points.iter().map(|point| &point.index).collect();
				let index = &active.get_camera().get_index();
				if !occupied_indices.contains(index) {
					// Camera has disconnected.
					self.active_to_disconnected();
					Some(CameraStatus::Disconnected)
				} else {
					None
				}
			},
			ConnectionStatus::Disconnected { .. } => {
				let camera = Self::try_make_camera_from_points(points)?;
				// Succesful reconnection.
				let occupied_indices: HashSet<_> =
					points.iter().map(|point| point.index.clone()).collect();
				self.disconnected_to_active(camera, occupied_indices);
				Some(CameraStatus::Active)
			},
			ConnectionStatus::Limbo { .. } => {
				let camera = Self::try_make_camera_from_points(points)?;
				// Succesful reconnection.
				let occupied_indices: HashSet<_> =
					points.iter().map(|point| point.index.clone()).collect();
				self.limbo_to_active(camera, occupied_indices);
				Some(CameraStatus::Active)
			},
			ConnectionStatus::Broken { .. }
			| ConnectionStatus::Invalid { invalid: _ } => {
				// Do nothing.
				None
			},
		}
	}
}
