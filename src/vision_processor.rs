use image::RgbImage;
use serde::{Deserialize, Serialize};

pub mod demonstration;

#[must_use]
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct CameraReadings {
	intensity: f32,
	deviation: f32,
}

impl CameraReadings {
	pub fn new(intensity: f32, deviation: f32) -> Self {
		Self {
			intensity,
			deviation,
		}
	}

	pub fn intensity(&self) -> f32 {
		self.intensity
	}

	pub fn deviation(&self) -> f32 {
		self.deviation
	}
}

impl From<(f32, f32)> for CameraReadings {
	fn from(value: (f32, f32)) -> Self {
		Self::new(value.0, value.1)
	}
}

/// A machine vision laser intensity measuring capable object.
///
/// Implement on a type to declare it as capable of performing machine vision intensity measurement.
pub trait VisionInterface {
	fn process_frame(&mut self, frame: &RgbImage, setpoint: bool) -> CameraReadings;
}
