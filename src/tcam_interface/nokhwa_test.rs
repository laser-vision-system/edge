#![expect(clippy::print_stdout, reason = "Tests are allowed to print")]
#![expect(clippy::use_debug, reason = "Tests are a debug context")]
#![expect(clippy::panic, reason = "Tests are allowed to panic")]
#![expect(
	clippy::unwrap_used,
	clippy::unwrap_in_result,
	reason = "Tests are allowed to unwrap"
)]

//! # Nokhwa Test
//!
//! Runs a quick demonstration of the nokhwa library:
//!
//! - Queries available camera devices, selecting the first one for use
//! - Displays configuration options
//! - Creates a polled camera, accessing 5 frames and printing image dimensions
//! - Creates a callback camera, printing image dimensions. Simultaneously polls
//!   the camera 5 times and saves the images.
use std::{
	fs::{self, OpenOptions},
	io::Write as _,
	path::Path,
	thread,
	time::Duration,
};

use nokhwa::{
	self, Buffer, CameraRunner, NokhwaError, OpenRequest, OpenedCamera,
	RunnerConfig, StreamCamera,
	format_types::{Mjpeg, Nv12, RawBgr, RawRgb, Yuyv},
	frame::{Frame, IntoRgb as _, IntoRgba as _, RgbaConversion},
	open,
	session::open_with_buffers,
	utils::{CameraIndex, CameraInfo, FrameFormat},
};

use crate::settings::CAMERA_BUFFER_COUNT;

const POLL_INTERVAL: u64 = 400;
const OUTPUT_PATH: &str = "./temp/nokhwa_test_output/";

#[test]
#[ignore = "IO bound"]
fn main() {
	println!("\t==== NOKHWA TEST ====");
	let cameras = super::CameraDevice::query_for_available_cameras();
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
	println!("Available devices: [");
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

/// Print the known camera controls for a camera found at a
/// [`CameraIndex`]
fn get_known_camera_controls(camera_index: CameraIndex) -> String {
	let cam_temp = create_camera(camera_index);
	match cam_temp.controls() {
		Ok(controls) => format!("Known controls: {controls:#?}"),
		Err(e) => format!("No known controls: {e:?}"),
	}
}

/// Helper function to create a [`StreamCamera`].
fn create_camera(camera_index: CameraIndex) -> StreamCamera {
	let expect_msg = format!(
		"the supplied camera index should be a valid camera: {camera_index:?}"
	);
	let opened = open_with_buffers(
		camera_index,
		OpenRequest::any(),
		CAMERA_BUFFER_COUNT,
	)
	.expect(&expect_msg);
	let OpenedCamera::Stream(stream_cam) = opened else {
		panic!("AA")
	};
	stream_cam
}

///// Helper function for the camera format.
// fn get_format() -> RequestedFormat<'static> {
// 	RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate)
//}

/// Uses a camera that must be polled for frames.
///
/// Decodes the frame to an RGB image.
fn nokhwa_polling_example(camera_index: CameraIndex) {
	let mut polling_camera = create_camera(camera_index);
	polling_camera.open().unwrap();
	for i in 0..5u8 {
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

/// Uses a threaded camera that will automatically use a callback
/// function.
///
/// Decodes the frame to an RGBA image.
///
/// Saves the captured frame to a .png file.
fn nokhwa_callback_example(camera_index: CameraIndex) {
	make_sure_output_exists();

	// fn callback(buffer: Buffer) {
	// 	let image = buffer.decode_image::<RgbAFormat>().unwrap();
	// 	println!(
	// 		"Frame callback @ {}x{} ({} bytes)",
	// 		image.width(),
	// 		image.height(),
	// 		image.len()
	// 	);
	//}

	let opened = open(camera_index, OpenRequest::any()).unwrap();
	let runner = CameraRunner::spawn(opened, RunnerConfig::default()).unwrap();
	let frames = runner.frames().unwrap();

	// let mut callback_camera =
	// 	CallbackCamera::new(camera_index, get_format(),
	// callback).expect(&expect_msg); callback_camera.open_stream().
	// unwrap();

	for _ in 0..5_u8 {
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

		// let frame = callback_camera.poll_frame().unwrap();
		// let image = frame.decode_image::<RgbAFormat>().unwrap();
		// let path = format!("{OUTPUT_PATH}{i}.png");
		// println!("Asynchronously saved frame to `{path}`");
		// image
		// 	.save(path)
		// 	.expect("output path should be a valid path");
		thread::sleep(Duration::from_millis(POLL_INTERVAL));
	}

	runner.stop().unwrap();
}

fn make_sure_output_exists() {
	let output_folder = Path::new(OUTPUT_PATH);
	let expect_msg =
		format!("the output path should be a valid location: {OUTPUT_PATH}");
	fs::create_dir_all(output_folder).expect(&expect_msg);

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
