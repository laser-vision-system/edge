//! A client handle provided to the vision processor.

use std::ops::{
	Deref,
	DerefMut,
};

use crate::interface::VisionRx;

/// The handle give to the vision processor.
#[derive(Debug)]
pub struct VisionClientHandle(VisionRx);

impl VisionClientHandle {
	pub(crate) fn new(recv: VisionRx) -> Self {
		Self(recv)
	}
}

impl Deref for VisionClientHandle {
	type Target = VisionRx;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for VisionClientHandle {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
