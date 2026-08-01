use image::RgbImage;
use lab::Lab;
use rayon::prelude::*;

pub struct Demonstration {
	setpoint: f32,
}

impl Demonstration {
	pub fn new() -> Self {
		Self { setpoint: -1.0 }
	}

	pub fn average_lightness(&mut self, frame: &RgbImage, setpoint: bool) -> (f32, f32) {
		let raw_bytes = frame.as_raw();
		let pixel_count = (raw_bytes.len() / 3) as f32;

		let total_lightness: f32 = raw_bytes
			.par_chunks_exact(3)
			.map(|pixel| {
				let lab_colour = Lab::from_rgb(&[pixel[0], pixel[1], pixel[2]]);

				lab_colour.l
			})
			.sum();

		let lightness = total_lightness / pixel_count;

		if setpoint {
			self.setpoint = lightness;
		}

		let mut deviation = 0.0;
		if self.setpoint > 0.0 {
			deviation = lightness / self.setpoint;
		}

		(lightness, deviation)
	}
}
