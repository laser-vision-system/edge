use image::RgbImage;
use serde::{Deserialize, Serialize};

use crate::tcam_interface::Setpoint;

use server_edge_datatypes::shared::VisionTechnique;

mod cnn;
pub(crate) use cnn::Cnn;

mod demonstration;
pub(crate) use demonstration::Demonstration;

mod diff;
pub(crate) use diff::Diff;

mod empty;
pub(crate) use empty::Empty;


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

	/// Returns the deviation from the setpoint
	// potentially change RgbImage to LumaImage here
	fn process_frame(&self, frame: &RgbImage, setpoint: &Setpoint) -> f32;

	/// Each processor needs this function to change with its respective struct inside Setpoint.
	/// No processor should ever touch any struct of Setpoint other than its counterpart.
	fn calibrate_setpoint(&self, frame: &RgbImage, setpoint: &mut Setpoint);
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
