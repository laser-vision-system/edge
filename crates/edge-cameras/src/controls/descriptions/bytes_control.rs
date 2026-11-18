//! Defines for a camera control that consists of a `value` and `default`.

use std::fmt::{
	self,
	Write,
};

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
	},
	types::ControlTypeBytes,
	values::{
		BytesControlValue,
		BytesControlValueType,
	},
};
use crate::{
	controls::write_flags,
	error::EdgeCamerasResult,
};

/// A camera control that consists of a `value` and `default`.
///
/// Maps to [`ControlValueDescription::Bytes`].
///
/// [`ControlValueDescription::Bytes`]: nokhwa::utils::ControlValueDescription::Bytes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BytesControl {
	control_type: KnownCameraControl,
	value: BytesControlValue,
	default: BytesControlValueType,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	flags: Vec<KnownCameraControlFlag>,
}

impl BytesControl {
	/// Create a new [`BytesControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: BytesControlValue,
		default: BytesControlValueType,
		flags: Vec<KnownCameraControlFlag>,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			flags,
		}
	}
}

impl ControlDescription for BytesControl {
	type ConType = ControlTypeBytes;
	type ValueType = BytesControlValue;

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
		self.value.clone()
	}
}

impl ControlDefault for BytesControl {
	type DefaultType = Self::ConType;

	fn control_default(&self) -> Self::DefaultType {
		self.default.clone()
	}
}

impl ControlFlags for BytesControl {
	fn control_flags(&self) -> &[KnownCameraControlFlag] {
		&self.flags
	}
}

impl fmt::Display for BytesControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(
				f,
				"{} {} *{} ",
				self.control_type, self.value, self.default
			)?;
			write_flags(f, &self.flags)?;
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '{}' flags: '",
				self.control_type, self.value, self.default
			)?;
			write_flags(f, &self.flags)?;
			f.write_char('\'')?;
		}
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
		let value = BytesControlValue::from(vec![0; 5]);
		let default = ControlTypeBytes::from(vec![0; 5]);
		let flags = vec![KnownCameraControlFlag::Disabled];
		let control = BytesControl::new(control_type, value, default, flags);
		super::super::tests::display_with_different_formats(
			"BytesControl",
			&control,
		);
	}
}
