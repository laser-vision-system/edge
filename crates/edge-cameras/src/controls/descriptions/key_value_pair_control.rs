//! Defines for a camera control that consists of a `value` and `default`.

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
	},
	types::ConTypeKeyValue,
	values::{
		ConValueKeyValue,
		ConValueKeyValueType,
	},
};
use crate::error::EdgeCamerasResult;

/// A camera control that consists of a `value` and `default`.
///
/// Maps to [`ControlValueDescription::KeyValuePair`].
///
/// [`ControlValueDescription::KeyValuePair`]: nokhwa::utils::ControlValueDescription::KeyValuePair
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct KeyValuePairControl {
	control_type: KnownCameraControl,
	value: ConValueKeyValue,
	default: ConValueKeyValueType,
}

impl KeyValuePairControl {
	/// Create a new [`KeyValuePairControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueKeyValue,
		default: ConValueKeyValueType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlDescription for KeyValuePairControl {
	type ConType = ConTypeKeyValue;
	type ValueType = ConValueKeyValue;

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

impl ControlDefault for KeyValuePairControl {
	type DefaultType = Self::ConType;

	fn control_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl fmt::Display for KeyValuePairControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(
				f,
				"{} {{{}}} *{}",
				self.control_type, self.value, self.default
			)
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '{}'",
				self.control_type, self.value, self.default
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
		let value = ConValueKeyValue::from((0, 0));
		let default = ConTypeKeyValue::from((0, 0));
		let control = KeyValuePairControl::new(control_type, value, default);
		super::super::tests::display_with_different_formats(
			"KeyValuePairControl",
			control,
		);
	}
}
