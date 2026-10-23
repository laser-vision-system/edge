//! A handle given to the conductor to check the status.

use std::ops::{
	Deref,
	DerefMut,
};

use crate::interface::StatusRx;

/// The handle that is used to check the current connection status.
#[derive(Debug)]
pub struct StatusHandle(StatusRx);

impl StatusHandle {
	pub(crate) fn new(recv: StatusRx) -> Self {
		Self(recv)
	}
}

impl Deref for StatusHandle {
	type Target = StatusRx;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for StatusHandle {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
