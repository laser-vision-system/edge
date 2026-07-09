use image::RgbImage;
use lab::Lab;
use rayon::prelude::*;

pub struct Demonstration {
	//pub setpoint: f32,
}

impl Demonstration {
	pub fn average_lightness(frame: &RgbImage) -> f32 {
		let raw_bytes = frame.as_raw();
		let pixel_count = (raw_bytes.len() / 3) as f32;

		let total_lightness: f32 = raw_bytes
			.par_chunks_exact(3)
			.map(|pixel| {
				let lab_colour = Lab::from_rgb(&[pixel[0], pixel[1], pixel[2]]);

				lab_colour.l
			})
			.sum();

		total_lightness / pixel_count
	}
}
