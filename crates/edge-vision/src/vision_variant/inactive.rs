use std::fmt;

use crate::{
	batch_item::BatchItem,
	session::SessionToken,
	vision_interface::VisionInterface,
	vision_state::VisionState,
};

#[derive(Clone, Default)]
pub struct Inactive {
	session: SessionToken,
}

impl Inactive {
	pub fn new() -> Self {
		Self {
			session: SessionToken::new(),
		}
	}
}

impl VisionInterface for Inactive {
	fn process(
		&mut self,
		_: &mut [BatchItem],
	) {
	}

	fn new_state(&self) -> VisionState {
		VisionState::Inactive {
			handle: self.session.downgrade(),
			data: ExternalData,
		}
	}
}

impl fmt::Debug for Inactive {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "Inactive")
	}
}

impl fmt::Display for Inactive {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "Inactive")
	}
}

/// Closely associated data used by external holders.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExternalData;

impl fmt::Display for ExternalData {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "ExternalData for Inactive")
	}
}
