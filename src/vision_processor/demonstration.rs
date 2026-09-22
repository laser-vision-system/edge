use std::fmt;

use image::RgbImage;
use lab::Lab;
use rayon::prelude::*;

use super::{BatchItem, SessionToken, VisionInterface, VisionState};

#[derive(Clone, Default)]
pub struct Demonstration {
	session: SessionToken,
}

impl Demonstration {
	pub fn new() -> Self {
		Self {
			session: SessionToken::new(),
		}
	}

	// NOTE: Clippy complains about indexing the slice without asserting, but
	// the par_chunk_exact guarantees this won't panic.
	#[allow(clippy::missing_asserts_for_indexing)]
	#[allow(clippy::indexing_slicing)]
	#[allow(clippy::cast_precision_loss)]
	fn average_lightness(frame: &RgbImage) -> f32 {
		let raw_bytes = frame.as_raw();
		let pixel_count = raw_bytes.len() as f32 / 3.0;

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

impl VisionInterface for Demonstration {
	fn process(&mut self, batch: &mut [BatchItem]) {
		for item in batch {
			let VisionState::Demonstration { data, .. } = item.state else {
				// TODO: This should raise an error.
				continue;
			};

			let lightness = Self::average_lightness(&item.image);

			if item.calibrate {
				data.setpoint = Some(lightness);
			}

			let mut deviation = 0.0;
			if let Some(setpoint) = data.setpoint
				&& setpoint > 0.0
			{
				deviation = lightness / setpoint;
			}

			data.intensity = Some(lightness);
			data.deviation = Some(deviation);
		}
	}

	fn new_state(&self) -> VisionState {
		VisionState::Demonstration {
			handle: self.session.downgrade(),
			data: ExternalData::default(),
		}
	}
}

impl fmt::Debug for Demonstration {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Demonstration")
	}
}

impl fmt::Display for Demonstration {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Demonstration")
	}
}

/// Closely associated data used by external holders.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ExternalData {
	pub setpoint: Option<f32>,
	pub intensity: Option<f32>,
	pub deviation: Option<f32>,
}

impl fmt::Display for ExternalData {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ExternalData for Demonstration: {:?}", self.setpoint)
	}
}
