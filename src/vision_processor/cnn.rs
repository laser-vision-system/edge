use image::RgbImage;

use super::{CameraReadings, VisionInterface};

use server_edge_datatypes::shared::VisionTechnique;

pub struct Cnn {
    // f32 is a placeholder for now
    deviation: f32,
}

impl Default for Cnn {
    fn default() -> Self {
        Self { deviation: 4.2 }
    }
}

impl Cnn {
    pub fn new(deviation: f32) -> Self {
        Self { deviation }
    }
}

impl VisionInterface for Cnn {
    fn process_frame(&mut self, _frame: &RgbImage, _setpoint: bool) -> CameraReadings {
        // Placeholder for an actual implementation
        CameraReadings::new(self.deviation)
    }
    fn identifier(&self) -> VisionTechnique {
        VisionTechnique::Cnn
    }
}