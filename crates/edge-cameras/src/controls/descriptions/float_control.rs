//! Defines for a camera control that consists of a `value`, `default`, and
//! `step`.

use nokhwa::utils::KnownCameraControl;
use serde::{
	Deserialize,
	Serialize,
};

use super::super::{
	traits::{
		ControlDefault,
		ControlDescription,
		ControlStep,
		ControlValue as _,
	},
	types::ConTypeFloat,
	values::{
		ConValueFloat,
		ConValueFloatType,
	},
};
use crate::error::{
	EdgeCamerasError,
	EdgeCamerasResult,
};

/// A camera control that consists of a `value`, `default`, and `step`.
///
/// Maps to [`ControlValueDescription::Float`].
///
/// [`ControlValueDescription::Float`]: nokhwa::utils::ControlValueDescription::Float
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct FloatControl {
	control_type: KnownCameraControl,
	value: ConValueFloat,
	default: ConValueFloatType,
	step: ConValueFloatType,
}

impl FloatControl {
	/// Create a new [`FloatControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueFloat,
		default: ConValueFloatType,
		step: ConValueFloatType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			step,
		}
	}
}

impl ControlDescription for FloatControl {
	type ConType = ConTypeFloat;
	type ValueType = ConValueFloat;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(
		&mut self,
		value: Self::ValueType,
	) -> EdgeCamerasResult<()> {
		if self.valid(value.get_value()) {
			self.value = value;
			Ok(())
		} else {
			Err(EdgeCamerasError::InvalidControlValue)
		}
	}

	fn get_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for FloatControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid");
	}
}

impl ControlStep for FloatControl {
	// Default should be based on ValueType, so this makes sure they can be
	// compared.
	type StepType = Self::ConType;

	fn get_step(&self) -> Self::StepType {
		self.step
	}

	fn valid(
		&self,
		value: Self::StepType,
	) -> bool {
		(value - self.get_default()).abs() % self.get_step() == 0.0
			|| (value - self.get_value().get_value()).abs() % self.get_step()
				== 0.0
	}
}
