//! Defines for a camera control that consists of a `value`, `default`, `step`,
//! `min`, and `max`.

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
		ControlMin,
		ControlStep,
		ControlValue as _,
	},
	types::ConTypeFloat,
	values::{
		FloatControlValue,
		FloatControlValueType,
	},
};
use crate::{
	controls::write_flags,
	error::{
		EdgeCamerasError,
		EdgeCamerasResult,
	},
};

/// A camera control that consists of a `value`, `default`, `step`, `min`, and
/// `max`.
///
/// Maps to [`ControlValueDescription::FloatRange`].
///
/// [`ControlValueDescription::FloatRange`]: nokhwa::utils::ControlValueDescription::FloatRange
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FloatRangeControl {
	control_type: KnownCameraControl,
	value: FloatControlValue,
	default: FloatControlValueType,
	step: FloatControlValueType,
	min: FloatControlValueType,
	max: FloatControlValueType,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	flags: Vec<KnownCameraControlFlag>,
}

impl FloatRangeControl {
	/// Create a new [`FloatRangeControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: FloatControlValue,
		default: FloatControlValueType,
		step: FloatControlValueType,
		min: FloatControlValueType,
		max: FloatControlValueType,
		flags: Vec<KnownCameraControlFlag>,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			step,
			min,
			max,
			flags,
		}
	}
}

impl ControlDescription for FloatRangeControl {
	type ConType = ConTypeFloat;
	type ValueType = FloatControlValue;

	fn control_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn try_set_control_value(
		&mut self,
		value: Self::ValueType,
	) -> EdgeCamerasResult {
		if ControlMin::validate(self, value.get())
			&& ControlMax::validate(self, value.get())
			&& ControlStep::validate(self, value.get())
		{
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

impl ControlDefault for FloatRangeControl {
	type DefaultType = Self::ConType;

	fn control_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlStep for FloatRangeControl {
	// Default should be based on ValueType, so this makes sure they can be
	// compared.
	type StepType = Self::ConType;

	fn control_step(&self) -> Self::StepType {
		self.step
	}

	fn validate(
		&self,
		value: Self::StepType,
	) -> bool {
		(value - self.control_default()).abs() % self.control_step() == 0.0
			|| (value - self.control_value().get()).abs() % self.control_step()
				== 0.0
	}
}

impl ControlMin for FloatRangeControl {
	type MinType = Self::ConType;

	fn control_min(&self) -> Self::MinType {
		self.min
	}
}

impl ControlMax for FloatRangeControl {
	type MaxType = Self::ConType;

	fn control_max(&self) -> Self::MaxType {
		self.max
	}
}

impl ControlFlags for FloatRangeControl {
	fn control_flags(&self) -> &[KnownCameraControlFlag] {
		&self.flags
	}
}

impl fmt::Display for FloatRangeControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(
				f,
				"{} {} *{} >{} <{} %{} ",
				self.control_type,
				self.value,
				self.default,
				self.min,
				self.max,
				self.step
			)?;
			write_flags(f, &self.flags)?;
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '{}' min: '{}' max: '{}' step: '{}' flags: '",
				self.control_type,
				self.value,
				self.default,
				self.min,
				self.max,
				self.step
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
		let value = FloatControlValue::from(0.0);
		let default = 0.0;
		let step = 1.0;
		let min = -1.0;
		let max = 1.0;
		let flags = vec![KnownCameraControlFlag::Disabled];
		let control = FloatRangeControl::new(
			control_type,
			value,
			default,
			step,
			min,
			max,
			flags,
		);
		super::super::tests::display_with_different_formats(
			"FloatRangeControl",
			&control,
		);
	}
}
