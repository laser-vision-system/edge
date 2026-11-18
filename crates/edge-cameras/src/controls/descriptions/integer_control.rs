//! Defines for a camera control that consists of a `value`, `default`, and
//! `step`.

use std::fmt;

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
		ControlStep,
		ControlValue as _,
	},
	types::ConTypeInt,
	values::{
		IntegerControlValue,
		IntegerControlValueType,
	},
};
use crate::error::{
	EdgeCamerasError,
	EdgeCamerasResult,
};

/// A camera control that consists of a `value`, `default`, and `step`.
///
/// Maps to [`ControlValueDescription::Integer`].
///
/// [`ControlValueDescription::Integer`]: nokhwa::utils::ControlValueDescription::Integer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntegerControl {
	control_type: KnownCameraControl,
	value: IntegerControlValue,
	default: IntegerControlValueType,
	step: IntegerControlValueType,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	flags: Vec<KnownCameraControlFlag>,
}

impl IntegerControl {
	/// Create a new [`IntegerControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: IntegerControlValue,
		default: IntegerControlValueType,
		step: IntegerControlValueType,
		flags: Vec<KnownCameraControlFlag>,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			step,
			flags,
		}
	}
}

impl ControlDescription for IntegerControl {
	type ConType = ConTypeInt;
	type ValueType = IntegerControlValue;

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

impl ControlDefault for IntegerControl {
	type DefaultType = Self::ConType;

	fn control_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlStep for IntegerControl {
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
		(value - self.control_default()).abs() % self.control_step() == 0
			|| (value - self.control_value().get()).abs() % self.control_step()
				== 0
	}
}

impl ControlFlags for IntegerControl {
	fn control_flags(&self) -> &[KnownCameraControlFlag] {
		&self.flags
	}
}

impl fmt::Display for IntegerControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(
				f,
				"{} {} *{} %{}",
				self.control_type, self.value, self.default, self.step
			)?;
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '{}' step: '{}'",
				self.control_type, self.value, self.default, self.step
			)?;
		}
		super::write_flags(f, &self.flags)?;
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
		let value = IntegerControlValue::from(0);
		let default = 0;
		let step = 1;
		let control =
			IntegerControl::new(control_type, value, default, step, Vec::new());
		super::super::tests::display_with_different_formats(
			"IntegerControl",
			&control,
		);
	}
}
