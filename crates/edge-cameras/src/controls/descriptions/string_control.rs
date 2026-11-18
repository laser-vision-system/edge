//! Defines for a camera control that consists of a `value` and an optional
//! `default`.

use std::fmt::{
	self,
	Write as _,
};

use edge_core::write_optional;
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
	types::ConTypeString,
	values::{
		ConValueString,
		ConValueStringType,
	},
};
use crate::error::EdgeCamerasResult;

/// A camera control that consists of a `value` and an optional `default`.
///
/// Maps to [`ControlValueDescription::String`].
///
/// [`ControlValueDescription::String`]: nokhwa::utils::ControlValueDescription::String
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StringControl {
	control_type: KnownCameraControl,
	value: ConValueString,
	default: Option<ConValueStringType>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	flags: Vec<KnownCameraControlFlag>,
}

impl StringControl {
	/// Create a new [`StringControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueString,
		default: Option<ConValueStringType>,
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

impl ControlDescription for StringControl {
	type ConType = ConTypeString;
	type ValueType = ConValueString;

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

impl ControlDefault for StringControl {
	type DefaultType = Option<Self::ConType>;

	fn control_default(&self) -> Self::DefaultType {
		self.default.clone()
	}
}

impl ControlFlags for StringControl {
	fn control_flags(&self) -> &[KnownCameraControlFlag] {
		&self.flags
	}
}

impl fmt::Display for StringControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(f, "{} {} *", self.control_type, self.value)?;
			write_optional(f, &self.default)?;
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '",
				self.control_type, self.value,
			)?;
			write_optional(f, &self.default)?;
			f.write_char('\'')?;
		}
		super::write_flags(f, &self.flags)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::controls::traits::ControlValue as _;

	#[test]
	fn reset_default_preserves_value_when_default_is_missing() {
		let current = ConValueString::from("current".to_owned());
		let mut control = StringControl::new(
			KnownCameraControl::Other(1),
			current.clone(),
			None,
			Vec::new(),
		);

		assert!(matches!(
			control.reset_to_default(),
			Err(crate::error::EdgeCamerasError::ControlDefaultUnavailable(
				KnownCameraControl::Other(1)
			))
		));
		assert_eq!(control.control_value(), current);
	}

	#[test]
	fn reset_default_applies_present_default() {
		let default = ConTypeString::from("default".to_owned());
		let mut control = StringControl::new(
			KnownCameraControl::Other(1),
			ConValueString::from("current".to_owned()),
			Some(default.clone()),
			Vec::new(),
		);

		control.reset_to_default().unwrap();
		assert_eq!(control.control_value().get(), default);
	}

	#[test]
	#[ignore = "for manual inspection"]
	fn control_display() {
		let control_type = KnownCameraControl::Pan;
		let value = ConValueString::from("Value".to_owned());
		let default = Some(ConTypeString::from("default".to_owned()));
		let control =
			StringControl::new(control_type, value, default, Vec::new());
		super::super::tests::display_with_different_formats(
			"StringControl",
			&control,
		);
	}
}
