//! Low level types used for camera control values.

use std::{
	fmt,
	ops::{
		Deref,
		DerefMut,
	},
};

use nokhwa::utils::ControlValueSetter;
use serde::{
	Deserialize,
	Serialize,
};

// TODO: Use COW for String and Bytes.

pub(super) type ConTypeInt = i64;
pub(super) type ConTypeFloat = f64;
pub(super) type ConTypeBool = bool;
pub(super) type ConTypeEnum = i64;
pub(super) type ConTypeMenu = u32;

type _ConTypeKeyValue = i128;
type _ConTypePoint = f64;
type _ConTypeRGB = f64;

pub(super) type ConInnerTypeString = String;
pub(super) type ConInnerTypeBytes = Vec<u8>;
pub(super) type ConInnerTypeKeyValue = (_ConTypeKeyValue, _ConTypeKeyValue);
pub(super) type ConInnerTypePoint = (_ConTypePoint, _ConTypePoint);
pub(super) type ConInnerTypeRGB = (_ConTypeRGB, _ConTypeRGB, _ConTypeRGB);

/// Represents a key and value use by [`ConValueString`] and by extention,
/// [`StringControl`].
///
/// [`ConValueString`]: super::values::ConValueString
/// [`StringControl`]: super::descriptions::StringControl
#[derive(
	Clone,
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
pub struct ConTypeString(ConInnerTypeString);

impl Deref for ConTypeString {
	type Target = ConInnerTypeString;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for ConTypeString {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl From<ConInnerTypeString> for ConTypeString {
	fn from(value: ConInnerTypeString) -> Self {
		Self(value)
	}
}

impl From<ConTypeString> for ConInnerTypeString {
	fn from(value: ConTypeString) -> Self {
		value.0
	}
}

impl From<ConTypeString> for ControlValueSetter {
	fn from(value: ConTypeString) -> Self {
		ControlValueSetter::String(value.0)
	}
}

impl fmt::Display for ConTypeString {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// Represents a key and value use by [`ConValueBytes`] and by extention,
/// [`BytesControl`].
///
/// [`ConValueBytes`]: super::values::ConValueBytes
/// [`BytesControl`]: super::descriptions::BytesControl
#[derive(
	Clone,
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
pub struct ConTypeBytes(ConInnerTypeBytes);

impl Deref for ConTypeBytes {
	type Target = ConInnerTypeBytes;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for ConTypeBytes {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl From<ConInnerTypeBytes> for ConTypeBytes {
	fn from(value: ConInnerTypeBytes) -> Self {
		Self(value)
	}
}

impl From<ConTypeBytes> for ConInnerTypeBytes {
	fn from(value: ConTypeBytes) -> Self {
		value.0
	}
}

impl From<ConTypeBytes> for ControlValueSetter {
	fn from(value: ConTypeBytes) -> Self {
		ControlValueSetter::Bytes(value.0)
	}
}

impl fmt::Display for ConTypeBytes {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "[")?;
		let last_byte = self.0.len() - 1;
		for (index, byte) in self.0.iter().enumerate() {
			match index {
				0 => write!(f, "[{byte}")?,
				val if val == last_byte => write!(f, "{byte}]")?,
				_ => write!(f, "{byte}, ")?,
			}
		}
		Ok(())
	}
}

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
pub struct ConTypeKeyValue(ConInnerTypeKeyValue);

impl ConTypeKeyValue {
	/// Get the key field.
	pub fn key(&self) -> _ConTypeKeyValue {
		self.0.0
	}

	/// Get the value field.
	pub fn value(&self) -> _ConTypeKeyValue {
		self.0.1
	}
}

impl From<ConInnerTypeKeyValue> for ConTypeKeyValue {
	fn from(value: ConInnerTypeKeyValue) -> Self {
		Self(value)
	}
}

impl From<ConTypeKeyValue> for ConInnerTypeKeyValue {
	fn from(value: ConTypeKeyValue) -> Self {
		value.0
	}
}

impl From<ConTypeKeyValue> for ControlValueSetter {
	fn from(value: ConTypeKeyValue) -> Self {
		ControlValueSetter::KeyValue(value.key(), value.value())
	}
}

impl fmt::Display for ConTypeKeyValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "{}: {}", self.key(), self.value())
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
pub struct ConTypePoint(ConInnerTypePoint);

impl ConTypePoint {
	/// Get the first value of this point.
	pub fn x_value(&self) -> _ConTypePoint {
		self.0.0
	}

	/// Get the second value of this point.
	pub fn y_value(&self) -> _ConTypePoint {
		self.0.1
	}
}

impl From<ConInnerTypePoint> for ConTypePoint {
	fn from(value: ConInnerTypePoint) -> Self {
		Self(value)
	}
}

impl From<ConTypePoint> for ConInnerTypePoint {
	fn from(value: ConTypePoint) -> Self {
		value.0
	}
}

impl From<ConTypePoint> for ControlValueSetter {
	fn from(value: ConTypePoint) -> Self {
		ControlValueSetter::Point(value.x_value(), value.y_value())
	}
}

impl fmt::Display for ConTypePoint {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "({}, {})", self.x_value(), self.y_value())
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
pub struct ConTypeRGB(ConInnerTypeRGB);

impl ConTypeRGB {
	/// Get the red channel.
	pub fn red(&self) -> _ConTypeRGB {
		self.0.0
	}

	/// Get the green channel.
	pub fn green(&self) -> _ConTypeRGB {
		self.0.1
	}

	/// Get the blue channel.
	pub fn blue(&self) -> _ConTypeRGB {
		self.0.2
	}
}

impl From<ConInnerTypeRGB> for ConTypeRGB {
	fn from(value: ConInnerTypeRGB) -> Self {
		Self(value)
	}
}

impl From<ConTypeRGB> for ConInnerTypeRGB {
	fn from(value: ConTypeRGB) -> Self {
		value.0
	}
}

impl From<ConTypeRGB> for ControlValueSetter {
	fn from(value: ConTypeRGB) -> Self {
		ControlValueSetter::RGB(value.red(), value.green(), value.blue())
	}
}

impl fmt::Display for ConTypeRGB {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "({}, {}, {})", self.red(), self.green(), self.blue())
	}
}
