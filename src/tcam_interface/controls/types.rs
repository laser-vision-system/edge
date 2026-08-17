//! Low level and higher level types used for camera control values.

#[allow(unused_imports, reason = "Needed for docstring")]
use super::{
	ConValueKeyValue, ConValuePoint, ConValueRGB, KeyValuePairControl, PointControl, RGBControl,
};

// TODO: Use COW for String and Bytes.

pub(super) type ConTypeInt = i64;
pub(super) type ConTypeFloat = f64;
pub(super) type ConTypeBool = bool;
pub(super) type ConTypeString = String;
pub(super) type ConTypeBytes = Vec<u8>;
pub(super) type ConTypeEnum = i64;

type __ConTypeKeyValue = i128;
type __ConTypePoint = f64;
type __ConTypeRGB = f64;

/// Represents a key and value use by [`ControlValueKeyValue`] and by extention, [`KeyValuePairControl`].
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConTypeKeyValue(__ConTypeKeyValue, __ConTypeKeyValue);

impl ConTypeKeyValue {
	pub fn get_key(&self) -> __ConTypeKeyValue {
		self.0
	}

	pub fn get_value(&self) -> __ConTypeKeyValue {
		self.1
	}
}

impl From<(__ConTypeKeyValue, __ConTypeKeyValue)> for ConTypeKeyValue {
	fn from(value: (__ConTypeKeyValue, __ConTypeKeyValue)) -> Self {
		Self(value.0, value.1)
	}
}

impl From<ConTypeKeyValue> for (__ConTypeKeyValue, __ConTypeKeyValue) {
	fn from(value: ConTypeKeyValue) -> Self {
		(value.0, value.1)
	}
}

/// Represents a point value use by [`ControlValuePoint`] and by extention, [`PointControl`].
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ConTypePoint(__ConTypePoint, __ConTypePoint);

impl ConTypePoint {
	pub fn get_x(&self) -> __ConTypePoint {
		self.0
	}

	pub fn get_y(&self) -> __ConTypePoint {
		self.1
	}
}

impl From<(__ConTypePoint, __ConTypePoint)> for ConTypePoint {
	fn from(value: (__ConTypePoint, __ConTypePoint)) -> Self {
		Self(value.0, value.1)
	}
}

impl From<ConTypePoint> for (__ConTypePoint, __ConTypePoint) {
	fn from(value: ConTypePoint) -> Self {
		(value.0, value.1)
	}
}

/// Represents an RGB value use by [`ControlValueRGB`] and by extention, [`RGBControl`].
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ConTypeRGB(__ConTypeRGB, __ConTypeRGB, __ConTypeRGB);

impl ConTypeRGB {
	pub fn get_r(&self) -> __ConTypeRGB {
		self.0
	}

	pub fn get_g(&self) -> __ConTypeRGB {
		self.1
	}

	pub fn get_b(&self) -> __ConTypeRGB {
		self.2
	}
}

impl From<(__ConTypeRGB, __ConTypeRGB, __ConTypeRGB)> for ConTypeRGB {
	fn from(value: (__ConTypeRGB, __ConTypeRGB, __ConTypeRGB)) -> Self {
		Self(value.0, value.1, value.2)
	}
}

impl From<ConTypeRGB> for (__ConTypeRGB, __ConTypeRGB, __ConTypeRGB) {
	fn from(value: ConTypeRGB) -> Self {
		(value.0, value.1, value.2)
	}
}
