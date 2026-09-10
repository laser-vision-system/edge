use std::{
	fmt,
	rc::{Rc, Weak},
};

use image::RgbImage;
use server_edge_datatypes::shared::{CameraId, VisionTechnique};

pub mod cnn;
pub mod demonstration;
pub mod diff;
pub mod empty;

pub use cnn::Cnn;
pub use demonstration::Demonstration;
pub use diff::Diff;
pub use empty::Empty;

// NOTE: If we do multithreading, this has to be swapped for `std::sync::Arc`.

#[derive(Clone)]
pub struct SessionToken(Rc<()>);

impl SessionToken {
	pub fn new() -> Self {
		Self(Rc::new(()))
	}

	pub fn downgrade(&self) -> SessionHandle {
		SessionHandle(Rc::downgrade(&self.0))
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
}

#[derive(Debug)]
pub enum VisionVariant {
	Empty(Empty),
	Demonstration(Demonstration),
	Cnn(Cnn),
	Diff(Diff),
}

impl VisionVariant {
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
	/// [this](VisionVariant) and [VisionTechnique].
	pub fn swap(self, swap_to: VisionTechnique) -> Self {
		if self.identifier() == swap_to {
			return self;
		}
		match swap_to {
			VisionTechnique::Empty => Self::Empty(Empty::new()),
			VisionTechnique::Demonstration => Self::Demonstration(Demonstration::new()),
			VisionTechnique::Cnn => Self::Cnn(Cnn::new()),
			VisionTechnique::Diff => Self::Diff(Diff::new()),
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
}

impl Default for VisionVariant {
	fn default() -> Self {
		Self::Empty(Default::default())
	}
}

// Pass-through implementation.
impl fmt::Display for VisionVariant {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Empty(empty) => write!(f, "{}", empty),
			Self::Demonstration(demonstration) => write!(f, "{}", demonstration),
			Self::Cnn(cnn) => write!(f, "{}", cnn),
			Self::Diff(diff) => write!(f, "{}", diff),
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
	/// Returns true only if the owning vision processor variant is still active.
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

/// A single record that is passed into `process_frame`.
///
/// Currently holds a reference to the vision state, but should probably be
/// swapped to using an [`mpsc`](std::sync::mpsc) for multithreading. The [`VisionState`] would be sent back through it.
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
		let mut variant = VisionVariant::default();
		variant = variant.swap(VisionTechnique::Demonstration);
		println!("Current variant: {}", variant)
	}
}
