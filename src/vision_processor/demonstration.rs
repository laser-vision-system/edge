use image::RgbImage;
use lab::Lab;
use rayon::prelude::*;

use super::VisionInterface;

use server_edge_datatypes::shared::VisionTechnique;

use crate::tcam_interface::Setpoint;

#[must_use]
#[derive(Default)]
pub struct Demonstration {}

impl Demonstration {
	fn new() -> Self {
		Self {}
	}

	// NOTE: Clippy complains about indexing the slice without asserting, but the par_chunk_exact guarantees this won't panic.
	#[allow(clippy::missing_asserts_for_indexing)]
	#[allow(clippy::indexing_slicing)]
	#[allow(clippy::cast_precision_loss)]
	fn average_lightness(&self, frame: &RgbImage) -> f32 {
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

		lightness
	}
}

impl VisionInterface for Demonstration {
	fn identifier(&self) -> VisionTechnique {
		VisionTechnique::Demonstration
	}

	fn process_frame(&self, frame: &RgbImage, setpoint: &Setpoint) -> f32 {
		match setpoint.demonstration_setpoint.intensity {
			Some(_setpoint_intensity) => {
				return 1.0 - (self.average_lightness(frame) / _setpoint_intensity);
			},
			None => {
				return -1.0;
			},
		}
	}

	fn calibrate_setpoint(&self, frame: &RgbImage, setpoint: &mut Setpoint) {
		setpoint.demonstration_setpoint.intensity = Some(self.average_lightness(frame));
	}
}
