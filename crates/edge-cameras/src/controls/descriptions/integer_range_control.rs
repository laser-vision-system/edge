//! Defines for a camera control that consists of a `value`, `default`, `step`,
//! `min`, and `max`.

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
		ControlMin,
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

/// A camera control that consists of a `value`, `default`, `step`, `min`, and
/// `max`.
///
/// Maps to [`ControlValueDescription::IntegerRange`].
///
/// [`ControlValueDescription::IntegerRange`]: nokhwa::utils::ControlValueDescription::IntegerRange
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct IntegerRangeControl {
	control_type: KnownCameraControl,
	value: ConValueInteger,
	default: ConValueIntegerType,
	step: ConValueIntegerType,
	min: ConValueIntegerType,
	max: ConValueIntegerType,
}

impl IntegerRangeControl {
	/// Create a new [`IntegerRangeControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueInteger,
		default: ConValueIntegerType,
		step: ConValueIntegerType,
		min: ConValueIntegerType,
		max: ConValueIntegerType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			step,
			min,
			max,
		}
	}
}

impl ControlDescription for IntegerRangeControl {
	type ConType = ConTypeInt;
	type ValueType = ConValueInteger;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(
		&mut self,
		value: Self::ValueType,
	) -> EdgeCamerasResult<()> {
		if ControlMin::valid(self, value.get_value())
			&& ControlMax::valid(self, value.get_value())
			&& ControlStep::valid(self, value.get_value())
		{
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

impl ControlDefault for IntegerRangeControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid");
	}
}

impl ControlStep for IntegerRangeControl {
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
			|| (value - self.get_value().get_value()).abs()
				% self.get_step()
				== 0
	}
}

impl ControlMin for IntegerRangeControl {
	type MinType = Self::ConType;

	fn get_min(&self) -> Self::MinType {
		self.min
	}
}

impl ControlMax for IntegerRangeControl {
	type MaxType = Self::ConType;

	fn get_max(&self) -> Self::MaxType {
		self.max
	}
}
