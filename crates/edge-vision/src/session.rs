use std::sync::{
	Arc,
	Weak,
};

#[derive(Clone)]
pub struct SessionToken(Arc<()>);

impl Default for SessionToken {
	fn default() -> Self {
		Self::new()
	}
}

impl SessionToken {
	pub fn new() -> Self {
		Self(Arc::new(()))
	}

	pub fn downgrade(&self) -> SessionHandle {
		SessionHandle(Arc::downgrade(&self.0))
	}
}

#[derive(Clone, Debug)]
pub struct SessionHandle(Weak<()>);

impl SessionHandle {
	/// Returns true only if the owning vision processor is still active.
	pub fn is_valid(&self) -> bool {
		self.0.upgrade().is_some()
	}
}
