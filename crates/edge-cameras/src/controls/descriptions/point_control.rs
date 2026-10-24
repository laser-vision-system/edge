//! Defines for a camera control that consists of a `value` and `default`.

use nokhwa::utils::KnownCameraControl;
use serde::{
	Deserialize,
	Serialize,
};

use super::super::{
	traits::{
		ControlDefault,
		ControlDescription,
	},
	types::ConTypePoint,
	values::{
		ConValuePoint,
		ConValuePointType,
	},
};
use crate::error::EdgeCamerasResult;

/// A camera control that consists of a `value` and `default`.
///
/// Maps to [`ControlValueDescription::Point`].
///
/// [`ControlValueDescription::Point`]: nokhwa::utils::ControlValueDescription::Point
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PointControl {
	control_type: KnownCameraControl,
	value: ConValuePoint,
	default: ConValuePointType,
}

impl PointControl {
	/// Create a new [`PointControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValuePoint,
		default: ConValuePointType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlDescription for PointControl {
	type ConType = ConTypePoint;
	type ValueType = ConValuePoint;

	fn control_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn try_set_control_value(
		&mut self,
		value: Self::ValueType,
	) -> EdgeCamerasResult<()> {
		self.value = value;
		Ok(())
	}

	fn control_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for PointControl {
	type DefaultType = Self::ConType;

	fn control_default(&self) -> Self::DefaultType {
		self.default
	}
}
