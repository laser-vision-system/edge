//! Defines for a camera control that consists of a `value` and an optional
//! `default`.

use std::fmt::{
	self,
	Write as _,
};

use edge_core::write_optional;
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
}

impl StringControl {
	/// Create a new [`StringControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueString,
		default: Option<ConValueStringType>,
	) -> Self {
		Self {
			control_type,
			value,
			default,
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

	#[expect(
		clippy::expect_used,
		reason = "Default value should never be invalid"
	)]
	fn reset_to_default(&mut self) {
		self.try_set_control_value(
			self.control_default().unwrap_or_default().into(),
		)
		.expect("default value should never be invalid");
	}
}

impl fmt::Display for StringControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(f, "{} {} *", self.control_type, self.value)?;
			write_optional(f, &self.default)
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '",
				self.control_type, self.value,
			)?;
			write_optional(f, &self.default)?;
			f.write_char('\'')
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
		let value = ConValueString::from("Value".to_owned());
		let default = Some(ConTypeString::from("default".to_owned()));
		let control = StringControl::new(control_type, value, default);
		super::super::tests::display_with_different_formats(
			"StringControl",
			control,
		);
	}
}
