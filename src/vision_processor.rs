// TODO: Remove all unwraps and then this.
#![expect(clippy::unwrap_used, reason = "WIP development")]

pub mod avg_lightness;
pub mod cnn;
pub mod diff;
pub mod inactive;

use std::{
	cell::RefCell,
	fmt,
	sync::{
		Arc,
		Mutex,
		OnceLock,
		Weak,
	},
};

pub use avg_lightness::AvgLightness;
pub use cnn::Cnn;
pub use diff::Diff;
use image::RgbImage;
pub use inactive::Inactive;
use server_edge_datatypes::shared::{
	CameraId,
	VisionTechnique,
};
use thiserror::Error;

use crate::{
	error::Result,
	logger::logger,
	tcam_interface::Image,
};

#[derive(Debug, Error)]
pub enum VisionError {
	#[error(transparent)]
	Cnn(#[from] cnn::CnnError),
}

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

	#[expect(
		clippy::expect_used,
		reason = "Closely protected internal option type"
	)]
	pub fn swap(
		&self,
		swap_to: VisionTechnique,
	) -> Result<()> {
		let inner = self.inner.lock().unwrap_or_else(|err| {
			logger()
				.lock()
				.unwrap()
				.log_warn("Vision mutex was poisoned");
			err.into_inner()
		});
		let mut vision = inner.borrow_mut();
		vision
			.as_mut()
			.expect("vision should never be None outside of this method")
			.swap(swap_to)
	}

	#[expect(
		clippy::expect_used,
		reason = "Closely protected internal option type"
	)]
	pub fn interact<F, R>(
		&self,
		func: F,
	) -> R
	where
		F: FnOnce(&mut VisionVariant) -> R,
	{
		let inner = self.inner.lock().unwrap_or_else(|err| {
			logger()
				.lock()
				.unwrap()
				.log_warn("Vision mutex was poisoned");
			err.into_inner()
		});
		let mut vision = inner.borrow_mut();
		let variant = vision
			.as_mut()
			.expect("vision should never be None outside of VisionGuard::swap");
		func(variant)
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
	/// Process all frames in a single batch, setting the return value in-place.
	fn process(
		&mut self,
		batch: &mut [BatchItem],
	);

	/// Get a new [`VisionState`] for this technique.
	fn new_state(&self) -> VisionState;

	/// Make sure the batch item has a valid [`VisionState`].
	fn validate_batch(
		&self,
		batch: &mut BatchItem,
	) {
		if !batch.state.is_valid() {
			*batch.state = self.new_state();
		}
	}
}

#[derive(Debug)]
pub enum VisionVariant {
	Inactive(Inactive),
	AvgLightness(AvgLightness),
	Cnn(Box<Cnn>),
	Diff(Diff),
}

impl VisionVariant {
	/// Private constructor.
	fn new() -> Self {
		Self::Inactive(Inactive::default())
	}

	/// Request to switch this variant with another one.
	///
	/// If the requested type is the same as the current, this is a no-op.
	///
	/// Swaps the current vision processing technique to the chosen one.
	/// A failed CNN model load leaves the current technique active.
	/// This function must be called from the conductor where the necessary
	/// setpoint logic is implemented. The setpoint logic is not implemented
	/// here because it is not the responsibility of the vision processor to
	/// manage setpoints.
	///
	/// This method statically asserts exhaustiveness between
	/// [this](VisionVariant) and [`VisionTechnique`].
	#[expect(clippy::unwrap_in_result, reason = "Logger")]
	fn swap(
		&mut self,
		swap_to: VisionTechnique,
	) -> Result<()> {
		if self.identifier() == swap_to {
			logger()
				.lock()
				.unwrap()
				.log_info(&format!("{swap_to:?} is already active"));
			return Ok(());
		}

		logger()
			.lock()
			.unwrap()
			.log_info(&format!("Swapping to: {swap_to:?}"));
		match swap_to {
			VisionTechnique::Inactive => {
				*self = Self::Inactive(Inactive::new())
			},
			VisionTechnique::AvgLightness => {
				*self = Self::AvgLightness(AvgLightness::new());
			},
			VisionTechnique::Cnn => *self = Self::Cnn(Box::new(Cnn::new()?)),
			VisionTechnique::Diff => *self = Self::Diff(Diff::new()),
			_ => {
				logger().lock().unwrap().log_warn(
					"Attempting to swap to an unsupported vison technique, nothing changes",
				);
			},
		}
		Ok(())
	}

	/// A marker for which vision type this is.
	///
	/// Used when interacting with the server.
	fn identifier(&self) -> VisionTechnique {
		match self {
			Self::Inactive(_) => VisionTechnique::Inactive,
			Self::AvgLightness(_) => VisionTechnique::AvgLightness,
			Self::Cnn(_) => VisionTechnique::Cnn,
			Self::Diff(_) => VisionTechnique::Diff,
		}
	}
}

impl VisionInterface for VisionVariant {
	fn process(
		&mut self,
		batch: &mut [BatchItem],
	) {
		if batch.is_empty() {
			return;
		}

		match self {
			Self::Inactive(inner) => inner.process(batch),
			Self::AvgLightness(inner) => inner.process(batch),
			Self::Cnn(inner) => inner.process(batch),
			Self::Diff(inner) => inner.process(batch),
		}

		let process_results = batch
			.iter()
			.map(ToString::to_string)
			.collect::<Vec<_>>()
			.join("\n");
		logger().lock().unwrap().log_info(&process_results);
	}

	fn new_state(&self) -> VisionState {
		match self {
			Self::Inactive(inner) => inner.new_state(),
			Self::AvgLightness(inner) => inner.new_state(),
			Self::Cnn(inner) => inner.new_state(),
			Self::Diff(inner) => inner.new_state(),
		}
	}
}

// Pass-through implementation.
impl fmt::Display for VisionVariant {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		match self {
			Self::Inactive(empty) => empty.fmt(f),
			Self::AvgLightness(demonstration) => demonstration.fmt(f),
			Self::Cnn(cnn) => cnn.fmt(f),
			Self::Diff(diff) => diff.fmt(f),
		}
	}
}

/// External state for the vision processor variants.
#[derive(Clone, Debug)]
pub enum VisionState {
	Inactive {
		handle: SessionHandle,
		data: inactive::ExternalData,
	},
	AvgLightness {
		handle: SessionHandle,
		data: avg_lightness::ExternalData,
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
			Self::Inactive {
				handle, ..
			}
			| Self::AvgLightness {
				handle, ..
			}
			| Self::Cnn {
				handle, ..
			}
			| Self::Diff {
				handle, ..
			} => handle.is_valid(),
		}
	}

	pub fn get_deviation(&self) -> Option<f32> {
		match self {
			Self::Inactive {
				..
			} => None,
			Self::AvgLightness {
				data, ..
			} => data.deviation,
			Self::Cnn {
				data, ..
			} => data.deviation,
			Self::Diff {
				data, ..
			} => data.deviation,
		}
	}
}

impl Default for VisionState {
	fn default() -> Self {
		vision().interact(|vision| vision.new_state())
	}
}

impl fmt::Display for VisionState {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		match self {
			Self::Inactive {
				data, ..
			} => data.fmt(f),
			Self::AvgLightness {
				data, ..
			} => data.fmt(f),
			Self::Cnn {
				data, ..
			} => data.fmt(f),
			Self::Diff {
				data, ..
			} => data.fmt(f),
		}
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
	pub image: Image,
	pub calibrate: bool,
	pub send_to_server: bool,
}

impl<'a> BatchItem<'a> {
	pub fn new(
		camera_id: CameraId,
		state: &'a mut VisionState,
		image: Image,
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

impl fmt::Display for BatchItem<'_> {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		writeln!(
			f,
			"Batch Item: [\n\tID: {}\n\tState: {}\n\tImage\n\tCalibrate: {}\n\tSend: {}\n]",
			self.camera_id, self.state, self.calibrate, self.send_to_server
		)
	}
}

/// Helper function for writing optional values for display.
fn write_optional<V>(
	fmtr: &mut fmt::Formatter<'_>,
	value: Option<V>,
) -> fmt::Result
where
	V: fmt::Display,
{
	match value {
		Some(value) => write!(fmtr, "{value}"),
		None => fmtr.write_str("None"),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn swap() {
		let mut variant = VisionVariant::new();
		variant
			.swap(VisionTechnique::AvgLightness)
			.expect("demonstration swap should succeed");
		println!("Current variant: {variant}");
	}
}
