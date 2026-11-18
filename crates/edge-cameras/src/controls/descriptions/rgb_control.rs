//! Defines for a camera control that consists of a `value`, `default`, and
//! `max`.

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
		ControlMax,
		ControlValue as _,
	},
	types::ConTypeRGB,
	values::{
		RgbControlValue,
		RgbControlValueType,
	},
};
use crate::{
	controls::write_flags,
	error::{
		EdgeCamerasError,
		EdgeCamerasResult,
	},
};

/// A camera control that consists of a `value`, `default`, and `max`.
///
/// The assumption is that the min will always be 0.
///
/// Maps to [`ControlValueDescription::RGB`].
///
/// [`ControlValueDescription::RGB`]: nokhwa::utils::ControlValueDescription::RGB
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RGBControl {
	control_type: KnownCameraControl,
	value: RgbControlValue,
	default: RgbControlValueType,
	max: RgbControlValueType,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	flags: Vec<KnownCameraControlFlag>,
}

impl RGBControl {
	/// Create a new [`RGBControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: RgbControlValue,
		default: RgbControlValueType,
		max: RgbControlValueType,
		flags: Vec<KnownCameraControlFlag>,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			max,
			flags,
		}
	}
}

impl ControlDescription for RGBControl {
	type ConType = ConTypeRGB;
	type ValueType = RgbControlValue;

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

impl ControlFlags for RGBControl {
	fn control_flags(&self) -> &[KnownCameraControlFlag] {
		&self.flags
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
				"{} {} *{} <{} ",
				self.control_type, self.value, self.default, self.max
			)?;
			write_flags(f, &self.flags)?;
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '{}' max: '{}' flags: '",
				self.control_type, self.value, self.default, self.max
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
		let value = RgbControlValue::from((0.0, 0.0, 0.0));
		let default = ConTypeRGB::from((0.0, 0.0, 0.0));
		let max = ConTypeRGB::from((0.0, 0.0, 0.0));
		let flags = vec![KnownCameraControlFlag::Disabled];
		let control = RGBControl::new(control_type, value, default, max, flags);
		super::super::tests::display_with_different_formats(
			"RGBControl",
			&control,
		);
	}
}
