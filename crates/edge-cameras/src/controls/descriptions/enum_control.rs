//! Defines for a camera control that consists of a `value`, `default`, and a
//! vector of `possible` options.
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
		ControlPossible,
	},
	types::ConTypeEnum,
	values::{
		ConValueEnumType,
		EnumControlValue,
	},
};
use crate::{
	controls::types::ConTypePossible,
	error::EdgeCamerasResult,
};

/// A camera control that consists of a `value`, `default`, and a vector of
/// `possible` options.
///
/// Maps to [`ControlValueDescription::Enum`].
///
/// [`ControlValueDescription::Enum`]: nokhwa::utils::ControlValueDescription::Enum
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumControl {
	control_type: KnownCameraControl,
	value: EnumControlValue,
	default: ConValueEnumType,
	possible: ConTypePossible,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	flags: Vec<KnownCameraControlFlag>,
}

impl EnumControl {
	/// Create a new [`EnumControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: EnumControlValue,
		default: ConValueEnumType,
		possible: ConTypePossible,
		flags: Vec<KnownCameraControlFlag>,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			possible,
			flags,
		}
	}
}

impl ControlDescription for EnumControl {
	type ConType = ConTypeEnum;
	type ValueType = EnumControlValue;

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
	type PossibleType = ConTypePossible;

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

impl ControlFlags for EnumControl {
	fn control_flags(&self) -> &[KnownCameraControlFlag] {
		&self.flags
	}
}

impl fmt::Display for EnumControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(
				f,
				"{} {} *{} {}",
				self.control_type, self.value, self.default, self.possible
			)?;
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '{}' possible: '{}'",
				self.control_type, self.value, self.default, self.possible
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
		let value = EnumControlValue::from(0);
		let default = 0;
		let possible = ConTypePossible::from(vec![0; 5]);
		let control = EnumControl::new(
			control_type,
			value,
			default,
			possible,
			Vec::new(),
		);
		super::super::tests::display_with_different_formats(
			"EnumControl",
			&control,
		);
	}
}
