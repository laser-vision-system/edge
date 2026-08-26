use std::error::Error;

use edge::{
	tcam_interface::camera::Camera,
	vision_processor::{VisionInterface, demonstration::Demonstration},
};
use image::RgbImage;
use rerun::{self, ColorModel, Image, RecordingStream, RecordingStreamBuilder};

struct Context {
	rec: RecordingStream,
	demo: Demonstration,
}

impl Context {
	fn new() -> Result<Self, Box<dyn Error>> {
		let rec: RecordingStream = RecordingStreamBuilder::new("cam_test_harness").spawn()?;
		let demo = Demonstration::default();
		Ok(Context { rec, demo })
	}

	pub fn send_image(&self, path: &str, image: &Image) {
		self.rec.log(path, image).unwrap();
	}

	pub fn send_scalar(&self, path: &str, value: f64) {
		self.rec.log(path, &rerun::Scalars::single(value)).unwrap();
	}

	pub fn log_frame(&mut self, path: &str, frame: &RgbImage) {
		let rerun_image =
			Image::from_elements(frame, [frame.width(), frame.height()], ColorModel::RGB);
		self.send_image(path, &rerun_image);

		let readings = self.demo.process_frame(frame, false);

		let lightness_path = format!("{path}/lightness");
		self.send_scalar(&lightness_path, readings.intensity() as f64);

		let deviation_path = format!("{path}/deviation");
		self.send_scalar(&deviation_path, readings.deviation() as f64);
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut context = Context::new()?;
	let mut camera = Camera::new(0, None)?;

	loop {
		let frame = camera.get_image_shrink()?;
		context.log_frame("camera/live_feed", &frame);
	}
}
