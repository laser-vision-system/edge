use image::RgbImage;
use serde::{Deserialize, Serialize};

use crate::vision_processor::{cnn::Cnn, diff::Diff, empty::Empty};

use server_edge_datatypes::shared::VisionTechnique;

pub mod cnn;
pub mod demonstration;
pub mod diff;
pub mod empty;

#[must_use]
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct CameraReadings {
	deviation: f32,
}

impl CameraReadings {
	pub fn new(deviation: f32) -> Self {
		Self { deviation }
	}

	pub fn deviation(&self) -> f32 {
		self.deviation
	}
}

impl From<f32> for CameraReadings {
	fn from(value: f32) -> Self {
		Self::new(value)
	}
}

/// A machine vision laser intensity measuring capable object.
///
/// Implement on a type to declare it as capable of performing machine vision intensity measurement.
pub trait VisionInterface {
	fn identifier(&self) -> VisionTechnique;

	fn process_frame(&mut self, frame: &RgbImage, setpoint: bool) -> CameraReadings;
}

pub fn swap(current: &dyn VisionInterface, swap_to: &VisionTechnique) -> Option<Box<dyn VisionInterface>> {
	if swap_to == &current.identifier() {
		return None;
	}
	match swap_to {
		VisionTechnique::Empty => Some(Box::new(Empty::default())),
		VisionTechnique::Cnn => Some(Box::new(Cnn::default())),
		VisionTechnique::Diff => Some(Box::new(Diff::default())),
		VisionTechnique::Demonstration => Some(Box::new(demonstration::Demonstration::default())),
	}
}

// 
// setpoint is per camera and per interface
// it needs to be its own object that is also swapped when the vision interface is swapped
// the setpoint contains the information necessary
// the setpoint will porbs be a dynamic trait
// instead of a bool, the process frame function takes a setpoint oibject
// the setpoint will need to be stored in the metadata struct
// when the vision interface is swapped out, setpoint needs to be cleared (back to None, setpoint will have to be Option<Box< dyn Setpoint>>)
// implement demonstration too in case 
