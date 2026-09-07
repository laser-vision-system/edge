use image::RgbImage;

use super::{VisionInterface};

use server_edge_datatypes::shared::VisionTechnique;

use crate::tcam_interface::{Setpoint};

#[derive(Default)]
pub struct Cnn {
    // Anything may be stored here if necessary (such as the model, or the model's weights)
}

impl Cnn {
    pub fn new() -> Self {
        Self {}
    }

    fn perform_analysis(&self, frame: &RgbImage) -> f32 {
        // whatever it needs to do
        4.2
    }
}

impl VisionInterface for Cnn {
	fn identifier(&self) -> VisionTechnique {
		VisionTechnique::Cnn
	}

	fn process_frame(&self, frame: &RgbImage, setpoint: &Setpoint) -> f32 {
		match setpoint.cnn_setpoint.intensity {
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