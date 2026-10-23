use std::ops::{
	Deref,
	DerefMut,
};

use crate::interface::VisionRx;

#[derive(Debug)]
pub struct VisionClientHandle(VisionRx);

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
