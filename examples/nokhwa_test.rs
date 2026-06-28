use std::{thread, time::Duration};

use nokhwa::{
	Buffer, CallbackCamera, Camera,
	pixel_format::{RgbAFormat, RgbFormat},
	query,
	utils::{ApiBackend, CameraIndex, RequestedFormat, RequestedFormatType},
};

fn main() {
	nokhwa_example();
	println!();
	nokhwa_threaded_example();
}

fn nokhwa_example() {
	let index = CameraIndex::Index(0);
	let requested =
		RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

	let mut camera = Camera::new(index, requested).unwrap();
	println!("\t== Using ==\n{:#?}", camera.info());

	camera.open_stream().unwrap();

	for i in 0..5 {
		let frame = camera.frame().unwrap();
		println!("Captured single frame of {}", frame.buffer().len());
		let decoded = frame.decode_image::<RgbFormat>().unwrap();
		println!(
			"Decoded frame {i} of {}x{} ({})",
			decoded.width(),
			decoded.height(),
			decoded.len()
		);
	}
}

fn nokhwa_threaded_example() {
	let cameras = query(ApiBackend::Auto).unwrap();
	println!("\t== Available Cameras ==");
	cameras.iter().for_each(|cam| println!("{cam:#?}"));

	let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

	let first_camera = cameras.first().unwrap();
	println!("\t== Using {:?} ==", first_camera.description());

	let callback = |buffer: Buffer| {
		let image = buffer.decode_image::<RgbAFormat>().unwrap();
		println!(
			"Frame callback: {}x{} ({})",
			image.width(),
			image.height(),
			image.len()
		);
	};

	let mut threaded = CallbackCamera::new(first_camera.index().clone(), format, callback).unwrap();

	let path = format!("data/output/");
	std::fs::create_dir_all(path).unwrap();

	threaded.open_stream().unwrap();

	for i in 0..5 {
		let frame = threaded.poll_frame().unwrap();
		let image = frame.decode_image::<RgbAFormat>().unwrap();
		let path = format!("data/output/{i}.png");
		println!("Asynchronously saved frame to `{path}`");
		image.save(path).unwrap();
		thread::sleep(Duration::from_millis(400));
	}
}
