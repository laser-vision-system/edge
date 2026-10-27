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
	types::ConTypeBool,
	values::{
		ConValueBoolean,
		ConValueBooleanType,
	},
};
use crate::error::EdgeCamerasResult;

/// A camera control that consists of a `value` and `default`.
///
/// Maps to [`ControlValueDescription::Boolean`].
///
/// [`ControlValueDescription::Boolean`]: nokhwa::utils::ControlValueDescription::Boolean
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BooleanControl {
	control_type: KnownCameraControl,
	value: ConValueBoolean,
	default: ConValueBooleanType,
}

impl BooleanControl {
	/// Create a new [`BooleanControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueBoolean,
		default: ConValueBooleanType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlDescription for BooleanControl {
	type ConType = ConTypeBool;
	type ValueType = ConValueBoolean;

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

impl ControlDefault for BooleanControl {
	type DefaultType = Self::ConType;

	fn control_default(&self) -> Self::DefaultType {
		self.default
	}
}
