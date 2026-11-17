//! A physical camera device, as made available by [`nokhwa`].

use std::cmp::Ordering;

use edge_core::{
	image::FrameBuffer,
	logger::logger,
	settings::CAMERA_BUFFER_COUNT,
};
use nokhwa::{
	OpenRequest,
	OpenedCamera,
	StreamCamera,
	session::open_with_buffers,
	utils::{
		ApiBackend,
		CameraControl,
		CameraIndex,
		CameraInfo,
		ControlValueSetter,
		KnownCameraControl,
	},
};

use super::CameraLike;
use crate::{
	controls::{
		ControlMap,
		descriptions::CamControl,
		traits::ControlDescription as _,
	},
	error::{
		EdgeCamerasError,
		EdgeCamerasResult,
	},
};

/// An abstraction over the nokhwa camera objects.
///
/// Prefer using [`CameraLike`] when defining type bounds.
#[must_use]
pub struct CameraDevice {
	inner: StreamCamera,
}

/// Initialisers.
impl CameraDevice {
	/// Attempt to create a device at the given index.
	///
	/// # Errors
	///
	/// If the camera's type is unsupported.
	pub fn try_new(index: CameraIndex) -> EdgeCamerasResult<Self> {
		let index_str = index.to_string();

		let opened =
			open_with_buffers(index, OpenRequest::any(), CAMERA_BUFFER_COUNT)
				.map_err(|why| {
				EdgeCamerasError::NokhwaInitialisation {
					index: index_str,
					source: why,
				}
			})?;

		let stream_camera = match opened {
			OpenedCamera::Stream(stream) => stream,
			OpenedCamera::Shutter(_) => {
				return Err(EdgeCamerasError::UnsupportedCameraType {
					cam_type: "Shutter".into(),
				});
			},
			OpenedCamera::Hybrid(hybrid) => {
				logger().log_warn(
					"HybridCamera detected, converting to StreamCamera",
				);
				hybrid.into()
			},
		};

		Ok(Self {
			inner: stream_camera,
		})
	}

	/// Create the camera at the index and immediately open it.
	///
	/// # Errors
	///
	/// If the camera's type is unsupported.
	pub fn try_new_open(index: CameraIndex) -> EdgeCamerasResult<Self> {
		Self::try_new(index)?.open()
	}

	// pub fn try_new_open_with_default_controls(
	// 	index: CameraIndex,
	//) -> EdgeCamerasResult<Self> {
	// 	Self::try_new(index)?.open()?.with_default_controls()
	//}

	/// Make the camera ready to be read from.
	///
	/// # Errors
	///
	/// If nokhwa could not open the camera.
	pub fn open(mut self) -> EdgeCamerasResult<Self> {
		self.inner.open().map_err(EdgeCamerasError::StreamOpen)?;
		Ok(self)
	}

	// pub fn with_default_controls(mut self) -> EdgeCamerasResult<Self> {
	// 	self.reset_controls_to_default();
	// 	Ok(self)
	//}

	/// Set a camera control using the concrete Nokhwa value type.
	///
	/// # Errors
	///
	/// If attempting to set the control to an invalid value.
	pub fn set_control_value<T>(
		&mut self,
		id: KnownCameraControl,
		value: T,
	) -> EdgeCamerasResult
	where
		T: Into<ControlValueSetter>,
	{
		self.set_value(id, value.into())
	}

	fn set_value(
		&mut self,
		id: KnownCameraControl,
		value: ControlValueSetter,
	) -> EdgeCamerasResult {
		if matches!(id, KnownCameraControl::WhiteBalance) {
			// HACK: Cheekily skip controls that may be inactive/disabled.
			return Ok(());
		}

		let value_string = value.to_string();
		self.inner.set_control(id, value).map_err(|why| {
			EdgeCamerasError::SetControl {
				id,
				value: value_string,
				source: why,
			}
		})
	}
}

impl CameraLike for CameraDevice {
	fn get_info(&self) -> &CameraInfo {
		self.inner.info()
	}

	fn get_index(&self) -> &CameraIndex {
		self.inner.info().index()
	}

	fn get_frame_buffer(&mut self) -> EdgeCamerasResult<FrameBuffer> {
		let frame = self
			.inner
			.frame()
			// Could error if the stream hasn't been opened.
			.map_err(EdgeCamerasError::FrameRead)?;

		Ok(FrameBuffer::from(frame))
	}

	fn get_all_supported_controls(
		&self
	) -> EdgeCamerasResult<Vec<KnownCameraControl>> {
		self.inner
			.controls()
			.map_err(EdgeCamerasError::CannotGetControls)?
			.iter()
			.map(CameraControl::control)
			.map(Ok)
			.collect()
	}

	fn get_controls(&self) -> EdgeCamerasResult<ControlMap> {
		self.inner
			.controls()
			.map_err(EdgeCamerasError::CannotGetControls)
			.map(ControlMap::from)
	}

	fn get_control(
		&self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult<CamControl> {
		self.inner
			.controls()
			.map_err(EdgeCamerasError::CannotGetControls)?
			.into_iter()
			.find(|con| con.control() == id)
			.ok_or(EdgeCamerasError::GetControl(id))
			.map(CamControl::from)
	}

	fn set_control(
		&mut self,
		id: KnownCameraControl,
		value: ControlValueSetter,
	) -> EdgeCamerasResult {
		self.set_value(id, value)
	}

	fn set_controls(
		&mut self,
		controls: ControlMap,
	) -> EdgeCamerasResult {
		controls.into_inner().into_iter().try_for_each(|(id, val)| {
			let CamControl::IntegerRange(control) = val else {
				return Err(EdgeCamerasError::SetControlType(id));
			};
			// TODO: Handle error. Maybe try make this operation atomic?
			self.set_value(id, control.control_value().into())
		})
	}

	///// Set every control on the camera back to its default value.
	//#[expect(
	// 	clippy::too_many_lines,
	// 	clippy::panic,
	// 	clippy::unwrap_used,
	// 	clippy::expect_used,
	// 	reason = "WIP development"
	//)]
	// pub fn reset_controls_to_default(&mut self) {
	// 	let mut controls = self.get_controls().unwrap();
	// 	if let Some(control) =
	// 		controls.get_mut(&KnownCameraControl::AutoExposure)
	// 	{
	// 		let CamControl::Menu(control) = control else {
	// 			panic!("Sorry, I didn't hardcode this setting for your camera")
	// 		};
	//
	// 		let mut value = control.get_value();
	// 		value.set_value(1);
	// 		self.set_control(KnownCameraControl::AutoExposure, value)
	// 			.unwrap();
	// 	}
	//
	// 	if let Some(control) =
	// 		controls.get_mut(&KnownCameraControl::ExposureAbsolute)
	// 	{
	// 		let CamControl::IntegerRange(control) = control else {
	// 			panic!("Sorry, I didn't hardcode this setting for your camera")
	// 		};
	//
	// 		control.reset_to_default();
	// 		self.set_control(
	// 			KnownCameraControl::ExposureAbsolute,
	// 			control.get_value(),
	// 		)
	// 		.unwrap();
	// 	}
	//
	// 	if let Some(control) =
	// 		controls.get_mut(&KnownCameraControl::AutoExposure)
	// 	{
	// 		let CamControl::Menu(control) = control else {
	// 			panic!("Sorry, I didn't hardcode this setting for your camera")
	// 		};
	//
	// 		let mut value = control.get_value();
	// 		value.set_value(3);
	// 		self.set_control(KnownCameraControl::AutoExposure, value)
	// 			.unwrap();
	// 	}
	//
	// 	for (id, control) in self.get_controls().unwrap().into_inner() {
	// 		if id == KnownCameraControl::ExposureAbsolute {
	// 			continue;
	// 		}
	// 		match control {
	// 			CamControl::Integer(mut control) => {
	// 				control.reset_to_default();
	// 				self.set_control(control.get_id(), control.get_value())
	// 					.expect("default controls should always be valid");
	// 			},
	// 			CamControl::IntegerRange(mut control) => {
	// 				control.reset_to_default();
	// 				self.set_control(control.get_id(), control.get_value())
	// 					.expect("default controls should always be valid");
	// 			},
	// 			CamControl::Float(mut control) => {
	// 				control.reset_to_default();
	// 				self.set_control(control.get_id(), control.get_value())
	// 					.expect("default controls should always be valid");
	// 			},
	// 			CamControl::FloatRange(mut control) => {
	// 				control.reset_to_default();
	// 				self.set_control(control.get_id(), control.get_value())
	// 					.expect("default controls should always be valid");
	// 			},
	// 			CamControl::Boolean(mut control) => {
	// 				control.reset_to_default();
	// 				self.set_control(control.get_id(), control.get_value())
	// 					.expect("default controls should always be valid");
	// 			},
	// 			CamControl::String(mut control) => {
	// 				control.reset_to_default();
	// 				self.set_control(
	// 					control.get_id(),
	// 					control.get_value().clone(),
	// 				)
	// 				.expect("default controls should always be valid");
	// 			},
	// 			CamControl::Bytes(mut control) => {
	// 				control.reset_to_default();
	// 				self.set_control(
	// 					control.get_id(),
	// 					control.get_value().clone(),
	// 				)
	// 				.expect("default controls should always be valid");
	// 			},
	// 			CamControl::KeyValuePair(mut control) => {
	// 				control.reset_to_default();
	// 				self.set_control(control.get_id(), control.get_value())
	// 					.expect("default controls should always be valid");
	// 			},
	// 			CamControl::Point(mut control) => {
	// 				control.reset_to_default();
	// 				self.set_control(control.get_id(), control.get_value())
	// 					.expect("default controls should always be valid");
	// 			},
	// 			CamControl::Enum(mut control) => {
	// 				control.reset_to_default();
	// 				self.set_control(control.get_id(), control.get_value())
	// 					.expect("default controls should always be valid");
	// 			},
	// 			CamControl::RGB(mut control) => {
	// 				control.reset_to_default();
	// 				self.set_control(control.get_id(), control.get_value())
	// 					.expect("default controls should always be valid");
	// 			},
	// 			CamControl::Menu(mut control) => {
	// 				control.reset_to_default();
	// 				self.set_control(control.get_id(), control.get_value())
	// 					.expect("default controls should always be valid");
	// 			},
	// 			CamControl::None => (),
	// 		}
	// 	}
	//}

	// The setting of camera controls is handled by the controls themselves.
}

/// Query [`nokhwa`] for all available cameras and then sort them by their
/// index.
///
/// # Panics
///
/// If no nokhwa backend was available.
#[expect(clippy::expect_used, reason = "Low priority edge case")]
pub fn query_for_available_cameras() -> Vec<CameraInfo> {
	let mut cameras = nokhwa::query(ApiBackend::Auto)
		.expect("The backend should be supported");
	cameras.sort_by(|left, right| -> Ordering {
		let left_ind = left.index().as_index().unwrap_or(u32::MAX);
		let right_ind = right.index().as_index().unwrap_or(u32::MAX);
		if left_ind == u32::MAX && right_ind == u32::MAX {
			Ordering::Equal
		} else if left_ind < right_ind {
			Ordering::Less
		} else {
			Ordering::Greater
		}
	});
	cameras
}

#[cfg(test)]
mod tests {
	use std::process::{
		ExitCode,
		Termination,
	};

	use nokhwa::utils::CameraControl;

	use super::*;
	use crate::controls::traits::{
		ControlDefault as _,
		ControlItems as _,
		ControlMax as _,
		ControlStep as _,
		ControlValue as _,
	};

	impl CameraDevice {
		fn get_raw_controls(&self) -> Vec<CameraControl> {
			self.inner.controls().unwrap()
		}
	}

	impl Termination for CameraDevice {
		fn report(self) -> ExitCode {
			ExitCode::SUCCESS
		}
	}

	#[test]
	#[ignore = "IO bound"]
	fn print_devices() {
		let cameras = query_for_available_cameras();

		println!("Available devices: [");
		for cam in cameras {
			if cam.is_metadata() {
				println!("\tindex `{}` (metadata):", cam.index().as_string());
			} else {
				println!("\tindex `{}`:", cam.index().as_string());
			}
			println!(
				"\t\tname: {}\n\t\tdesc: {}",
				cam.human_name(),
				cam.description()
			);
			let misc = cam.misc();
			if !misc.is_empty() {
				println!("\t\tmisc: {misc}");
			}
		}
		println!("]");
	}

	#[test]
	#[ignore = "IO bound"]
	fn new() -> EdgeCamerasResult<CameraDevice> {
		CameraDevice::try_new(CameraIndex::Index(0))
	}

	#[test]
	#[ignore = "IO bound"]
	fn print_camera_details() {
		let cam = CameraDevice::try_new(CameraIndex::Index(0)).unwrap();
		println!("{:#?}", cam.inner.info());
	}

	// Can't implement `Termination` for `RgbImage`.
	#[test]
	#[ignore = "IO bound"]
	fn get_frame() {
		let mut cam = CameraDevice::try_new(CameraIndex::Index(0))
			.unwrap()
			.open()
			.unwrap();
		let _ = cam.get_frame_buffer().unwrap();
		let _ = cam.get_frame_buffer().unwrap();
	}

	#[test]
	#[ignore = "IO bound"]
	fn get_raw_controls() {
		let cam = CameraDevice::try_new(CameraIndex::Index(0)).unwrap();
		let controls = cam.get_raw_controls();
		println!("{controls:#?}");
	}

	#[test]
	#[ignore = "IO bound"]
	fn get_all_available_controls() {
		let cam = CameraDevice::try_new(CameraIndex::Index(0)).unwrap();
		let controls = cam.get_controls().unwrap();
		println!("{controls}");
	}

	#[test]
	#[ignore = "IO bound"]
	fn get_stripped_controls() {
		let cam = CameraDevice::try_new(CameraIndex::Index(0)).unwrap();
		let controls = cam.get_controls().unwrap().to_stripped();
		println!("{controls}");
	}

	#[test]
	#[ignore = "IO bound"]
	fn set_control() -> EdgeCamerasResult {
		let mut cam = CameraDevice::try_new(CameraIndex::Index(0))
			.unwrap()
			.open()
			.unwrap();
		let id = KnownCameraControl::Brightness;

		// Assume it will always be this variant.
		let CamControl::IntegerRange(mut control) = cam.get_control(id)? else {
			return Err(EdgeCamerasError::General(
				"unexpected control variant".into(),
			));
		};
		println!("Start: {}", control.control_value().get());

		let new = control.control_max() - (control.control_step() * 8);
		control
			.try_set_control_value(new.into())
			.expect("new value should be valid");
		cam.set_control_value(id, control.control_value())?;

		let CamControl::IntegerRange(mut control) = cam.get_control(id)? else {
			return Err(EdgeCamerasError::General(
				"unexpected control variant".into(),
			));
		};
		println!("Modified: {}", control.control_value().get());

		control.reset_to_default();
		cam.set_control_value(id, control.control_value())?;

		let CamControl::IntegerRange(control) = cam.get_control(id)? else {
			return Err(EdgeCamerasError::General(
				"unexpected control variant".into(),
			));
		};
		println!("Default: {}", control.control_value().get());

		Ok(())
	}

	#[test]
	#[ignore = "IO bound"]
	fn set_control_exposure() -> EdgeCamerasResult {
		let mut cam = CameraDevice::try_new(CameraIndex::Index(0))
			.unwrap()
			.open()
			.unwrap();
		let id = KnownCameraControl::AutoExposure;
		let desired = 1;

		// Assume it will always be this variant.
		let CamControl::Menu(mut control) = cam.get_control(id)? else {
			return Err(EdgeCamerasError::General(
				"unexpected control variant".into(),
			));
		};
		println!(
			"Start: {}, {:?}",
			control.control_value().get(),
			control.control_items()
		);

		control
			.try_set_control_value(desired.into())
			.expect("new value should be valid");
		cam.set_control_value(id, control.control_value())?;

		let CamControl::Menu(mut control) = cam.get_control(id)? else {
			return Err(EdgeCamerasError::General(
				"unexpected control variant".into(),
			));
		};
		println!(
			"Modified: {}, {:?}",
			control.control_value().get(),
			control.control_items()
		);

		control.reset_to_default();
		cam.set_control_value(id, control.control_value())?;

		let CamControl::Menu(control) = cam.get_control(id)? else {
			return Err(EdgeCamerasError::General(
				"unexpected control variant".into(),
			));
		};
		println!(
			"Default: {}, {:?}",
			control.control_value().get(),
			control.control_items()
		);

		Ok(())
	}

	//#[test]
	//#[ignore = "IO bound"]
	// fn reset_controls_to_default() {
	// 	let mut cam = CameraDevice::try_new(CameraIndex::Index(0))
	// 		.unwrap()
	// 		.open()
	// 		.unwrap();
	// 	cam.reset_controls_to_default();
	//}
}
