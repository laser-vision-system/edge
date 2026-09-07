use image::RgbImage;

use super::{VisionInterface};

use server_edge_datatypes::shared::VisionTechnique;

use crate::tcam_interface::{Setpoint, SetpointType};

#[derive(Default)]
pub struct Cnn {
    // Anything may be stored here if necessary (such as the model, or the model's weights)
}

impl Cnn {
    pub fn new() -> Self {
        Self {}
    }
}

impl VisionInterface for Cnn {
    // type SetpointType = f32;

    fn process_frame(&self, frame: &RgbImage, setpoint: &mut dyn Setpoint) -> f32 {
        // Placeholder for an actual implementation
        3.0
    }
    fn identifier(&self) -> VisionTechnique {
        VisionTechnique::Cnn
    }
}

pub struct CnnSetpoint {
    setpoint: CnnSetpointDataType,
    calibrate_next_image: bool,
}

impl Setpoint for CnnSetpoint {
    fn set_calibrate_next_image(&mut self, flag: bool) {
        self.calibrate_next_image = flag;
    }

    fn calibrate_next_image(&self) -> bool {
        self.calibrate_next_image
    }

    fn clear(&mut self) {
        self.setpoint = None;
    }

    fn calibrate(&mut self, image: &RgbImage) {
        // Placeholder for an actual implementation
        self.setpoint = Some(1.1);
        self.calibrate_next_image = false;
    }
}

impl From<CnnSetpoint> for SetpointType {
    fn from(x: CnnSetpoint) -> Self {
        Self::Cnn(x)
    }
}

pub struct CnnSetpointDataType {
	setpoint: Option<f32>,
}
