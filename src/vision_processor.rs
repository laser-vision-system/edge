use std::{
	cell::RefCell,
	fmt,
	sync::{Arc, Mutex, OnceLock, Weak},
};

use image::RgbImage;
use server_edge_datatypes::shared::{CameraId, VisionTechnique};

use crate::logger::logger;

pub mod cnn;
pub mod demonstration;
pub mod diff;
pub mod empty;

pub use cnn::Cnn;
pub use demonstration::Demonstration;
pub use diff::Diff;
pub use empty::Empty;

pub type VisionRef = &'static VisionGuard;

static VISION: OnceLock<VisionGuard> = OnceLock::new();

pub fn vision() -> VisionRef {
	VISION.get_or_init(VisionGuard::new)
}

#[derive(Debug)]
pub struct VisionGuard {
	inner: Mutex<RefCell<Option<VisionVariant>>>,
}

impl VisionGuard {
	fn new() -> Self {
		Self {
			inner: Mutex::new(RefCell::new(Some(VisionVariant::new()))),
		}
	}

	pub fn swap(&self, swap_to: VisionTechnique) {
		let old_vision = self
			.inner
			.lock()
			.unwrap()
			.take()
			.expect("vision should never be None outside of this method");
		let new_vision = old_vision.swap(swap_to);
		self.inner.lock().unwrap().borrow_mut().replace(new_vision);
	}

	pub fn interact<F, R>(&self, func: F) -> R
	where
		F: FnOnce(&mut VisionVariant) -> R,
	{
		func(self.inner.lock().unwrap().borrow_mut().as_mut().expect(
			"vision should never be None outside of VisionGuard::swap_to",
		))
	}
}

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

/// A machine vision laser intensity measuring capable object.
///
/// Implement on a type to declare it as capable of performing machine vision
/// intensity measurement.
pub trait VisionInterface: fmt::Debug + fmt::Display {
	// fn new_state(&self) -> VisionState;

	/// Process all frames in a single batch, setting the return value in-place.
	fn process(&mut self, batch: &mut [BatchItem]);

	fn new_state(&self) -> VisionState;
}

#[derive(Debug)]
pub enum VisionVariant {
	Empty(Empty),
	Demonstration(Demonstration),
	Cnn(Cnn),
	Diff(Diff),
}

impl VisionVariant {
	/// Private constructor.
	fn new() -> Self {
		Self::Empty(Empty::default())
	}

	/// Request to switch this variant with another one.
	///
	/// If the requested type is the same as the current, this is a no-op.
	///
	/// Swaps the current vision processing technique to the chosen one.
	/// If the current technique is the same as the chosen one, returns None.
	/// This function must be called from the conductor where the necessary
	/// setpoint logic is implemented. The setpoint logic is not implemented
	/// here because it is not the responsibility of the vision processor to
	/// manage setpoints.
	///
	/// This method statically asserts exhaustiveness between
	/// [this](VisionVariant) and [`VisionTechnique`].
	fn swap(self, swap_to: VisionTechnique) -> Self {
		if self.identifier() == swap_to {
			logger()
				.lock()
				.unwrap()
				.log_warn("That VisionTechnique is already active.");
			return self;
		}

		logger()
			.lock()
			.unwrap()
			.log_info(&format!("Swapping VisionTechnique to: {swap_to:?}..."));
		match swap_to {
			VisionTechnique::Empty => Self::Empty(Empty::new()),
			VisionTechnique::Demonstration => {
				Self::Demonstration(Demonstration::new())
			},
			VisionTechnique::Cnn => Self::Cnn(Cnn::new()),
			VisionTechnique::Diff => Self::Diff(Diff::new()),
			_ => {
				logger().lock().unwrap().log_warn(
					"Attempting to swap to an unsupported vison technique",
				);
				self
			},
		}
	}

	/// A marker for which vision type this is.
	///
	/// Used when interacting with the server.
	fn identifier(&self) -> VisionTechnique {
		match self {
			Self::Empty(_) => VisionTechnique::Empty,
			Self::Demonstration(_) => VisionTechnique::Demonstration,
			Self::Cnn(_) => VisionTechnique::Cnn,
			Self::Diff(_) => VisionTechnique::Diff,
		}
	}
}

impl VisionInterface for VisionVariant {
	fn process(&mut self, batch: &mut [BatchItem]) {
		match self {
			Self::Empty(inner) => inner.process(batch),
			Self::Demonstration(inner) => inner.process(batch),
			Self::Cnn(inner) => inner.process(batch),
			Self::Diff(inner) => inner.process(batch),
		}
	}

	fn new_state(&self) -> VisionState {
		match self {
			Self::Empty(inner) => inner.new_state(),
			Self::Demonstration(inner) => inner.new_state(),
			Self::Cnn(inner) => inner.new_state(),
			Self::Diff(inner) => inner.new_state(),
		}
	}
}

// Pass-through implementation.
impl fmt::Display for VisionVariant {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Empty(empty) => empty.fmt(f),
			Self::Demonstration(demonstration) => demonstration.fmt(f),
			Self::Cnn(cnn) => cnn.fmt(f),
			Self::Diff(diff) => diff.fmt(f),
		}
	}
}

/// External state for the vision processor variants.
#[derive(Clone, Debug)]
pub enum VisionState {
	Empty {
		handle: SessionHandle,
		data: empty::ExternalData,
	},
	Demonstration {
		handle: SessionHandle,
		data: demonstration::ExternalData,
	},
	Cnn {
		handle: SessionHandle,
		data: cnn::ExternalData,
	},
	Diff {
		handle: SessionHandle,
		data: diff::ExternalData,
	},
}

impl VisionState {
	/// Returns true only if the owning vision processor variant is still
	/// active.
	pub fn is_valid(&self) -> bool {
		match self {
			Self::Empty { handle, .. }
			| Self::Demonstration { handle, .. }
			| Self::Cnn { handle, .. }
			| Self::Diff { handle, .. } => handle.is_valid(),
		}
	}

	pub fn get_deviation(&self) -> Option<f32> {
		match self {
			Self::Empty { .. } => None,
			Self::Demonstration { data, .. } => data.deviation,
			Self::Cnn { data, .. } => data.deviation,
			Self::Diff { data, .. } => data.deviation,
		}
	}
}

impl Default for VisionState {
	fn default() -> Self {
		vision().interact(|vision| vision.new_state())
	}
}

/// A single record that is passed into `process_frame`.
///
/// Currently holds a reference to the vision state, but should probably be
/// swapped to using an [`mpsc`](std::sync::mpsc) for multithreading. The
/// [`VisionState`] would be sent back through it.
pub struct BatchItem<'a> {
	pub camera_id: CameraId,
	pub state: &'a mut VisionState,
	pub image: RgbImage,
	pub calibrate: bool,
	pub send_to_server: bool,
}

impl<'a> BatchItem<'a> {
	pub fn new(
		camera_id: CameraId,
		state: &'a mut VisionState,
		image: RgbImage,
		calibrate: bool,
		send_to_server: bool,
	) -> Self {
		Self {
			camera_id,
			state,
			image,
			calibrate,
			send_to_server,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn swap() {
		let mut variant = VisionVariant::new();
		variant = variant.swap(VisionTechnique::Demonstration);
		println!("Current variant: {}", variant)
	}
}
