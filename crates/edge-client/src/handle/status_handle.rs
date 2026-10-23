use std::ops::{
	Deref,
	DerefMut,
};

use crate::interface::StatusRx;

#[derive(Debug)]
pub struct StatusHandle(StatusRx);

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
