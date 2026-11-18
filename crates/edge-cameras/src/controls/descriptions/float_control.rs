//! Defines for a camera control that consists of a `value`, `default`, and
//! `step`.

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
		ControlStep,
		ControlValue as _,
	},
	types::ControlTypeFloat,
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

/// A camera control that consists of a `value`, `default`, and `step`.
///
/// Maps to [`ControlValueDescription::Float`].
///
/// [`ControlValueDescription::Float`]: nokhwa::utils::ControlValueDescription::Float
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FloatControl {
	control_type: KnownCameraControl,
	value: FloatControlValue,
	default: FloatControlValueType,
	step: FloatControlValueType,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	flags: Vec<KnownCameraControlFlag>,
}

impl FloatControl {
	/// Create a new [`FloatControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: FloatControlValue,
		default: FloatControlValueType,
		step: FloatControlValueType,
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

impl ControlDescription for FloatControl {
	type ConType = ControlTypeFloat;
	type ValueType = FloatControlValue;

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

impl ControlDefault for FloatControl {
	type DefaultType = Self::ConType;

	fn control_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlStep for FloatControl {
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

impl ControlFlags for FloatControl {
	fn control_flags(&self) -> &[KnownCameraControlFlag] {
		&self.flags
	}
}

impl fmt::Display for FloatControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(
				f,
				"{} {} *{} %{} ",
				self.control_type, self.value, self.default, self.step
			)?;
			write_flags(f, &self.flags)?;
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '{}' step: '{}' flags: '",
				self.control_type, self.value, self.default, self.step
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
		let flags = vec![KnownCameraControlFlag::Disabled];
		let control =
			FloatControl::new(control_type, value, default, step, flags);
		super::super::tests::display_with_different_formats(
			"FloatControl",
			&control,
		);
	}
}
