use std::cmp::Ordering;

use image::{DynamicImage, RgbImage};
use nokhwa::{
	self, OpenRequest, OpenedCamera,
	format_types::Mjpeg,
	frame::{Frame, IntoRgb},
	open,
	utils::{CameraControl, CameraIndex, CameraInfo, ControlValueSetter, KnownCameraControl},
};

use crate::error::{EdgeError, Result};

use super::controls::{
	ControlMap,
	descriptions::CamControl,
	extract_control, extract_controls,
	traits::{ControlDefault, ControlDescription, ControlValue},
};

/// An abstraction over the nokhwa camera objects.
///
/// TODO: The `frame()` function seems to copy the data into a buffer, and then calls decode image which will copy the data again. It may be a good idea to use `frame_raw()` to avoid pointless copy operations.
#[must_use]
pub struct Camera {
	inner: nokhwa::StreamCamera,
	name: String,
}

impl Camera {
	pub fn new(index: u32, name: Option<&str>) -> Result<Self> {
		let name = name.unwrap_or("camera").to_string();

		// TODO: Handle potential error cases.
		let opened = open(CameraIndex::Index(index), OpenRequest::any()).map_err(|why| {
			EdgeError::NokhwaInitialisation {
				index,
				reason: why.to_string(),
			}
		})?;
		let OpenedCamera::Stream(mut camera) = opened else {
			// TODO: Discuss how we handle this.
			panic!("AA")
		};
		camera.open().map_err(|why| EdgeError::StreamOpen {
			reason: why.to_string(),
		})?;

		Ok(Self {
			inner: camera,
			name,
		})
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	// == Images ==

	/// Request an image of the current camera frame.
	///
	/// There seems to be a 2 frame latency.
	pub fn get_image(&mut self) -> Result<RgbImage> {
		// TODO: Handle potential error cases.
		let buffer = self.inner.frame().map_err(|_| EdgeError::FrameRead)?;
		let frame: Frame<Mjpeg> = Frame::new(buffer);
		let decoded = frame
			.into_rgb()
			.materialize()
			// We just assume it is always corruption for now.
			.map_err(|_| EdgeError::FrameDecodeCorruption)?;
		Ok(decoded)
	}

	pub fn get_image_shrink(&mut self) -> Result<RgbImage> {
		let frame = DynamicImage::ImageRgb8(self.get_image()?);
		let new = frame
			.crop_imm(0, 0, frame.width(), frame.height())
			.into_rgb8();
		Ok(new)
	}

	// == Configurations ==

	/// Get the IDs of all supported controls.
	pub fn get_all_supported_controls(&self) -> Vec<KnownCameraControl> {
		self.inner
			.controls()
			.unwrap()
			.iter()
			.map(|con| con.control())
			.collect()
	}

	/// Get a map of the camera controls
	pub fn get_controls(&self) -> ControlMap {
		extract_controls(self.inner.controls().unwrap())
	}

	/// Get a camera control by ID
	pub fn get_control(&self, id: KnownCameraControl) -> CamControl {
		let nokhwa_control = self
			.inner
			.controls()
			.unwrap()
			.into_iter()
			.find(|con| con.control() == id)
			.unwrap();
		extract_control(nokhwa_control)
	}

	/// Set a camera control
	pub fn set_control<T>(&mut self, id: KnownCameraControl, value: T) -> Result<()>
	where
		T: Into<ControlValueSetter>,
	{
		if match id {
			KnownCameraControl::WhiteBalance => true,
			_ => false,
		} {
			// HACK: Cheekily skip controls that may be inactive/disabled.
			return Ok(());
		}
		self.inner
			.set_control(id, value.into())
			.map_err(|_| EdgeError::SetControl)
	}

	#[allow(dead_code)]
	fn get_raw_controls(&self) -> Vec<CameraControl> {
		self.inner.controls().unwrap()
	}

	/// Set every control on the camera back to its default value.
	pub fn reset_controls_to_default(&mut self) {
		let mut controls = self.get_controls();
		if let Some(control) = controls.get_mut(&KnownCameraControl::AutoExposure) {
			let CamControl::Menu(control) = control else {
				panic!("Sorry, I didn't hardcode this setting for your camera")
			};

			let mut value = control.get_value();
			value.set_value(1);
			self.set_control(KnownCameraControl::AutoExposure, value)
				.unwrap();
		}

		if let Some(control) = controls.get_mut(&KnownCameraControl::ExposureAbsolute) {
			let CamControl::IntegerRange(control) = control else {
				panic!("Sorry, I didn't hardcode this setting for your camera")
			};

			control.reset_to_default();
			self.set_control(KnownCameraControl::ExposureAbsolute, control.get_value())
				.unwrap();
		}

		if let Some(control) = controls.get_mut(&KnownCameraControl::AutoExposure) {
			let CamControl::Menu(control) = control else {
				panic!("Sorry, I didn't hardcode this setting for your camera")
			};

			let mut value = control.get_value();
			value.set_value(3);
			self.set_control(KnownCameraControl::AutoExposure, value)
				.unwrap();
		}

		for (id, control) in self.get_controls() {
			if id == KnownCameraControl::ExposureAbsolute {
				continue;
			}
			match control {
				CamControl::Integer(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid")
				},
				CamControl::IntegerRange(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid")
				},
				CamControl::Float(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid")
				},
				CamControl::FloatRange(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid")
				},
				CamControl::Boolean(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid")
				},
				CamControl::String(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), (&control.get_value()).clone())
						.expect("default controls should always be valid")
				},
				CamControl::Bytes(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), (&control.get_value()).clone())
						.expect("default controls should always be valid")
				},
				CamControl::KeyValuePair(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid")
				},
				CamControl::Point(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid")
				},
				CamControl::Enum(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid")
				},
				CamControl::RGB(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid")
				},
				CamControl::Menu(mut control) => {
					control.reset_to_default();
					self.set_control(control.get_id(), control.get_value())
						.expect("default controls should always be valid")
				},
				CamControl::None => (),
			};
		}
	}

	// The setting of camera controls is handled by the controls themselves.

	/// Query [`nokhwa`] for all available cameras and then sort them by their index.
	pub fn query_for_available_cameras() -> Vec<CameraInfo> {
		let mut cameras = nokhwa::query(nokhwa::utils::ApiBackend::Auto).unwrap();
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

#[cfg(test)]
mod tests {
	use std::process::{ExitCode, Termination};

	use crate::tcam_interface::controls::traits::{
		ControlDefault, ControlDescription, ControlItems, ControlMax, ControlStep, ControlValue,
	};

	use super::*;

	impl Termination for Camera {
		fn report(self) -> ExitCode {
			ExitCode::SUCCESS
		}
	}

	#[test]
	fn print_devices() {
		let cameras = Camera::query_for_available_cameras();

		print!("Available devices: [\n");
		for cam in cameras {
			print!(
				"\t{}:\n\t\t{}\n\t\t{}",
				cam.index().as_string(),
				cam.human_name(),
				cam.description(),
			);
			let misc = cam.misc();
			if !misc.is_empty() {
				print!("\n\t\t{misc:?}");
			}
			println!();
		}
		println!("]");
	}

	#[test]
	fn new() -> Result<Camera> {
		Camera::new(0, None)
	}

	#[test]
	fn print_camera_details() {
		let cam = Camera::new(0, None).unwrap();
		println!("{:#?}", cam.inner.info());
	}

	// Can't implement `Termination` for `RgbImage`.
	#[test]
	fn get_image() {
		let mut cam = Camera::new(0, None).unwrap();
		let _ = cam.get_image().unwrap();
	}

	#[test]
	fn get_image_shrink() {
		let mut cam = Camera::new(0, None).unwrap();
		let _ = cam.get_image_shrink().unwrap();
	}

	#[test]
	fn get_raw_controls() {
		let cam = Camera::new(0, None).unwrap();
		let c = cam.get_raw_controls();
		println!("{c:#?}");
	}

	#[test]
	fn get_all_available_controls() {
		let cam = Camera::new(0, None).unwrap();
		let c = cam.get_controls();
		println!("{c:#?}");
	}

	#[test]
	fn set_control() -> Result<()> {
		let mut cam = Camera::new(0, None).unwrap();
		let id = KnownCameraControl::Brightness;

		// Assume it will always be this variant.
		let CamControl::IntegerRange(mut control) = cam.get_control(id) else {
			panic!("unexpected control variant")
		};
		println!("Start: {}", control.get_value().get_value());

		let new = control.get_max() - (control.get_step() * 8);
		control
			.set_value(new.into())
			.expect("new value should be valid");
		cam.set_control(id, control.get_value())?;

		let CamControl::IntegerRange(mut control) = cam.get_control(id) else {
			panic!("unexpected control variant")
		};
		println!("Modified: {}", control.get_value().get_value());

		control.reset_to_default();
		cam.set_control(id, control.get_value())?;

		let CamControl::IntegerRange(control) = cam.get_control(id) else {
			panic!("unexpected control variant")
		};
		println!("Default: {}", control.get_value().get_value());

		Ok(())
	}

	#[test]
	fn set_control_exposure() -> Result<()> {
		let mut cam = Camera::new(0, None).unwrap();
		let id = KnownCameraControl::AutoExposure;
		let desired = 1;

		// Assume it will always be this variant.
		let CamControl::Menu(mut control) = cam.get_control(id) else {
			panic!("unexpected control variant")
		};
		println!(
			"Start: {}, {:?}",
			control.get_value().get_value(),
			control.get_items()
		);

		control
			.set_value(desired.into())
			.expect("new value should be valid");
		cam.set_control(id, control.get_value())?;

		let CamControl::Menu(mut control) = cam.get_control(id) else {
			panic!("unexpected control variant")
		};
		println!(
			"Modified: {}, {:?}",
			control.get_value().get_value(),
			control.get_items()
		);

		control.reset_to_default();
		cam.set_control(id, control.get_value())?;

		let CamControl::Menu(control) = cam.get_control(id) else {
			panic!("unexpected control variant")
		};
		println!(
			"Default: {}, {:?}",
			control.get_value().get_value(),
			control.get_items()
		);

		Ok(())
	}

	#[test]
	fn reset_controls_to_default() {
		let mut cam = Camera::new(0, None).unwrap();
		cam.reset_controls_to_default();
	}
}

#[cfg(test)]
mod nokhwa_test {
	//! # Nokhwa Test
	//!
	//! Runs a quick demonstration of the nokhwa library:
	//!
	//! - Queries available camera devices, selecting the first one for use
	//! - Displays configuration options
	//! - Creates a polled camera, accessing 5 frames and printing image dimensions
	//! - Creates a callback camera, printing image dimensions. Simultaneously polls the camera 5 times and saves the images.
	use std::{
		fs::{self, OpenOptions},
		io::Write,
		path::Path,
		thread,
		time::Duration,
	};

	use nokhwa::{
		self, Buffer, CameraRunner, NokhwaError, OpenRequest, OpenedCamera, RunnerConfig,
		StreamCamera,
		format_types::{Mjpeg, Nv12, RawBgr, RawRgb, Yuyv},
		frame::{Frame, IntoRgb, IntoRgba, RgbaConversion},
		open,
		utils::{CameraIndex, CameraInfo, FrameFormat},
	};

	const POLL_INTERVAL: u64 = 400;
	const OUTPUT_PATH: &str = "./temp/nokhwa_test_output/";

	#[test]
	fn main() {
		println!("\t==== NOKHWA TEST ====");
		let cameras = super::Camera::query_for_available_cameras();
		print_available_camera_info(&cameras);
		let first_camera = cameras.first().unwrap();
		print_camera_details(first_camera);
		let camera_index = first_camera.index().clone();
		println!("\n\t==== Nokhwa Polling Example ====");
		nokhwa_polling_example(camera_index.clone());
		println!("\n\t==== Nokhwa Callback Example ====");
		nokhwa_callback_example(camera_index);
	}

	/// List the details from a supplied [`Vec`] of [`CameraInfo`].
	fn print_available_camera_info(cameras: &Vec<CameraInfo>) {
		print!("Available devices: [\n");
		for cam in cameras {
			print!(
				"\t{}:\n\t\t{}\n\t\t{}",
				cam.index().as_string(),
				cam.human_name(),
				cam.description(),
			);
			let misc = cam.misc();
			if !misc.is_empty() {
				print!("\n\t\t{misc:?}");
			}
			println!();
		}
		println!("]");
	}

	/// Print details from the supplied [`CameraInfo`].
	fn print_camera_details(camera: &CameraInfo) {
		println!(
			"\nUsing camera {}: {:?}\n―  {}",
			camera.index().as_index().unwrap_or(u32::MAX),
			camera.human_name(),
			get_known_camera_controls(camera.index().clone())
		);
	}

	/// Print the known camera controls for a camera found at a [`CameraIndex`]
	fn get_known_camera_controls(camera_index: CameraIndex) -> String {
		let cam_temp = create_camera(camera_index);
		match cam_temp.controls() {
			Ok(controls) => format!("Known controls: {controls:#?}"),
			Err(e) => String::from(format!("No known controls: {e:?}")),
		}
	}

	/// Helper function to create a [`StreamCamera`].
	fn create_camera(camera_index: CameraIndex) -> StreamCamera {
		let expect_msg =
			format!("the supplied camera index should be a valid camera: {camera_index:?}");
		let opened = open(camera_index, OpenRequest::any()).expect(&expect_msg);
		let OpenedCamera::Stream(stream_cam) = opened else {
			panic!("AA")
		};
		return stream_cam;
	}

	///// Helper function for the camera format.
	//fn get_format() -> RequestedFormat<'static> {
	//	RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate)
	//}

	/// Uses a camera that must be polled for frames.
	///
	/// Decodes the frame to an RGB image.
	fn nokhwa_polling_example(camera_index: CameraIndex) {
		let mut polling_camera = create_camera(camera_index);
		polling_camera.open().unwrap();
		for i in 0..5 {
			let buffer = polling_camera.frame().unwrap();
			println!("Captured frame of size {} bytes", buffer.buffer().len());
			let frame: Frame<Mjpeg> = Frame::new(buffer);
			let decoded = frame.into_rgb().materialize().unwrap();
			println!(
				"Decoded frame {i} @ {}x{} ({} bytes)",
				decoded.width(),
				decoded.height(),
				decoded.len()
			);
			thread::sleep(Duration::from_millis(POLL_INTERVAL));
		}
		polling_camera.close().unwrap();
	}

	/// Uses a threaded camera that will automatically use a callback function.
	///
	/// Decodes the frame to an RGBA image.
	///
	/// Saves the captured frame to a .png file.
	fn nokhwa_callback_example(camera_index: CameraIndex) {
		make_sure_output_exists();

		//fn callback(buffer: Buffer) {
		//	let image = buffer.decode_image::<RgbAFormat>().unwrap();
		//	println!(
		//		"Frame callback @ {}x{} ({} bytes)",
		//		image.width(),
		//		image.height(),
		//		image.len()
		//	);
		//}

		let opened = open(camera_index, OpenRequest::any()).unwrap();
		let runner = CameraRunner::spawn(opened, RunnerConfig::default()).unwrap();
		let frames = runner.frames().unwrap();

		//let mut callback_camera =
		//	CallbackCamera::new(camera_index, get_format(), callback).expect(&expect_msg);
		//callback_camera.open_stream().unwrap();

		for _ in 0..5 {
			let buffer = frames.recv_timeout(Duration::from_secs(2)).unwrap();
			println!(
				"Callback recieved buffer of {} bytes",
				buffer.buffer().len()
			);
			let image = decode_to_rgba(buffer).unwrap().materialize().unwrap();
			println!(
				"Frame callback @ {}x{} ({} bytes)",
				image.width(),
				image.height(),
				image.len()
			);

			//let frame = callback_camera.poll_frame().unwrap();
			//let image = frame.decode_image::<RgbAFormat>().unwrap();
			//let path = format!("{OUTPUT_PATH}{i}.png");
			//println!("Asynchronously saved frame to `{path}`");
			//image
			//	.save(path)
			//	.expect("output path should be a valid path");
			thread::sleep(Duration::from_millis(POLL_INTERVAL));
		}

		runner.stop().unwrap();
	}

	fn make_sure_output_exists() {
		let output_folder = Path::new(OUTPUT_PATH);
		let expect_msg = format!("the output path should be a valid location: {OUTPUT_PATH}");
		fs::create_dir_all(&output_folder).expect(&expect_msg);

		let ignore_file = output_folder.join(".gitignore");
		let contents = fs::read_to_string(&ignore_file).unwrap_or_default();

		if !contents.lines().any(|line| line.trim() == "*") {
			let mut file = OpenOptions::new()
				.create(true)
				.append(true)
				.open(&ignore_file)
				.unwrap();

			if !contents.is_empty() && !contents.ends_with('\n') {
				file.write_all(b"\n\n*\n").unwrap();
			} else {
				file.write_all(b"*\n").unwrap();
			}
		}
	}

	/// Wrap a `Buffer` in the typed `Frame<F>` matching its own fourcc and
	/// start a lazy RGBA conversion. Dispatching on the buffer's format
	/// avoids the panic that `Frame::<Mjpeg>::new` raises when the backend
	/// negotiates a non-MJPEG format (most Linux webcams pick YUYV).
	fn decode_to_rgba(buf: Buffer) -> Result<RgbaConversion, NokhwaError> {
		match buf.source_frame_format() {
			FrameFormat::MJPEG => Ok(Frame::<Mjpeg>::try_new(buf)?.into_rgba()),
			FrameFormat::YUYV => Ok(Frame::<Yuyv>::try_new(buf)?.into_rgba()),
			FrameFormat::NV12 => Ok(Frame::<Nv12>::try_new(buf)?.into_rgba()),
			FrameFormat::RAWRGB => Ok(Frame::<RawRgb>::try_new(buf)?.into_rgba()),
			FrameFormat::RAWBGR => Ok(Frame::<RawBgr>::try_new(buf)?.into_rgba()),
			FrameFormat::GRAY => Err(NokhwaError::general(
				"threaded-capture does not support GRAY/Luma cameras",
			)),
		}
	}
}
