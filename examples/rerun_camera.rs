use std::error::Error;

use edge::{
	tcam_interface::Camera,
	vision_processor::{BatchItem, Demonstration, VisionInterface, VisionState},
};
use image::RgbImage;
use rerun::{self, ColorModel, Image, RecordingStream, RecordingStreamBuilder};

struct Context {
	rec: RecordingStream,
	demo: Demonstration,
	state: VisionState,
}

impl Context {
	fn new() -> Result<Self, Box<dyn Error>> {
		let rec: RecordingStream = RecordingStreamBuilder::new("cam_test_harness").spawn()?;
		let demo = Demonstration::default();
		let state = demo.new_state();
		Ok(Context { rec, demo, state })
	}

	pub fn send_image(&self, path: &str, image: &Image) {
		self.rec.log(path, image).unwrap();
	}

	pub fn send_scalar(&self, path: &str, value: f64) {
		self.rec.log(path, &rerun::Scalars::single(value)).unwrap();
	}

	pub fn log_frame(&mut self, path: &str, frame: RgbImage, incr: u8) {
		let rerun_image =
			Image::from_elements(&frame, [frame.width(), frame.height()], ColorModel::RGB);
		self.send_image(path, &rerun_image);

		let batch = BatchItem {
			camera_id: 0,
			image: frame,
			state: &mut self.state,
			calibrate: incr == 0,
			send_to_server: false,
		};
		let mut batched = [batch];
		self.demo.process(&mut batched);

		let VisionState::Demonstration { data, .. } = &self.state else {
			return;
		};

		let lightness_path = format!("{path}/lightness");
		self.send_scalar(&lightness_path, data.intensity.unwrap() as f64);

		let deviation_path = format!("{path}/deviation");
		self.send_scalar(&deviation_path, data.deviation.unwrap() as f64);
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut context = Context::new()?;
	let mut camera = Camera::new(0)?;

	let mut incr: u8 = 0;

	loop {
		let frame = camera.get_frame()?;
		let image = camera.decode_image(frame)?;
		context.log_frame("camera/live_feed", image, incr);
		if incr == 50 {
			incr = 0;
		} else {
			incr += 1;
		}
	}
}
