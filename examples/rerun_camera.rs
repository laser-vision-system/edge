use std::{env, error::Error, thread};

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
	let w = frame.width();
	let h = frame.height();
	let expected_len = (w * h * 3) as usize;

	let raw_bytes = frame.as_raw();
	let exact_bytes = &raw_bytes[..expected_len];

	let rerun_image = Image::from_elements(exact_bytes, [w, h], ColorModel::RGB);
	context.send_image(path, &rerun_image);

	let lightness = Demonstration::average_lightness(frame);

	let scalar_path = format!("{path}/lightness");
	context.send_scalar(&scalar_path, lightness as f64);
}

enum RunMode {
	Polling,
	Callback,
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut args = env::args().skip(1);
	let mode = match args.next().unwrap().to_lowercase().as_str() {
		"callback" => Ok(RunMode::Callback),
		"polling" => Ok(RunMode::Polling),
		unknown => Err(format!(
			"Unknown mode: '{unknown}'. Expected 'polling' or 'callback'."
		)),
	}?;

	let context = Context::new()?;

	match mode {
		RunMode::Polling => {
			println!("Starting in POLLING mode. Press Ctrl+C to stop.");
			let mut camera = Camera::new_polling(0)?;

			loop {
				let frame = camera.get_frame()?;
				log_frame(&context, "camera/live_feed", &frame);
			}
		},
		RunMode::Callback => {
			println!("Starting in CALLBACK mode. Press Ctrl+C to stop.");
			let ctx_for_callback = context.clone();

			let callback = move |frame: RgbImage| {
				log_frame(&ctx_for_callback, "camera/live_feed", &frame);
			};

			// Bind the camera and then park the main thread so the callback can operate on its own thread.
			let _camera = Camera::new_callback(0, callback)?;
			loop {
				thread::park();
			}
		},
	}
}
