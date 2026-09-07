use image::{GrayImage, RgbImage};

use super::VisionInterface;

use server_edge_datatypes::shared::VisionTechnique;

use crate::tcam_interface::{Setpoint, SetpointType};

#[derive(Default)]
pub struct Diff {}

impl Diff {
	pub fn new() -> Self {
		Self {}
	}
}

impl VisionInterface for Diff {
	fn process_frame(&mut self, frame: &RgbImage, setpoint: &mut dyn Setpoint) -> f32 {
		// Placeholder for an actual implementation
		2.0
	}
	fn identifier(&self) -> VisionTechnique {
		VisionTechnique::Diff
	}
}

pub struct DiffSetpoint {
	setpoint: DiffSetpointDataType,
	calibrate_next_image: bool,
}

impl Setpoint for DiffSetpoint {
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
		let gray_image = image::imageops::grayscale(image);
		self.setpoint = Some(gray_image);
		self.calibrate_next_image = false;
	}
}

impl From<DiffSetpoint> for SetpointType {
    fn from(x: DiffSetpoint) -> Self {
        Self::Diff(x)
    }
}

pub struct DiffSetpointDataType {
    // TODO: Store calculate metrics fot the setpoint image during calibrate and save them here.
	setpoint: Option<GrayImage>,
}