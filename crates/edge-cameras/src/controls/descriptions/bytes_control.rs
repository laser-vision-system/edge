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

impl fmt::Display for BytesControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(f, "{} {} *{}", self.control_type, self.value, self.default)
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
		let value = ConValueBytes::from(vec![0; 5]);
		let default = ConTypeBytes::from(vec![0; 5]);
		let control = BytesControl::new(control_type, value, default);
		super::super::tests::display_with_different_formats(
			"BytesControl",
			control,
		);
	}
}
