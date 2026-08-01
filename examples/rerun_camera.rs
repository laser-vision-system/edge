use std::error::Error;

use image::RgbImage;
use rerun::{self, ColorModel, Image, RecordingStream, RecordingStreamBuilder};

use edge::Camera;
use edge::Demonstration;

#[derive(Clone)]
struct Context {
	rec: RecordingStream,
}

impl Context {
	fn new() -> Result<Self, Box<dyn Error>> {
		let rec: RecordingStream = RecordingStreamBuilder::new("webcam_test_harness").spawn()?;
		Ok(Context { rec })
	}

	pub fn send_image(&self, path: &str, image: &Image) {
		self.rec.log(path, image).unwrap();
	}

	pub fn send_scalar(&self, path: &str, value: f64) {
		self.rec.log(path, &rerun::Scalars::single(value)).unwrap();
	}
}

fn log_frame(context: &Context, path: &str, frame: &RgbImage) {
	let rerun_image = Image::from_elements(frame, [frame.width(), frame.height()], ColorModel::RGB);
	context.send_image(path, &rerun_image);

	let lightness = Demonstration::average_lightness(frame);

	let scalar_path = format!("{path}/lightness");
	context.send_scalar(&scalar_path, lightness as f64);
}

fn main() -> Result<(), Box<dyn Error>> {
	let context = Context::new()?;

	println!("Starting in POLLING mode. Press Ctrl+C to stop.");
	let mut camera = Camera::new(0)?;

	loop {
		let frame = camera.get_image_shrink()?;
		log_frame(&context, "camera/live_feed", &frame);
	}
}
