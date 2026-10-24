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

impl ControlDefault for IntegerControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid");
	}
}

impl ControlStep for IntegerControl {
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
		(value - self.get_default()).abs() % self.get_step() == 0
			|| (value - self.get_value().get_value()).abs() % self.get_step()
				== 0
	}
}
