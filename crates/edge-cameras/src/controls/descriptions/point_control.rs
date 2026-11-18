//! Defines for a camera control that consists of a `value` and `default`.

use std::fmt::{
	self,
	Write,
};

use nokhwa::utils::{
	KnownCameraControl,
	KnownCameraControlFlag,
};
use serde::{
	Deserialize,
	Serialize,
};

use super::super::{
	traits::{
		ControlDefault,
		ControlDescription,
		ControlFlags,
	},
	types::ConTypePoint,
	values::{
		PointControlValue,
		PointControlValueType,
	},
};
use crate::{
	controls::write_flags,
	error::EdgeCamerasResult,
};

/// A camera control that consists of a `value` and `default`.
///
/// Maps to [`ControlValueDescription::Point`].
///
/// [`ControlValueDescription::Point`]: nokhwa::utils::ControlValueDescription::Point
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PointControl {
	control_type: KnownCameraControl,
	value: PointControlValue,
	default: PointControlValueType,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	flags: Vec<KnownCameraControlFlag>,
}

impl PointControl {
	/// Create a new [`PointControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: PointControlValue,
		default: PointControlValueType,
		flags: Vec<KnownCameraControlFlag>,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			flags,
		}
	}
}

impl ControlDescription for PointControl {
	type ConType = ConTypePoint;
	type ValueType = PointControlValue;

	fn control_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn try_set_control_value(
		&mut self,
		value: Self::ValueType,
	) -> EdgeCamerasResult {
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

impl ControlFlags for PointControl {
	fn control_flags(&self) -> &[KnownCameraControlFlag] {
		&self.flags
	}
}

impl fmt::Display for PointControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(
				f,
				"{} {} *{} ",
				self.control_type, self.value, self.default
			)?;
			write_flags(f, &self.flags)?;
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '{}' flags: '",
				self.control_type, self.value, self.default
			)?;
			write_flags(f, &self.flags)?;
			f.write_char('\'')?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore = "for manual inspection"]
	fn control_display() {
		let control_type = KnownCameraControl::Pan;
		let value = PointControlValue::from((0.0, 0.0));
		let default = ConTypePoint::from((0.0, 0.0));
		let flags = vec![KnownCameraControlFlag::Disabled];
		let control = PointControl::new(control_type, value, default, flags);
		super::super::tests::display_with_different_formats(
			"PointControl",
			&control,
		);
	}
}
