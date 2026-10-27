//! Defines for a camera control that consists of a `value` and `default`.

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
