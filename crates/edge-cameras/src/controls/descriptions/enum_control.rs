//! Defines for a camera control that consists of a `value`, `default`, and a
//! vector of `possible` options.
use nokhwa::utils::KnownCameraControl;
use serde::{
	Deserialize,
	Serialize,
};

use super::super::{
	traits::{
		ControlDefault,
		ControlDescription,
		ControlPossible,
	},
	types::ConTypeEnum,
	values::{
		ConValueEnum,
		ConValueEnumType,
	},
};
use crate::error::EdgeCamerasResult;

/// A camera control that consists of a `value`, `default`, and a vector of
/// `possible` options.
///
/// Maps to [`ControlValueDescription::Enum`].
///
/// [`ControlValueDescription::Enum`]: nokhwa::utils::ControlValueDescription::Enum
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumControl {
	control_type: KnownCameraControl,
	value: ConValueEnum,
	default: ConValueEnumType,
	possible: Vec<ConValueEnumType>,
}

impl EnumControl {
	/// Create a new [`EnumControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueEnum,
		default: ConValueEnumType,
		possible: Vec<ConValueEnumType>,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			possible,
		}
	}
}

impl ControlDescription for EnumControl {
	type ConType = ConTypeEnum;
	type ValueType = ConValueEnum;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(
		&mut self,
		value: Self::ValueType,
	) -> EdgeCamerasResult<()> {
		self.value = value;
		Ok(())
	}

	fn get_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for EnumControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid");
	}
}

impl ControlPossible for EnumControl {
	type PossibleType = Vec<Self::ConType>;

	fn get_possible(&self) -> Self::PossibleType {
		self.possible.clone()
	}

	fn valid(
		&self,
		value: &Self::ConType,
	) -> bool {
		self.possible.contains(value)
	}
}
