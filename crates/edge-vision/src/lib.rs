//! The vision capabilities of the Edge system.

#![cfg_attr(feature = "cnn-backend-wgpu", recursion_limit = "256")]
#![cfg_attr(
	test,
	allow(
		clippy::print_stdout,
		clippy::print_stderr,
		clippy::use_debug,
		clippy::panic,
		clippy::unwrap_used,
		clippy::unwrap_in_result,
		clippy::expect_used,
		clippy::indexing_slicing,
		reason = "Tests are allowed to perform all of these operations"
	)
)]

// Verify that only one backend feature flag is enabled at once.
mutually_exclusive_features::exactly_one_of!(
	"cnn-backend-flex",
	"cnn-backend-cuda",
	"cnn-backend-wgpu"
);

// Verify that only one model loading feature flag is enabled at once.
mutually_exclusive_features::exactly_one_of!(
	"cnn-model-embedded",
	"cnn-model-file",
	"cnn-model-zero-copy"
);

pub mod batch_item;
pub mod error;
pub mod session;
pub mod vision_interface;
pub mod vision_state;
pub mod vision_variant;

use std::{
	cell::RefCell,
	fmt,
	sync::{
		Mutex,
		OnceLock,
	},
};

use api::shared::VisionTechnique;
use edge_core::logger::logger;

use self::{
	error::EdgeVisionResult,
	vision_variant::VisionVariant,
};

static VISION: OnceLock<VisionGuard> = OnceLock::new();

/// Get exclusive access to the [`VisionGuard`].
pub fn vision() -> &'static VisionGuard {
	VISION.get_or_init(VisionGuard::new)
}

/// A thread-safe protection over the [`VisionVariant`] type.
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

	/// Swap to a different [`VisionTechnique`].
	///
	/// # Errors
	///
	/// If attempting to swap to the CNN model, which could not be loaded.
	///
	/// # Panics
	///
	/// This should never panic unless the code was modified in a way which did
	/// not respect the internal swapping control.
	#[expect(
		clippy::expect_used,
		reason = "Closely protected internal option type"
	)]
	pub fn swap(
		&self,
		swap_to: VisionTechnique,
	) -> EdgeVisionResult<()> {
		let inner = self.inner.lock().unwrap_or_else(|err| {
			logger().log_warn("Vision mutex was poisoned");
			err.into_inner()
		});
		let mut vision = inner.borrow_mut();
		vision
			.as_mut()
			.expect("vision should never be None outside of this method")
			.swap(swap_to)
	}

	/// Call a function on the internal [`VisionVariant`].
	///
	/// # Panics
	///
	/// This should never panic unless the code was modified in a way which did
	/// not respect the internal swapping control.
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
			logger().log_warn("Vision mutex was poisoned");
			err.into_inner()
		});
		let mut vision = inner.borrow_mut();
		let variant = vision
			.as_mut()
			.expect("vision should never be None outside of VisionGuard::swap");
		func(variant)
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
