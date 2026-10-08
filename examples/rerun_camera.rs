#![expect(clippy::unwrap_used, reason = "Quick and dirty example")]

use std::error::Error;

use edge::{
	tcam_interface::{CameraDevice, CameraLike as _, Image},
	vision_processor::{
		AvgLightness, BatchItem, VisionInterface as _, VisionState,
	},
};
use nokhwa::utils::CameraIndex;
use rerun::{self, ColorModel, RecordingStream, RecordingStreamBuilder};

struct Context {
	rec: RecordingStream,
	demo: AvgLightness,
	state: VisionState,
}

impl Context {
	fn new() -> Result<Self, Box<dyn Error>> {
		let rec: RecordingStream =
			RecordingStreamBuilder::new("cam_test_harness").spawn()?;
		let demo = AvgLightness::default();
		let state = demo.new_state();
		Ok(Context { rec, demo, state })
	}

	pub fn send_image(&self, path: &str, image: &rerun::Image) {
		self.rec.log(path, image).unwrap();
	}

	pub fn send_scalar(&self, path: &str, value: f64) {
		self.rec.log(path, &rerun::Scalars::single(value)).unwrap();
	}

	pub fn log_frame(&mut self, path: &str, frame: Image, incr: u8) {
		let rerun_image = rerun::Image::from_elements(
			&frame,
			[frame.width(), frame.height()],
			ColorModel::RGB,
		);
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

		let VisionState::AvgLightness { data, .. } = &self.state else {
			return;
		};

		let lightness_path = format!("{path}/lightness");
		self.send_scalar(&lightness_path, f64::from(data.intensity.unwrap()));

		let deviation_path = format!("{path}/deviation");
		self.send_scalar(&deviation_path, f64::from(data.deviation.unwrap()));
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut context = Context::new()?;
	let mut camera = CameraDevice::try_new_open(CameraIndex::Index(0))?;

	let mut incr: u8 = 0;

	loop {
		let frame = camera.get_frame_buffer()?;
		let image = frame.try_decode()?;
		context.log_frame("camera/live_feed", image, incr);
		if incr == 50 {
			incr = 0;
		} else {
			incr += 1;
		}
	}
}
