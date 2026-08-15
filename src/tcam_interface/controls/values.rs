//! Types that map to [`nokhwa`]'s [`ControlValueSetter`].

#[allow(unused_imports, reason = "Needed for module docstring")]
use nokhwa::utils::ControlValueSetter;

use super::{KeyValue, traits::ControlValue};

//pub struct ControlValueNone;

/// A control that has a value of the type [`i64`].
///
/// Maps to [`ControlValueSetter::Integer`].
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlValueInteger(i64);

impl From<i64> for ControlValueInteger {
	fn from(value: i64) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueInteger {
	type ValueType = i64;

	fn get_value(&self) -> Self::ValueType {
		self.0
	}
}

/// A control that has a value of the type [`f64`].
///
/// Maps to [`ControlValueSetter::Float`].
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ControlValueFloat(f64);

impl From<f64> for ControlValueFloat {
	fn from(value: f64) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueFloat {
	type ValueType = f64;

	fn get_value(&self) -> Self::ValueType {
		self.0
	}
}

/// A control that has a value of the type [`bool`].
///
/// Maps to [`ControlValueSetter::Boolean`].
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlValueBoolean(bool);

impl From<bool> for ControlValueBoolean {
	fn from(value: bool) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueBoolean {
	type ValueType = bool;

	fn get_value(&self) -> Self::ValueType {
		self.0
	}
}

/// A control that has a value of the type [`String`].
///
/// Maps to [`ControlValueSetter::String`].
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlValueString(String);

impl From<String> for ControlValueString {
	fn from(value: String) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueString {
	type ValueType = String;

	fn get_value(&self) -> Self::ValueType {
		self.0.clone()
	}
}

/// A control that has a value of the type [`Vec<u8>`].
///
/// Maps to [`ControlValueSetter::Bytes`].
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlValueBytes(Vec<u8>);

impl From<Vec<u8>> for ControlValueBytes {
	fn from(value: Vec<u8>) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueBytes {
	type ValueType = Vec<u8>;

	fn get_value(&self) -> Self::ValueType {
		self.0.clone()
	}
}

/// A control that has a value of the type [`KeyValue`].
///
/// Maps to [`ControlValueSetter::KeyValue`].
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlValueKeyValue(KeyValue);

impl From<KeyValue> for ControlValueKeyValue {
	fn from(value: KeyValue) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueKeyValue {
	type ValueType = KeyValue;

	fn get_value(&self) -> Self::ValueType {
		self.0
	}
}

/// A control that has a value of the type ([`f64`], [`f64`]).
///
/// Maps to [`ControlValueSetter::Point`].
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ControlValuePoint(f64, f64);

impl From<(f64, f64)> for ControlValuePoint {
	fn from(value: (f64, f64)) -> Self {
		Self(value.0, value.1)
	}
}

impl ControlValue for ControlValuePoint {
	type ValueType = (f64, f64);

	fn get_value(&self) -> Self::ValueType {
		(self.0, self.1)
	}
}

/// A control that has a value of the type [`i64`].
///
/// Maps to [`ControlValueSetter::EnumValue`].
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlValueEnum(i64);

impl From<i64> for ControlValueEnum {
	fn from(value: i64) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueEnum {
	type ValueType = i64;

	fn get_value(&self) -> Self::ValueType {
		self.0
	}
}

/// A control that has a value of the type ([`f64`], [`f64`], [`f64`]).
///
/// Maps to [`ControlValueSetter::RGB`].
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ControlValueRGB(f64, f64, f64);

impl From<(f64, f64, f64)> for ControlValueRGB {
	fn from(value: (f64, f64, f64)) -> Self {
		Self(value.0, value.1, value.2)
	}
}

impl ControlValue for ControlValueRGB {
	type ValueType = (f64, f64, f64);

	fn get_value(&self) -> Self::ValueType {
		(self.0, self.1, self.2)
	}
}
