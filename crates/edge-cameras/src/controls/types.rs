//! Low level types used for camera control values.

use nokhwa::utils::ControlValueSetter;
use serde::{
	Deserialize,
	Serialize,
};

// TODO: Use COW for String and Bytes.

pub(super) type ConTypeInt = i64;
pub(super) type ConTypeFloat = f64;
pub(super) type ConTypeBool = bool;
pub(super) type ConTypeString = String;
pub(super) type ConTypeBytes = Vec<u8>;
pub(super) type ConTypeEnum = i64;
pub(super) type ConTypeMenu = u32;

type _ConTypeKeyValue = i128;
type _ConTypePoint = f64;
type _ConTypeRGB = f64;

/// Represents a key and value use by [`ConValueKeyValue`] and by extention,
/// [`KeyValuePairControl`].
///
/// [`ConValueKeyValue`]: super::values::ConValueKeyValue
/// [`KeyValuePairControl`]: super::descriptions::KeyValuePairControl
#[derive(
	Clone,
	Copy,
	Debug,
	Default,
	Hash,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Serialize,
	Deserialize,
)]
pub struct ConTypeKeyValue(_ConTypeKeyValue, _ConTypeKeyValue);

impl ConTypeKeyValue {
	/// Get the key field.
	pub fn key(&self) -> _ConTypeKeyValue {
		self.0
	}

	/// Get the value field.
	pub fn value(&self) -> _ConTypeKeyValue {
		self.1
	}
}

impl From<(_ConTypeKeyValue, _ConTypeKeyValue)> for ConTypeKeyValue {
	fn from(value: (_ConTypeKeyValue, _ConTypeKeyValue)) -> Self {
		Self(value.0, value.1)
	}
}

impl From<ConTypeKeyValue> for (_ConTypeKeyValue, _ConTypeKeyValue) {
	fn from(value: ConTypeKeyValue) -> Self {
		(value.0, value.1)
	}
}

impl From<ConTypeKeyValue> for ControlValueSetter {
	fn from(value: ConTypeKeyValue) -> Self {
		ControlValueSetter::KeyValue(value.key(), value.value())
	}
}

/// Represents a point value use by [`ConValuePoint`] and by extention,
/// [`PointControl`].
///
/// [`ConValuePoint`]: super::values::ConValuePoint
/// [`PointControl`]: super::descriptions::PointControl
#[derive(
	Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct ConTypePoint(_ConTypePoint, _ConTypePoint);

impl ConTypePoint {
	/// Get the first value of this point.
	pub fn x_value(&self) -> _ConTypePoint {
		self.0
	}

	/// Get the second value of this point.
	pub fn y_value(&self) -> _ConTypePoint {
		self.1
	}
}

impl From<(_ConTypePoint, _ConTypePoint)> for ConTypePoint {
	fn from(value: (_ConTypePoint, _ConTypePoint)) -> Self {
		Self(value.0, value.1)
	}
}

impl From<ConTypePoint> for (_ConTypePoint, _ConTypePoint) {
	fn from(value: ConTypePoint) -> Self {
		(value.0, value.1)
	}
}

impl From<ConTypePoint> for ControlValueSetter {
	fn from(value: ConTypePoint) -> Self {
		ControlValueSetter::Point(value.x_value(), value.y_value())
	}
}

/// Represents an RGB value use by [`ConValueRGB`] and by extention,
/// [`RGBControl`].
///
/// [`ConValueRGB`]: super::values::ConValueRGB
/// [`RGBControl`]: super::descriptions::RGBControl
#[derive(
	Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct ConTypeRGB(_ConTypeRGB, _ConTypeRGB, _ConTypeRGB);

impl ConTypeRGB {
	/// Get the red channel.
	pub fn red(&self) -> _ConTypeRGB {
		self.0
	}

	/// Get the green channel.
	pub fn green(&self) -> _ConTypeRGB {
		self.1
	}

	/// Get the blue channel.
	pub fn blue(&self) -> _ConTypeRGB {
		self.2
	}
}

impl From<(_ConTypeRGB, _ConTypeRGB, _ConTypeRGB)> for ConTypeRGB {
	fn from(value: (_ConTypeRGB, _ConTypeRGB, _ConTypeRGB)) -> Self {
		Self(value.0, value.1, value.2)
	}
}

impl From<ConTypeRGB> for (_ConTypeRGB, _ConTypeRGB, _ConTypeRGB) {
	fn from(value: ConTypeRGB) -> Self {
		(value.0, value.1, value.2)
	}
}

impl From<ConTypeRGB> for ControlValueSetter {
	fn from(value: ConTypeRGB) -> Self {
		ControlValueSetter::RGB(value.red(), value.green(), value.blue())
	}
}
