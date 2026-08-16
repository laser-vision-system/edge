//! Low level and higher level types used for camera control values.

#[allow(unused_imports, reason = "Needed for docstring")]
use super::{
	ControlValueKeyValue, ControlValuePoint, ControlValueRGB, KeyValuePairControl, PointControl,
	RGBControl,
};

// TODO: Use COW for String and Bytes.

pub(super) type ControlTypeInteger = i64;
pub(super) type ControlTypeFloat = f64;
pub(super) type ControlTypeBool = bool;
pub(super) type ControlTypeString = String;
pub(super) type ControlTypeBytes = Vec<u8>;
pub(super) type ControlTypeEnum = i64;

type ControlTypeKeyValue = i128;
type ControlTypePoint = f64;
type ControlTypeRGB = f64;

/// Represents a key and value use by [`ControlValueKeyValue`] and by extention, [`KeyValuePairControl`].
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyValue(ControlTypeKeyValue, ControlTypeKeyValue);

impl KeyValue {
	pub fn get_key(&self) -> ControlTypeKeyValue {
		self.0
	}

	pub fn get_value(&self) -> ControlTypeKeyValue {
		self.1
	}
}

impl From<(ControlTypeKeyValue, ControlTypeKeyValue)> for KeyValue {
	fn from(value: (ControlTypeKeyValue, ControlTypeKeyValue)) -> Self {
		Self(value.0, value.1)
	}
}

impl From<KeyValue> for (ControlTypeKeyValue, ControlTypeKeyValue) {
	fn from(value: KeyValue) -> Self {
		(value.0, value.1)
	}
}

/// Represents a point value use by [`ControlValuePoint`] and by extention, [`PointControl`].
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Point(ControlTypePoint, ControlTypePoint);

impl Point {
	pub fn get_x(&self) -> ControlTypePoint {
		self.0
	}

	pub fn get_y(&self) -> ControlTypePoint {
		self.1
	}
}

impl From<(ControlTypePoint, ControlTypePoint)> for Point {
	fn from(value: (ControlTypePoint, ControlTypePoint)) -> Self {
		Self(value.0, value.1)
	}
}

impl From<Point> for (ControlTypePoint, ControlTypePoint) {
	fn from(value: Point) -> Self {
		(value.0, value.1)
	}
}

/// Represents an RGB value use by [`ControlValueRGB`] and by extention, [`RGBControl`].
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RGB(ControlTypeRGB, ControlTypeRGB, ControlTypeRGB);

impl RGB {
	pub fn get_r(&self) -> ControlTypeRGB {
		self.0
	}

	pub fn get_g(&self) -> ControlTypeRGB {
		self.1
	}

	pub fn get_b(&self) -> ControlTypeRGB {
		self.2
	}
}

impl From<(ControlTypeRGB, ControlTypeRGB, ControlTypeRGB)> for RGB {
	fn from(value: (ControlTypeRGB, ControlTypeRGB, ControlTypeRGB)) -> Self {
		Self(value.0, value.1, value.2)
	}
}

impl From<RGB> for (ControlTypeRGB, ControlTypeRGB, ControlTypeRGB) {
	fn from(value: RGB) -> Self {
		(value.0, value.1, value.2)
	}
}
