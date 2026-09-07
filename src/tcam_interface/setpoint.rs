use image::RgbImage;

use crate::vision_processor::{
	VisionInterface, cnn::{Cnn, CnnSetpoint, CnnSetpointDataType}, demonstration::{Demonstration, DemonstrationSetpoint, DemonstrationSetpointDataType}, diff::{Diff, DiffSetpoint, DiffSetpointDataType}, empty::{Empty, EmptySetpoint, EmptySetpointDataType},
};

pub trait Setpoint {
	/// Calibrate the next image that is processed by the vision interface.
	/// This will set a flag that will be checked within the process_frame function of the vision interface.
	/// If the flag is set, the vision interface will perform a calibration on the next image that is processed.
	/// The flag must be cleared after the calibration is performed.

	fn set_calibrate_next_image(&mut self, flag: bool);

	/// Returns the current state of the calibrate_next_image flag.
	fn calibrate_next_image(&self) -> bool;

	/// Clear the setpoint. This will reset the setpoint to its default state.
	fn clear(&mut self);

	/// Calibrate the setpoint with the provided data.
	/// This function must reset the calibrate_next_image flag to false after the calibration is performed.
    /// A reference to a vision implementation must be passed so that the calibration function can
    /// perform the correct processing of the image to produce a setpoint.
	fn calibrate(&mut self, frame: &RgbImage, vision_implementation: &dyn VisionInterface);
}

// Please excuse the naming it is horrendous and I am tired
// pub enum SetpointVariant {
// 	DemonstrationSetpointVariant(Demonstration),
// 	DiffSetpointVariant(Diff),
// 	CnnSetpointVariant(Cnn),
// 	EmptySetpointVariant(Empty),
// }

// impl Setpoint for SetpointVariant {
// 	fn set_calibrate_next_image(&mut self, flag: bool) {
// 		match self {
// 			SetpointVariant::Demonstration(setpoint) => setpoint.set_calibrate_next_image(flag),
// 			SetpointVariant::Diff(setpoint) => setpoint.set_calibrate_next_image(flag),
// 			SetpointVariant::Cnn(setpoint) => setpoint.set_calibrate_next_image(flag),
// 			SetpointVariant::Empty(setpoint) => setpoint.set_calibrate_next_image(flag),
// 		}
// 	}

// 	fn calibrate_next_image(&self) -> bool {
// 		match self {
// 			SetpointVariant::Demonstration(setpoint) => setpoint.calibrate_next_image(),
// 			SetpointVariant::Diff(setpoint) => setpoint.calibrate_next_image(),
// 			SetpointVariant::Cnn(setpoint) => setpoint.calibrate_next_image(),
// 			SetpointVariant::Empty(setpoint) => setpoint.calibrate_next_image(),
// 		}
// 	}

// 	fn clear(&mut self) {
// 		match self {
// 			SetpointVariant::Demonstration(setpoint) => setpoint.clear(),
// 			SetpointVariant::Diff(setpoint) => setpoint.clear(),
// 			SetpointVariant::Cnn(setpoint) => setpoint.clear(),
// 			SetpointVariant::Empty(setpoint) => setpoint.clear(),
// 		}
// 	}

// 	fn calibrate(&mut self, image: &RgbImage, setpoint_type: SetpointType) {
// 		match self {
// 			SetpointVariant::Demonstration(setpoint) => setpoint.calibrate(image),
// 			SetpointVariant::Diff(setpoint) => setpoint.calibrate(image),
// 			SetpointVariant::Cnn(setpoint) => setpoint.calibrate(image),
// 			SetpointVariant::Empty(setpoint) => setpoint.calibrate(image),
// 		}
// 	}
// }
