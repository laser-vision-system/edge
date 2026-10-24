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
	types::ConTypeInt,
	values::{
		ConValueInteger,
		ConValueIntegerType,
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
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct IntegerControl {
	control_type: KnownCameraControl,
	value: ConValueInteger,
	default: ConValueIntegerType,
	step: ConValueIntegerType,
}

impl IntegerControl {
	/// Create a new [`IntegerControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueInteger,
		default: ConValueIntegerType,
		step: ConValueIntegerType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			step,
		}
	}
}

impl ControlDescription for IntegerControl {
	type ConType = ConTypeInt;
	type ValueType = ConValueInteger;

	fn control_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn try_set_control_value(
		&mut self,
		value: Self::ValueType,
	) -> EdgeCamerasResult<()> {
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
