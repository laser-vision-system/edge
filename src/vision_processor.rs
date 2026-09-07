use image::RgbImage;
use serde::{Deserialize, Serialize};

use crate::vision_processor::{cnn::Cnn, diff::Diff, empty::Empty, demonstration::Demonstration};

use crate::tcam_interface::Setpoint;

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

/// A machine vision laser intensity measuring capable object.
///
/// Implement on a type to declare it as capable of performing machine vision intensity measurement.
pub trait VisionInterface {
	fn identifier(&self) -> VisionTechnique;


	// potentially change RgbImage to LumaImage here
	fn process_frame(&self, frame: &RgbImage, setpoint: &mut dyn Setpoint) -> f32;

	fn analyse_frame(&self, frame: &RgbImage, setpoint: &dyn Setpoint) -> f32;

	// Write the new setpoint data to the setpoint
	fn calibrate_setpoint(&self, frame: &RgbImage, setpoint: &mut dyn Setpoint);
}

/// Swaps the current vision processing technique to the chosen one.
/// If the current technique is the same as the chosen one, returns None.
/// This function must be called from the conductor where the necessary setpoint logic is implemented.
/// The setpoint logic is not implemented here because it is not the responsibility of the vision processor to manage setpoints.
pub fn swap(
	swap_to: &VisionTechnique,
	current: &dyn VisionInterface,
) -> Option<Box<dyn VisionInterface>> {
	if swap_to == &current.identifier() {
		return None;
	}
	match swap_to {
		VisionTechnique::Empty => Some(Box::new(Empty::default())),
		VisionTechnique::Cnn => Some(Box::new(Cnn::default())),
		VisionTechnique::Diff => Some(Box::new(Diff::default())),
		VisionTechnique::Demonstration => Some(Box::new(Demonstration::default())),
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
