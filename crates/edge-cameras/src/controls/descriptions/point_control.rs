//! Defines for a camera control that consists of a `value` and `default`.

use std::fmt;

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

impl fmt::Display for PointControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(f, "{} {} *{}", self.control_type, self.value, self.default)
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '{}'",
				self.control_type, self.value, self.default
			)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore = "for manual inspection"]
	fn control_display() {
		let control_type = KnownCameraControl::Pan;
		let value = ConValuePoint::from((0.0, 0.0));
		let default = ConTypePoint::from((0.0, 0.0));
		let control = PointControl::new(control_type, value, default);
		super::super::tests::display_with_different_formats(
			"PointControl",
			control,
		);
	}
}
