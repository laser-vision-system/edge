use std::{cmp::Ordering, io::Cursor};

use image::{DynamicImage, RgbImage};
use nokhwa::{
	Buffer, NokhwaError, OpenRequest, OpenedCamera, StreamCamera,
	format_types::{Mjpeg, Nv12, RawBgr, RawRgb, Yuyv},
	frame::{Frame, IntoRgb as _, RgbConversion},
	session::open_with_buffers,
	utils::{
		CameraIndex, CameraInfo, ControlValueSetter, FrameFormat,
		KnownCameraControl,
	},
};

use super::{
	CameraDeviceError, CannotGetControls, FrameDecodeCorruption, FrameRead,
	NokhwaInitialisation, SelectDecodeFormat, SetControl, StreamOpen,
	controls::{
		ControlMap,
		descriptions::CamControl,
		traits::{
			ControlDefault as _, ControlDescription as _, ControlValue as _,
		},
	},
};
use crate::{
	error::{EdgeError, Result},
	logger::logger,
	settings::CAMERA_BUFFER_COUNT,
};

/// An abstraction over the nokhwa camera objects.
///
/// TODO: The `frame()` function seems to copy the data into a buffer, and then
/// calls decode image which will copy the data again. It may be a good idea to
/// use `frame_raw()` to avoid pointless copy operations.
#[must_use]
pub struct CameraDevice {
	inner: StreamCamera,
}

impl CameraDevice {
	/// Attempt to create a device at the given index.
	#[expect(
		clippy::unwrap_used,
		clippy::unwrap_in_result,
		reason = "WIP development"
	)]
	pub fn try_new(index: CameraIndex) -> Result<Self> {
		let index_str = index.to_string();

		let opened =
			open_with_buffers(index, OpenRequest::any(), CAMERA_BUFFER_COUNT)
				.map_err(|why| CameraDeviceError::NokhwaInitialisation {
				index: index_str,
				inner: NokhwaInitialisation::from(why),
			})?;

		let stream_camera = match opened {
			OpenedCamera::Stream(stream) => stream,
			OpenedCamera::Shutter(_) => {
				return Err(EdgeError::from(
					CameraDeviceError::UnsupportedCameraType {
						cam_type: "Shutter".into(),
					},
				));
			},
			OpenedCamera::Hybrid(hybrid) => {
				logger().lock().unwrap().log_warn(
					"HybridCamera detected, converting to StreamCamera",
				);
				hybrid.into()
			},
		};

		Ok(Self {
			inner: stream_camera,
		})
	}

	pub fn try_new_open(index: CameraIndex) -> Result<Self> {
		Self::try_new(index)?.open()
	}

	pub fn try_new_open_with_default_controls(
		index: CameraIndex,
	) -> Result<Self> {
		Self::try_new(index)?.open()?.with_default_controls()
	}

	pub fn open(mut self) -> Result<Self> {
		self.inner
			.open()
			.map_err(|why| CameraDeviceError::from(StreamOpen::from(why)))?;
		Ok(self)
	}

	pub fn with_default_controls(mut self) -> Result<Self> {
		self.reset_controls_to_default();
		Ok(self)
	}

	#[expect(clippy::unwrap_used, reason = "WIP development")]
	pub fn encode_image(data: &RgbImage) -> Vec<u8> {
		let mut buff = Cursor::new(Vec::new());
		let mut enc = image::codecs::jpeg::JpegEncoder::new(&mut buff);
		enc.encode_image(data).unwrap();
		buff.into_inner()
	}

	pub fn get_info(&self) -> &CameraInfo {
		self.inner.info()
	}

	pub fn get_index(&self) -> &CameraIndex {
		self.inner.info().index()
	}

	// == Images ==

	/// Request an image of the current camera frame.
	///
	/// There seems to be a 2 frame latency.
	pub fn get_frame_buffer(&mut self) -> Result<Buffer> {
		self
			.inner
			.frame()
			// Could error if the stream hasn't been opened.
			.map_err(|why| CameraDeviceError::from(FrameRead::from(why)).into())
	}

	/// Decode an image.
	pub fn decode_buffer(&mut self, buffer: Buffer) -> Result<RgbImage> {
		let decoded_res = select_decode_format(buffer)?;

		let img_buffer = decoded_res
			.materialize()
			// We just assume it is always corruption for now.
			// TODO: Figure out the why.
			// If it is because of memory, maybe try unwind and restart.
			.map_err(|why| CameraDeviceError::from(FrameDecodeCorruption::from(why)))?;

		let frame = DynamicImage::ImageRgb8(img_buffer);
		let new = frame
			.crop_imm(0, 0, frame.width(), frame.height())
			.into_rgb8();
		Ok(new)
	}

	// == Configurations ==

	/// Get the IDs of all supported controls.
	pub fn get_all_supported_controls(
		&self,
	) -> Result<Vec<KnownCameraControl>> {
		self.inner
			.controls()
			.map_err(|why| {
				CameraDeviceError::from(CannotGetControls::from(why))
			})?
			.iter()
			.map(nokhwa::utils::CameraControl::control)
			.map(|con| Ok(con))
			.collect()
	}

	/// Get a map of the camera controls
	pub fn get_controls(&self) -> Result<ControlMap> {
		self.inner
			.controls()
			.map_err(|why| {
				EdgeError::from(CameraDeviceError::from(
					CannotGetControls::from(why),
				))
			})
			.map(ControlMap::from)
	}

	/// Get a camera control by ID
	pub fn get_control(&self, id: KnownCameraControl) -> Result<CamControl> {
		self.inner
			.controls()
			.map_err(|why| {
				CameraDeviceError::from(CannotGetControls::from(why))
			})?
			.into_iter()
			.find(|con| con.control() == id)
			.ok_or_else(|| EdgeError::from(CameraDeviceError::GetControl(id)))
			.map(|con| CamControl::from(con))
	}

	/// Set a camera control
	pub fn set_control<T>(
		&mut self,
		id: KnownCameraControl,
		value: T,
	) -> Result<()>
	where
		T: Into<ControlValueSetter> + Clone,
	{
		if matches!(id, KnownCameraControl::WhiteBalance) {
			// HACK: Cheekily skip controls that may be inactive/disabled.
			return Ok(());
		}
		let setter = value.into();
		self.inner.set_control(id, setter.clone()).map_err(|why| {
			let err = CameraDeviceError::SetControl {
				id,
				value: setter.to_string(),
				inner: SetControl::from(why),
			};
			EdgeError::from(err)
		})
	}

	pub fn set_controls(&mut self, controls: ControlMap) -> Result<()> {
		controls.into_inner().into_iter().try_for_each(|(id, val)| {
			let CamControl::IntegerRange(control) = val else {
				return Err(EdgeError::from(
					CameraDeviceError::SetControlType(id),
				));
			};
			// TODO: Handle error. Maybe try make this operation atomic?
			self.set_control(id, control.get_value())
		})
	}

	/// Set every control on the camera back to its default value.
	#[expect(
		clippy::too_many_lines,
		clippy::panic,
		clippy::unwrap_used,
		reason = "WIP development"
	)]
	pub fn reset_controls_to_default(&mut self) {
		let mut controls = self.get_controls().unwrap();
		if let Some(control) =
			controls.get_mut(&KnownCameraControl::AutoExposure)
		{
			let CamControl::Menu(control) = control else {
				panic!("Sorry, I didn't hardcode this setting for your camera")
			};

			let mut value = control.get_value();
			value.set_value(1);
			self.set_control(KnownCameraControl::AutoExposure, value)
				.unwrap();
		}

		if let Some(control) =
			controls.get_mut(&KnownCameraControl::ExposureAbsolute)
		{
			let CamControl::IntegerRange(control) = control else {
				panic!("Sorry, I didn't hardcode this setting for your camera")
			};

			control.reset_to_default();
			self.set_control(
				KnownCameraControl::ExposureAbsolute,
				control.get_value(),
			)
			.unwrap();
		}

		if let Some(control) =
			controls.get_mut(&KnownCameraControl::AutoExposure)
		{
			let CamControl::Menu(control) = control else {
				panic!("Sorry, I didn't hardcode this setting for your camera")
			};

			let mut value = control.get_value();
			value.set_value(3);
			self.set_control(KnownCameraControl::AutoExposure, value)
				.unwrap();
		}

		for (id, control) in self.get_controls().unwrap().into_inner() {
			if id == KnownCameraControl::ExposureAbsolute {
				continue;
			}
			match control {
				CamControl::Integer(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid");
				},
				CamControl::IntegerRange(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid");
				},
				CamControl::Float(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid");
				},
				CamControl::FloatRange(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid");
				},
				CamControl::Boolean(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid");
				},
				CamControl::String(mut control) => {
					control.reset_to_default();
					self.set_control(
						control.get_id(),
						control.get_value().clone(),
					)
					.expect("default controls should always be valid");
				},
				CamControl::Bytes(mut control) => {
					control.reset_to_default();
					self.set_control(
						control.get_id(),
						control.get_value().clone(),
					)
					.expect("default controls should always be valid");
				},
				CamControl::KeyValuePair(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid");
				},
				CamControl::Point(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid");
				},
				CamControl::Enum(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid");
				},
				CamControl::RGB(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid");
				},
				CamControl::Menu(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid");
				},
				CamControl::None => (),
			}
		}
	}

	// The setting of camera controls is handled by the controls themselves.

	/// Query [`nokhwa`] for all available cameras and then sort them by their
	/// index.
	pub fn query_for_available_cameras() -> Vec<CameraInfo> {
		let mut cameras = nokhwa::query(nokhwa::utils::ApiBackend::Auto)
			.expect("The backend should be supported");
		cameras.sort_by(|a, b| -> Ordering {
			let a_ind = a.index().as_index().unwrap_or(u32::MAX);
			let b_ind = b.index().as_index().unwrap_or(u32::MAX);
			if a_ind == u32::MAX && b_ind == u32::MAX {
				Ordering::Equal
			} else if a_ind < b_ind {
				Ordering::Less
			} else {
				Ordering::Greater
			}
		});
		cameras
	}
}

/// Helper function to choose the decode method.
fn select_decode_format(buf: Buffer) -> Result<RgbConversion> {
	match buf.source_frame_format() {
		FrameFormat::MJPEG => Ok(Frame::<Mjpeg>::try_new(buf)
			.map_err(|why| {
				EdgeError::from(CameraDeviceError::from(
					SelectDecodeFormat::from(why),
				))
			})?
			.into_rgb()),
		FrameFormat::YUYV => Ok(Frame::<Yuyv>::try_new(buf)
			.map_err(|why| {
				EdgeError::from(CameraDeviceError::from(
					SelectDecodeFormat::from(why),
				))
			})?
			.into_rgb()),
		FrameFormat::NV12 => Ok(Frame::<Nv12>::try_new(buf)
			.map_err(|why| {
				EdgeError::from(CameraDeviceError::from(
					SelectDecodeFormat::from(why),
				))
			})?
			.into_rgb()),
		FrameFormat::RAWRGB => Ok(Frame::<RawRgb>::try_new(buf)
			.map_err(|why| {
				EdgeError::from(CameraDeviceError::from(
					SelectDecodeFormat::from(why),
				))
			})?
			.into_rgb()),
		FrameFormat::RAWBGR => Ok(Frame::<RawBgr>::try_new(buf)
			.map_err(|why| {
				EdgeError::from(CameraDeviceError::from(
					SelectDecodeFormat::from(why),
				))
			})?
			.into_rgb()),
		FrameFormat::GRAY => {
			let nok_err = NokhwaError::general(
				"threaded-capture does not support GRAY/Luma cameras",
			);
			let err = EdgeError::from(CameraDeviceError::from(
				SelectDecodeFormat::from(nok_err),
			));
			Err(err)
		},
	}
}

#[cfg(test)]
#[path = "camera_device_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "nokhwa_test.rs"]
mod nokhwa_test;
