#[cfg(test)]
#[path = "camera_device_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "nokhwa_test.rs"]
mod nokhwa_test;

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

use crate::{
	controls::{
		ControlMap,
		descriptions::CamControl,
		traits::ControlDescription as _,
	},
	error::{
		CannotGetControls,
		EdgeCamerasError,
		EdgeCamerasResult,
		FrameRead,
		NokhwaInitialisation,
		SetControl,
		StreamOpen,
	},
};

pub trait CameraLike {
	fn get_info(&self) -> &CameraInfo;

	fn get_index(&self) -> &CameraIndex;

	/// Request an image of the current camera frame.
	///
	/// Latency is based on the buffer count.
	///
	/// # Errors
	///
	/// If the stream hasn't been opened yet.
	fn get_frame_buffer(&mut self) -> EdgeCamerasResult<FrameBuffer>;

	/// # Errors
	///
	/// If this camera has no controls.
	fn get_all_supported_controls(
		&self
	) -> EdgeCamerasResult<Vec<KnownCameraControl>>;

	/// # Errors
	///
	/// If this camera has no controls.
	fn get_controls(&self) -> EdgeCamerasResult<ControlMap>;

	/// # Errors
	///
	/// If the control id could not be found on this camera.
	fn get_control(
		&self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult<CamControl>;

	/// # Errors
	///
	/// If attempting to set the controls to an invalid value.
	fn set_control(
		&mut self,
		id: KnownCameraControl,
		value: ControlValueSetter,
	) -> EdgeCamerasResult<()>;

	/// # Errors
	///
	/// If attempting to set the controls to an invalid value.
	fn set_controls(
		&mut self,
		controls: ControlMap,
	) -> EdgeCamerasResult<()>;
}

/// An abstraction over the nokhwa camera objects.
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
					inner: NokhwaInitialisation::from(why),
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
		self.inner
			.open()
			.map_err(|why| EdgeCamerasError::from(StreamOpen::from(why)))?;
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
	) -> EdgeCamerasResult<()>
	where
		T: Into<ControlValueSetter>,
	{
		self.set_value(id, value.into())
	}

	fn set_value(
		&mut self,
		id: KnownCameraControl,
		value: ControlValueSetter,
	) -> EdgeCamerasResult<()> {
		if matches!(id, KnownCameraControl::WhiteBalance) {
			// HACK: Cheekily skip controls that may be inactive/disabled.
			return Ok(());
		}

		let value_string = value.to_string();
		self.inner.set_control(id, value).map_err(|why| {
			EdgeCamerasError::SetControl {
				id,
				value: value_string,
				inner: SetControl::from(why),
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
			.map_err(|why| EdgeCamerasError::from(FrameRead::from(why)))?;

		Ok(FrameBuffer::from(frame))
	}

	/// Get the IDs of all supported controls.
	fn get_all_supported_controls(
		&self
	) -> EdgeCamerasResult<Vec<KnownCameraControl>> {
		self.inner
			.controls()
			.map_err(|why| {
				EdgeCamerasError::from(CannotGetControls::from(why))
			})?
			.iter()
			.map(CameraControl::control)
			.map(Ok)
			.collect()
	}

	/// Get a map of the camera controls.
	fn get_controls(&self) -> EdgeCamerasResult<ControlMap> {
		self.inner
			.controls()
			.map_err(|why| EdgeCamerasError::from(CannotGetControls::from(why)))
			.map(ControlMap::from)
	}

	/// Get a camera control by ID.
	fn get_control(
		&self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult<CamControl> {
		self.inner
			.controls()
			.map_err(|why| {
				EdgeCamerasError::from(CannotGetControls::from(why))
			})?
			.into_iter()
			.find(|con| con.control() == id)
			.ok_or(EdgeCamerasError::GetControl(id))
			.map(CamControl::from)
	}

	/// Set a camera control.
	fn set_control(
		&mut self,
		id: KnownCameraControl,
		value: ControlValueSetter,
	) -> EdgeCamerasResult<()> {
		self.set_value(id, value)
	}

	fn set_controls(
		&mut self,
		controls: ControlMap,
	) -> EdgeCamerasResult<()> {
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
