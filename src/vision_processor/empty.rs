use image::RgbImage;

use super::{VisionInterface};

use server_edge_datatypes::shared::VisionTechnique;

use crate::tcam_interface::{Setpoint, SetpointType};

pub struct Empty {
    // f32 is a placeholder for now
    deviation: f32,
}

impl Default for Empty {
    fn default() -> Self {
        Self { deviation: 0.0 }
    }
}

impl Empty {
    pub fn new(deviation: f32) -> Self {
        Self { deviation }
    }
}

impl VisionInterface for Empty {
    fn process_frame(&mut self, _frame: &RgbImage, _setpoint: &mut dyn Setpoint) -> f32 {
        0.0
    }

    fn identifier(&self) -> VisionTechnique {
        VisionTechnique::Empty
    }
}

impl From<EmptySetpoint> for SetpointType {
    fn from(x: EmptySetpoint) -> Self {
        Self::Empty(x)
    }
}

pub struct EmptySetpointDataType {
	setpoint: Option<f32>,
}