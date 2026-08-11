use image::{DynamicImage, RgbImage};
use nokhwa::{
	self, NokhwaError, pixel_format,
	utils::{
		CameraControl, CameraIndex, ControlValueSetter, KnownCameraControl, RequestedFormat,
		RequestedFormatType,
	},
};

/// # Camera
///
/// An abstraction over the nokhwa camera objects.
///
/// Nokhwa's callback camera literally just wraps the regular camera and executes a /loop on a spawned thread. There seems to be no benefit to using it.
///
/// NOTE: Uses optional so that errors can he handled by dropping the camera object. I don't know if this is the best idea, we will discuss this during the logic brainstorming.
///
/// ## TODO
///
/// The `frame()` function seems to copy the data into a buffer, and then calls decode image which will copy the data again. It may be a good idea to use `frame_raw()` to avoid pointless copy operations.
#[must_use]
pub struct Camera(nokhwa::Camera);

impl Camera {
	pub fn new(index: u32) -> Result<Self, NokhwaError> {
		let format = RequestedFormat::new::<pixel_format::RgbFormat>(
			RequestedFormatType::AbsoluteHighestFrameRate,
		);

		// TODO: Handle potential error cases.
		let mut camera = nokhwa::Camera::new(CameraIndex::Index(index), format)?;
		camera.open_stream()?;

		Ok(Self(camera))
	}

	// == Images ==

	/// Request an image of the current camera frame.
	///
	/// There seems to be a 2 frame latency.
	pub fn get_image(&mut self) -> Result<RgbImage, NokhwaError> {
		// TODO: Handle potential error cases.
		let frame = self.0.frame()?;
		let decoded = frame.decode_image::<pixel_format::RgbFormat>()?;
		Ok(decoded)
	}

	pub fn get_image_shrink(&mut self) -> Result<RgbImage, NokhwaError> {
		let frame = DynamicImage::ImageRgb8(self.get_image()?);
		let new = frame
			.crop_imm(0, 0, frame.width(), frame.height())
			.into_rgb8();
		Ok(new)
	}

	// == Configurations ==

	/// Get a list of the camera controls
	///
	/// TODO: Is it a good idea to share this directly?
	pub fn get_all_available_controls(&self) -> Vec<CameraControl> {
		self.0.camera_controls().unwrap()
	}

	/// Set a camera control
	///
	/// Wrapper around the internal nokhwa camera function.
	pub fn set_control(
		&mut self,
		id: KnownCameraControl,
		value: ControlValueSetter,
	) -> Result<(), NokhwaError> {
		self.0.set_camera_control(id, value)
	}
}

#[cfg(test)]
mod tests {
	use std::{
		cmp::Ordering,
		process::{ExitCode, Termination},
	};

	use super::*;

	impl Termination for Camera {
		fn report(self) -> ExitCode {
			ExitCode::SUCCESS
		}
	}

	#[test]
	fn print_devices() {
		let mut cameras = nokhwa::query(nokhwa::utils::ApiBackend::Auto)
			.expect("at least one camera should be connected");
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
	fn new() -> Result<Camera, NokhwaError> {
		Camera::new(0)
	}

	// Can't implement `Termination` for `RgbImage`.

	#[test]
	fn get_image() {
		let mut cam = Camera::new(0).unwrap();
		let _ = cam.get_image().unwrap();
	}

	#[test]
	fn get_image_shrink() {
		let mut cam = Camera::new(0).unwrap();
		let _ = cam.get_image_shrink().unwrap();
	}

	#[test]
	fn get_all_available_controls() {
		let cam = Camera::new(0).unwrap();
		let c = cam.get_all_available_controls();
		println!("{c:#?}");
	}

	#[test]
	fn set_control() -> Result<(), NokhwaError> {
		let mut cam = Camera::new(0).unwrap();
		let id = KnownCameraControl::Brightness;
		let value = ControlValueSetter::Integer(50);
		cam.set_control(id, value)?;
		let ctrl_res = cam.0.camera_control(id)?;
		println!("{}", ctrl_res.value());

		let value = ControlValueSetter::Integer(0);
		cam.set_control(id, value)?;
		let ctrl_res = cam.0.camera_control(id)?;
		println!("{}", ctrl_res.value());
		Ok(())
	}

	#[test]
	fn set_control_exposure() -> Result<(), NokhwaError> {
		let mut cam = Camera::new(0).unwrap();
		let id = KnownCameraControl::Other(10094849);
		let value = ControlValueSetter::Integer(1);
		cam.set_control(id, value)?;
		let ctrl_res = cam.0.camera_control(id)?;
		println!("{}", ctrl_res.value());

		let value = ControlValueSetter::Integer(3);
		cam.set_control(id, value)?;
		let ctrl_res = cam.0.camera_control(id)?;
		println!("{}", ctrl_res.value());
		Ok(())
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
		cmp::Ordering,
		fs::{self, OpenOptions},
		io::Write,
		path::Path,
		thread,
		time::Duration,
	};

	use nokhwa::{
		self, Buffer, CallbackCamera, Camera,
		pixel_format::{RgbAFormat, RgbFormat},
		utils::{ApiBackend, CameraIndex, CameraInfo, RequestedFormat, RequestedFormatType},
	};

	const POLL_INTERVAL: u64 = 400;
	const OUTPUT_PATH: &str = "./temp/nokhwa_test_output/";

	#[test]
	fn main() {
		println!("\t==== NOKHWA TEST ====");
		let cameras = get_all_cameras();
		print_available_camera_info(&cameras);
		let first_camera = cameras.first().unwrap();
		print_camera_details(first_camera);
		let camera_index = first_camera.index().clone();
		println!("\n\t==== Nokhwa Polling Example ====");
		nokhwa_polling_example(camera_index.clone());
		println!("\n\t==== Nokhwa Callback Example ====");
		nokhwa_callback_example(camera_index);
	}

	/// Query [`nokhwa`] for all available cameras and then sort them by their index.
	fn get_all_cameras() -> Vec<CameraInfo> {
		let mut cameras =
			nokhwa::query(ApiBackend::Auto).expect("at least one camera should be connected");
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
		return cameras;
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
		// BUG: v4l unwraps a unit tuple
		match cam_temp.camera_controls_known_camera_controls() {
			Ok(controls) => format!("Known controls: {controls:#?}"),
			Err(e) => String::from(format!("No known controls: {e:?}")),
		}
	}

	/// Helper function to create a [`Camera`].
	fn create_camera(camera_index: CameraIndex) -> Camera {
		let expect_msg =
			format!("the supplied camera index should be a valid camera: {camera_index:?}");
		Camera::new(camera_index, get_format()).expect(&expect_msg)
	}

	/// Helper function for the camera format.
	fn get_format() -> RequestedFormat<'static> {
		RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate)
	}

	/// Uses a camera that must be polled for frames.
	///
	/// Decodes the frame to an RGB image.
	fn nokhwa_polling_example(camera_index: CameraIndex) {
		let mut polling_camera = create_camera(camera_index);
		polling_camera.open_stream().unwrap();
		for i in 0..5 {
			let frame = polling_camera.frame().unwrap();
			println!("Captured frame of size {} bytes", frame.buffer().len());
			let decoded = frame.decode_image::<RgbFormat>().unwrap();
			println!(
				"Decoded frame {i} @ {}x{} ({} bytes)",
				decoded.width(),
				decoded.height(),
				decoded.len()
			);
			thread::sleep(Duration::from_millis(POLL_INTERVAL));
		}
	}

	/// Uses a threaded camera that will automatically use a callback function.
	///
	/// Decodes the frame to an RGBA image.
	///
	/// Saves the captured frame to a .png file.
	fn nokhwa_callback_example(camera_index: CameraIndex) {
		make_sure_output_exists();

		fn callback(buffer: Buffer) {
			let image = buffer.decode_image::<RgbAFormat>().unwrap();
			println!(
				"Frame callback @ {}x{} ({} bytes)",
				image.width(),
				image.height(),
				image.len()
			);
		}

		let expect_msg =
			format!("the supplied camera index should be a valid camera: {camera_index:?}");

		let mut callback_camera =
			CallbackCamera::new(camera_index, get_format(), callback).expect(&expect_msg);
		callback_camera.open_stream().unwrap();

		for i in 0..5 {
			let frame = callback_camera.poll_frame().unwrap();
			let image = frame.decode_image::<RgbAFormat>().unwrap();
			let path = format!("{OUTPUT_PATH}{i}.png");
			println!("Asynchronously saved frame to `{path}`");
			image
				.save(path)
				.expect("output path should be a valid path");
			thread::sleep(Duration::from_millis(POLL_INTERVAL));
		}
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
}
