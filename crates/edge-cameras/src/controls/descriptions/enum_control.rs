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

	fn control_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn try_set_control_value(
		&mut self,
		value: Self::ValueType,
	) -> EdgeCamerasResult {
		self.value = value;
		Ok(())
	}

	fn control_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for EnumControl {
	type DefaultType = Self::ConType;

	fn control_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlPossible for EnumControl {
	type PossibleType = Vec<Self::ConType>;

	fn control_possible(&self) -> Self::PossibleType {
		self.possible.clone()
	}

	fn validate(
		&self,
		value: &Self::ConType,
	) -> bool {
		self.possible.contains(value)
	}
}
