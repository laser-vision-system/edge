//! Defines for a camera control that consists of a `value`, `default`, `step`,
//! `min`, and `max`.

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

impl ControlDefault for IntegerRangeControl {
	type DefaultType = Self::ConType;

	fn control_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlStep for IntegerRangeControl {
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

impl ControlMin for IntegerRangeControl {
	type MinType = Self::ConType;

	fn control_min(&self) -> Self::MinType {
		self.min
	}
}

impl ControlMax for IntegerRangeControl {
	type MaxType = Self::ConType;

	fn control_max(&self) -> Self::MaxType {
		self.max
	}
}

impl fmt::Display for IntegerRangeControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(
				f,
				"{} {} *{} >{} <{} %{}",
				self.control_type,
				self.value,
				self.default,
				self.min,
				self.max,
				self.step
			)
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '{}' min: '{}' max: '{}' step: '{}'",
				self.control_type,
				self.value,
				self.default,
				self.min,
				self.max,
				self.step
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
		let value = ConValueInteger::from(0);
		let default = 0;
		let step = 1;
		let min = -1;
		let max = 1;
		let control = IntegerRangeControl::new(
			control_type,
			value,
			default,
			step,
			min,
			max,
		);
		super::super::tests::display_with_different_formats(
			"IntegerRangeControl",
			&control,
		);
	}
}
