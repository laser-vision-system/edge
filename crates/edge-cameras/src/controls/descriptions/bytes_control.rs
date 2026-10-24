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
	types::ConTypeBytes,
	values::{
		ConValueBytes,
		ConValueBytesType,
	},
};
use crate::error::EdgeCamerasResult;

/// A camera control that consists of a `value` and `default`.
///
/// Maps to [`ControlValueDescription::Bytes`].
///
/// [`ControlValueDescription::Bytes`]: nokhwa::utils::ControlValueDescription::Bytes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BytesControl {
	control_type: KnownCameraControl,
	value: ConValueBytes,
	default: ConValueBytesType,
}

impl BytesControl {
	/// Create a new [`BytesControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueBytes,
		default: ConValueBytesType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlDescription for BytesControl {
	type ConType = ConTypeBytes;
	type ValueType = ConValueBytes;

	fn control_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn try_set_control_value(
		&mut self,
		value: Self::ValueType,
	) -> EdgeCamerasResult<()> {
		self.value = value;
		Ok(())
	}

	fn control_value(&self) -> Self::ValueType {
		self.value.clone()
	}
}

impl ControlDefault for BytesControl {
	type DefaultType = Self::ConType;

	fn control_default(&self) -> Self::DefaultType {
		self.default.clone()
	}
}
