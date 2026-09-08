use std::fmt;

use image::RgbImage;

use super::{BatchItem, SessionToken, VisionInterface, VisionState};

#[derive(Clone)]
pub struct Diff {
	session: SessionToken,
}

impl Diff {
	pub fn new() -> Self {
		Self {
			session: SessionToken::new(),
		}
	}

	pub fn new_state(&self) -> VisionState {
		VisionState::Diff {
			handle: self.session.downgrade(),
			data: ExternalData::default(),
		}
	}
}

impl VisionInterface for Diff {
	fn process(&mut self, batch: &mut [BatchItem]) {
		for item in batch {
			let VisionState::Diff { data, .. } = &mut item.state else {
				// TODO: This should raise an error.
				continue;
			};
			data.deviation = 0.0
		}
	}
}

impl Default for Diff {
	fn default() -> Self {
		Self {
			session: SessionToken::new(),
		}
	}
}

impl fmt::Debug for Diff {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Diff")
	}
}

impl fmt::Display for Diff {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Diff")
	}
}

/// Closely associated data used by external holders.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalData {
	pub image: Option<RgbImage>,
	pub peak: Option<f32>,
	pub mean: Option<f32>,
	pub std_dev: Option<f32>,
	pub deviation: f32,
}

impl fmt::Display for ExternalData {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"ExternalData for Diff\n - peak: {:?}\n - mean: {:?}\n - std_dev: {:?}",
			self.peak, self.mean, self.std_dev
		)
	}
}
