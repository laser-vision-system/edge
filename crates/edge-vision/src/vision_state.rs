//! Variant specific data that is provided to a camera.

use std::fmt;

use crate::{
	session::SessionHandle,
	vision,
	vision_interface::VisionInterface as _,
	vision_variant::{
		avg_lightness,
		cnn,
		diff,
		inactive,
	},
};

/// External state for the vision processor variants.
#[derive(Clone, Debug)]
pub enum VisionState {
	/// Associated data for the [`Inactive`] variant.
	///
	/// Should not be directly interacted with.
	///
	/// [`Inactive`]: crate::vision_variant::VisionVariant::Inactive
	Inactive(SessionHandle, inactive::ExternalData),
	/// Associated data for the [`AvgLightness`] variant.
	///
	/// Should not be directly interacted with.
	///
	/// [`AvgLightness`]: crate::vision_variant::VisionVariant::AvgLightness
	AvgLightness(SessionHandle, avg_lightness::ExternalData),
	/// Associated data for the [`Cnn`] variant.
	///
	/// Should not be directly interacted with.
	///
	/// [`Cnn`]: crate::vision_variant::VisionVariant::Cnn
	Cnn(SessionHandle, cnn::ExternalData),
	/// Associated data for the [`Diff`] variant.
	///
	/// Should not be directly interacted with.
	///
	/// [`Diff`]: crate::vision_variant::VisionVariant::Diff
	Diff(SessionHandle, diff::ExternalData),
}

impl VisionState {
	/// Returns true only if the owning vision processor variant is still
	/// active.
	pub fn is_valid(&self) -> bool {
		match self {
			Self::Inactive(handle, _)
			| Self::AvgLightness(handle, _)
			| Self::Cnn(handle, _)
			| Self::Diff(handle, _) => handle.is_valid(),
		}
	}

	/// Return the current deviation value, if it exists.
	pub fn get_deviation(&self) -> Option<f32> {
		match self {
			Self::Inactive(..) => None,
			Self::AvgLightness(_, data) => data.deviation,
			Self::Cnn(_, data) => data.deviation,
			Self::Diff(_, data) => data.deviation,
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
			Self::Inactive(_, data) => data.fmt(f),
			Self::AvgLightness(_, data) => data.fmt(f),
			Self::Cnn(_, data) => data.fmt(f),
			Self::Diff(_, data) => data.fmt(f),
		}
	}
}
