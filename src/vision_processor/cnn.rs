use std::fmt;

use super::{BatchItem, SessionToken, VisionInterface, VisionState};

#[derive(Clone, Default)]
pub struct Cnn {
	session: SessionToken,
}

impl Cnn {
	pub fn new() -> Self {
		Self {
			session: SessionToken::new(),
		}
	}
}

impl VisionInterface for Cnn {
	fn process(&mut self, batch: &mut [BatchItem]) {
		for item in batch {
			let VisionState::Cnn { data, .. } = &mut item.state else {
				// TODO: This should raise an error.
				continue;
			};
			data.deviation = Some(0.0)
		}
	}

	fn new_state(&self) -> VisionState {
		VisionState::Cnn {
			handle: self.session.downgrade(),
			data: ExternalData::default(),
		}
	}
}

impl fmt::Debug for Cnn {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Cnn")
	}
}

impl fmt::Display for Cnn {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Cnn")
	}
}

/// Closely associated data used by external holders.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ExternalData {
	pub setpoint: Option<f32>,
	pub deviation: Option<f32>,
}

impl fmt::Display for ExternalData {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ExternalData for Cnn: {:?}", self.setpoint)
	}
}
