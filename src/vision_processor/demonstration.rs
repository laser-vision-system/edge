use image::RgbImage;
use lab::Lab;
use rayon::prelude::*;

use super::{CameraReadings, VisionInterface};

#[must_use]
pub struct Demonstration {
	setpoint: f32,
}

impl Default for Demonstration {
	fn default() -> Self {
		Self { setpoint: -1.0 }
	}
}

impl Demonstration {
	// NOTE: Clippy complains about indexing the slice without asserting, but the par_chunk_exact guarantees this won't panic.
	#[allow(clippy::missing_asserts_for_indexing)]
	#[allow(clippy::indexing_slicing)]
	#[allow(clippy::cast_precision_loss)]
	fn average_lightness(&mut self, frame: &RgbImage, setpoint: bool) -> (f32, f32) {
		let raw_bytes = frame.as_raw();
		let pixel_count = raw_bytes.len() as f32 / 3.0;

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

impl VisionInterface for Demonstration {
	fn process_frame(&mut self, frame: &RgbImage, setpoint: bool) -> CameraReadings {
		self.average_lightness(frame, setpoint).into()
	}
}
