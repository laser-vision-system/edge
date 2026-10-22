#![expect(
	clippy::new_without_default,
	reason = "Constructing should be hidden"
)]

pub mod avg_lightness;
pub mod cnn;
pub mod diff;
pub mod inactive;

use std::fmt;

use api::shared::VisionTechnique;
use edge_core::logger::logger;

pub use self::{
	avg_lightness::AvgLightness,
	cnn::Cnn,
	diff::Diff,
	inactive::Inactive,
};
use crate::{
	batch_item::BatchItem,
	error::EdgeVisionResult,
	vision_interface::VisionInterface,
	vision_state::VisionState,
};

/// This type should not be visible outside of this crate.
#[derive(Debug)]
pub enum VisionVariant {
	Inactive(Inactive),
	AvgLightness(AvgLightness),
	Cnn(Box<Cnn>),
	Diff(Diff),
}

impl VisionVariant {
	pub fn new() -> Self {
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
	pub fn swap(
		&mut self,
		swap_to: VisionTechnique,
	) -> EdgeVisionResult<()> {
		if self.identifier() == swap_to {
			logger().log_info(&format!("{swap_to:?} is already active"));
			return Ok(());
		}

		logger().log_info(&format!("Swapping to: {swap_to:?}"));
		match swap_to {
			VisionTechnique::Inactive => {
				*self = Self::Inactive(Inactive::new());
			},
			VisionTechnique::AvgLightness => {
				*self = Self::AvgLightness(AvgLightness::new());
			},
			VisionTechnique::Cnn => *self = Self::Cnn(Box::new(Cnn::new()?)),
			VisionTechnique::Diff => *self = Self::Diff(Diff::new()),
			_ => {
				logger().log_warn(
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
		logger().log_info(&process_results);
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
