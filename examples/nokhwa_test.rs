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
	Buffer, CallbackCamera, Camera,
	pixel_format::{RgbAFormat, RgbFormat},
	query,
	utils::{ApiBackend, CameraIndex, RequestedFormat, RequestedFormatType},
};

const POLL_INTERVAL: u64 = 400;
const OUTPUT_PATH: &str = "./data/nokhwa_test_output/";

fn main() {
	println!("\t==== NOKHWA TEST ====");
	let camera_index = query_available_devices();
	println!("\n\t==== Nokhwa Polling Example ====");
	nokhwa_polling_example(camera_index.clone());
	println!("\n\t==== Nokhwa Callback Example ====");
	nokhwa_callback_example(camera_index);
}

fn query_available_devices() -> CameraIndex {
	let cameras = query(ApiBackend::Auto).unwrap();

	print!("Available devices: [\n");
	for cam in &cameras {
		print!(
			"\t{}:\n\t\t{:?}\n\t\t{:?}",
			cam.index().as_index().unwrap_or(u32::MAX),
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

	let first_camera = cameras.first().unwrap();
	println!(
		"\nUsing camera {}: {:?}\n―  {}",
		first_camera.index().as_index().unwrap_or(u32::MAX),
		first_camera.human_name(),
		get_known_camera_controls(first_camera.index().clone())
	);

	first_camera.index().clone()
}

fn get_format() -> RequestedFormat<'static> {
	RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate)
}

fn get_known_camera_controls(camera_index: CameraIndex) -> String {
	let cam_temp = Camera::new(camera_index, get_format()).unwrap();
	match cam_temp.camera_controls_known_camera_controls() {
		Ok(controls) => format!("Known controls: {controls:#?}"),
		Err(_) => String::from("No known controls"),
	}
}

fn nokhwa_polling_example(camera_index: CameraIndex) {
	//! Uses a camera that must be polled for frames.
	//! Decodes the frame to an RGB image.

	let mut polling_camera = Camera::new(camera_index, get_format()).unwrap();
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

fn make_sure_output_exists() {
	let output_folder = Path::new(OUTPUT_PATH);
	fs::create_dir_all(&output_folder).unwrap();

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

fn nokhwa_callback_example(camera_index: CameraIndex) {
	//! Uses a threaded camera that will automatically use a callback function.
	//! Decodes the frame to an RGBA image.
	//! Saves the captured frame to a .png file.

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

	let mut callback_camera = CallbackCamera::new(camera_index, get_format(), callback).unwrap();
	callback_camera.open_stream().unwrap();

	for i in 0..5 {
		let frame = callback_camera.poll_frame().unwrap();
		let image = frame.decode_image::<RgbAFormat>().unwrap();
		let path = format!("{OUTPUT_PATH}{i}.png");
		println!("Asynchronously saved frame to `{path}`");
		image.save(path).unwrap();
		thread::sleep(Duration::from_millis(POLL_INTERVAL));
	}
}
