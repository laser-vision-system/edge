use std::fmt;

use super::{BatchItem, SessionToken, VisionInterface, VisionState};

#[derive(Clone)]
pub struct Empty {
	session: SessionToken,
}

impl Empty {
	pub fn new() -> Self {
		Self {
			session: SessionToken::new(),
		}
	}
}

impl VisionInterface for Empty {
	fn process(&mut self, _: &mut [BatchItem]) {
		return;
	}

	fn new_state(&self) -> VisionState {
		VisionState::Empty {
			handle: self.session.downgrade(),
			data: ExternalData::default(),
		}
	}
}

impl Default for Empty {
	fn default() -> Self {
		Self {
			session: SessionToken::new(),
		}
	}
}

impl fmt::Debug for Empty {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Empty")
	}
}

impl fmt::Display for Empty {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Empty")
	}
}

/// Closely associated data used by external holders.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExternalData;

impl fmt::Display for ExternalData {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ExternalData for Empty")
	}
}
