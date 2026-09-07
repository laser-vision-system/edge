use image::RgbImage;

use super::{CameraReadings, VisionInterface};

use server_edge_datatypes::shared::VisionTechnique;

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
    fn process_frame(&mut self, _frame: &RgbImage, _setpoint: bool) -> CameraReadings {
        // Placeholder for an actual implementation
        CameraReadings::new(self.deviation)
    }
    fn identifier(&self) -> VisionTechnique {
        VisionTechnique::Empty
    }
}