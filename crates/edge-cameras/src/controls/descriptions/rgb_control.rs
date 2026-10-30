//! Defines for a camera control that consists of a `value`, `default`, and
//! `max`.

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
		ControlMax,
		ControlValue as _,
	},
	types::ConTypeRGB,
	values::{
		ConValueRGB,
		ConValueRGBType,
	},
};
use crate::error::{
	EdgeCamerasError,
	EdgeCamerasResult,
};

/// A camera control that consists of a `value`, `default`, and `max`.
///
/// The assumption is that the min will always be 0.
///
/// Maps to [`ControlValueDescription::RGB`].
///
/// [`ControlValueDescription::RGB`]: nokhwa::utils::ControlValueDescription::RGB
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RGBControl {
	control_type: KnownCameraControl,
	value: ConValueRGB,
	default: ConValueRGBType,
	max: ConValueRGBType,
}

impl RGBControl {
	/// Create a new [`RGBControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueRGB,
		default: ConValueRGBType,
		max: ConValueRGBType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			max,
		}
	}
}

impl ControlDescription for RGBControl {
	type ConType = ConTypeRGB;
	type ValueType = ConValueRGB;

	fn control_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn try_set_control_value(
		&mut self,
		value: Self::ValueType,
	) -> EdgeCamerasResult {
		if self.validate(value.get()) {
			self.value = value;
			Ok(())
		} else {
			Err(EdgeCamerasError::InvalidControlValue)
		}
	}

	fn control_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for RGBControl {
	type DefaultType = Self::ConType;

	fn control_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlMax for RGBControl {
	type MaxType = Self::ConType;

	fn control_max(&self) -> Self::MaxType {
		self.max
	}
}

impl fmt::Display for RGBControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(
				f,
				"{} {} *{} <{}",
				self.control_type, self.value, self.default, self.max
			)
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '{}' max: '{}'",
				self.control_type, self.value, self.default, self.max
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
		let value = ConValueRGB::from((0.0, 0.0, 0.0));
		let default = ConTypeRGB::from((0.0, 0.0, 0.0));
		let max = ConTypeRGB::from((0.0, 0.0, 0.0));
		let control = RGBControl::new(control_type, value, default, max);
		super::super::tests::display_with_different_formats(
			"RGBControl",
			&control,
		);
	}
}
