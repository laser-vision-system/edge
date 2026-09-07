use image::RgbImage;
use lab::Lab;
use rayon::prelude::*;

use super::VisionInterface;

use server_edge_datatypes::shared::VisionTechnique;

use crate::tcam_interface::{Setpoint, SetpointData, SetpointType};

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
	fn average_lightness(&mut self, frame: &RgbImage) -> f32 {
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
	fn process_frame(&mut self, frame: &RgbImage, setpoint: &mut dyn Setpoint) -> f32 {
		if setpoint.calibrate_next_image() {
			// First deactivate the flag
			setpoint.set_calibrate_next_image(false);
			setpoint.calibrate(frame, self);
			return 0.0;
		}
		self.average_lightness(frame)
	}

	fn identifier(&self) -> VisionTechnique {
		VisionTechnique::Demonstration
	}
}

pub struct DemonstrationSetpoint {
	setpoint: DemonstrationSetpointDataType,
	calibrate_next_image: bool,
}

impl Setpoint for DemonstrationSetpoint {
	fn set_calibrate_next_image(&mut self, flag: bool) {
		self.calibrate_next_image = flag;
	}

	fn calibrate_next_image(&self) -> bool {
		self.calibrate_next_image
	}

	fn clear(&mut self) {
		self.setpoint = None;
	}

	fn calibrate(&mut self, frame: &RgbImage, vision_implementation: &dyn VisionInterface) {
		self.setpoint = vision_implementation.average_lightness(frame);
		self.calibrate_next_image = false;
	}
}

// impl From<DemonstrationSetpointDataType> for SetpointData {
// 	fn from(x: DemonstrationSetpointDataType) -> Self {
// 		Self::DemonstrationSetpointData(x)
// 	}
// }

// pub struct DemonstrationSetpointDataType {
// 	setpoint: Option<f32>,
// }

// impl DemonstrationSetpointDataType {
// 	fn new(value: Option<f32>) -> Self {
// 		Self { setpoint: value }
// 	}
// }
