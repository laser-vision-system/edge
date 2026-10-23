//! A token and and handle created by a [`VisionVariant`] to alert a holder when
//! the current variant has been swapped.
//!
//! [`VisionVariant`]: crate::vision_variant::VisionVariant

use std::sync::{
	Arc,
	Weak,
};

/// Created by a [`VisionVariant`] when instanced.
///
/// [`VisionVariant`]: crate::vision_variant::VisionVariant
#[derive(Clone)]
pub struct SessionToken(Arc<()>);

impl Default for SessionToken {
	fn default() -> Self {
		Self::new()
	}
}

impl SessionToken {
	/// Create a new [`SessionToken`].
	pub fn new() -> Self {
		Self(Arc::new(()))
	}

	/// Create a [`SessionHandle`] pointing to this token.
	pub fn downgrade(&self) -> SessionHandle {
		SessionHandle(Arc::downgrade(&self.0))
	}
}

/// A type that is provided by a [`VisionVariant`] and can be queried to check
/// if the current variant has been swapped.
///
/// [`VisionVariant`]: crate::vision_variant::VisionVariant
#[derive(Clone, Debug)]
pub struct SessionHandle(Weak<()>);

impl SessionHandle {
	/// Returns true only if the owning vision processor is still active.
	pub fn is_valid(&self) -> bool {
		self.0.upgrade().is_some()
	}
}
