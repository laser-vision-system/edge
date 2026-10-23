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
	Inactive(SessionHandle, inactive::ExternalData),
	AvgLightness(SessionHandle, avg_lightness::ExternalData),
	Cnn(SessionHandle, cnn::ExternalData),
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
