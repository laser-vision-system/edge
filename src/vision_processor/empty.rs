use image::RgbImage;

use super::VisionInterface;

use server_edge_datatypes::shared::VisionTechnique;

use crate::tcam_interface::Setpoint;

#[derive(Default)]
pub struct Empty {}

impl Empty {
	pub fn new() -> Self {
		Self {}
	}

	fn perform_analysis(&self, frame: &RgbImage) -> f32 {
		// whatever it needs to do
		0.0
	}
}

impl VisionInterface for Empty {
	fn identifier(&self) -> VisionTechnique {
		VisionTechnique::Empty
	}

	fn process_frame(&self, frame: &RgbImage, setpoint: &Setpoint) -> f32 {
		match setpoint.empty_setpoint.intensity {
			Some(_setpoint_intensity) => {
				return 1.0 - (self.perform_analysis(frame) / _setpoint_intensity);
			},
			None => {
				return -1.0;
			},
		}
	}

	fn calibrate_setpoint(&self, frame: &RgbImage, setpoint: &mut Setpoint) {
		setpoint.demonstration_setpoint.intensity = Some(self.perform_analysis(frame));
	}
}
