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
