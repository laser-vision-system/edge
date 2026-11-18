//! Low level types used for camera control values.

use std::{
	fmt,
	ops::{
		Deref,
		DerefMut,
	},
};

use nokhwa::utils::{
	ControlValueSetter,
	MenuEntry,
};
use serde::{
	Deserialize,
	Serialize,
};

use super::display_list;

// TODO: Use COW for String and Bytes.

pub(super) type ControlTypeInt = i64;
pub(super) type ControlTypeFloat = f64;
pub(super) type ControlTypeBoolean = bool;
pub(super) type ControlTypeEnum = i64;
pub(super) type ControlTypeMenu = u32;

pub(super) type ControlTypeStringInner = String;
pub(super) type ControlTypeBytesInner = Vec<u8>;

type KeyValuePrimitive = i128;
type PointPrimitive = f64;
type RgbPrimitive = f64;

pub(super) type ControlTypeKeyValueInner =
	(KeyValuePrimitive, KeyValuePrimitive);
pub(super) type ControlTypePointInner = (PointPrimitive, PointPrimitive);
pub(super) type ControlTypeRgbInner =
	(RgbPrimitive, RgbPrimitive, RgbPrimitive);

pub(super) type ControlTypePossibleInner = Vec<i64>;
pub(super) type ControlTypeItemsInner = Vec<MenuEntry>;

/// Represents a list of Strings used by [`ConValueString`] and by extention,
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
pub struct ControlTypeString(ControlTypeStringInner);

impl Deref for ControlTypeString {
	type Target = ControlTypeStringInner;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for ControlTypeString {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl From<ControlTypeStringInner> for ControlTypeString {
	fn from(value: ControlTypeStringInner) -> Self {
		Self(value)
	}
}

impl From<ControlTypeString> for ControlTypeStringInner {
	fn from(value: ControlTypeString) -> Self {
		value.0
	}
}

impl From<ControlTypeString> for ControlValueSetter {
	fn from(value: ControlTypeString) -> Self {
		ControlValueSetter::String(value.0)
	}
}

impl fmt::Display for ControlTypeString {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// Represents a list of bytes used by [`ConValueBytes`] and by extention,
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
pub struct ControlTypeBytes(ControlTypeBytesInner);

impl Deref for ControlTypeBytes {
	type Target = ControlTypeBytesInner;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for ControlTypeBytes {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl From<ControlTypeBytesInner> for ControlTypeBytes {
	fn from(value: ControlTypeBytesInner) -> Self {
		Self(value)
	}
}

impl From<ControlTypeBytes> for ControlTypeBytesInner {
	fn from(value: ControlTypeBytes) -> Self {
		value.0
	}
}

impl From<ControlTypeBytes> for ControlValueSetter {
	fn from(value: ControlTypeBytes) -> Self {
		ControlValueSetter::Bytes(value.0)
	}
}

impl fmt::Display for ControlTypeBytes {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		display_list(f, &self.0, false, None)
	}
}

/// Represents a key and value used by [`ConValueKeyValue`] and by extention,
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
pub struct ControlTypeKeyValue(ControlTypeKeyValueInner);

impl ControlTypeKeyValue {
	/// Get the key field.
	pub fn key(&self) -> KeyValuePrimitive {
		self.0.0
	}

	/// Get the value field.
	pub fn value(&self) -> KeyValuePrimitive {
		self.0.1
	}
}

impl From<ControlTypeKeyValueInner> for ControlTypeKeyValue {
	fn from(value: ControlTypeKeyValueInner) -> Self {
		Self(value)
	}
}

impl From<ControlTypeKeyValue> for ControlTypeKeyValueInner {
	fn from(value: ControlTypeKeyValue) -> Self {
		value.0
	}
}

impl From<ControlTypeKeyValue> for ControlValueSetter {
	fn from(value: ControlTypeKeyValue) -> Self {
		ControlValueSetter::KeyValue(value.key(), value.value())
	}
}

impl fmt::Display for ControlTypeKeyValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "{}: {}", self.key(), self.value())
	}
}

/// Represents a point value used by [`ConValuePoint`] and by extention,
/// [`PointControl`].
///
/// [`ConValuePoint`]: super::values::ConValuePoint
/// [`PointControl`]: super::descriptions::PointControl
#[derive(
	Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct ControlTypePoint(ControlTypePointInner);

impl ControlTypePoint {
	/// Get the first value of this point.
	pub fn x_value(&self) -> PointPrimitive {
		self.0.0
	}

	/// Get the second value of this point.
	pub fn y_value(&self) -> PointPrimitive {
		self.0.1
	}
}

impl From<ControlTypePointInner> for ControlTypePoint {
	fn from(value: ControlTypePointInner) -> Self {
		Self(value)
	}
}

impl From<ControlTypePoint> for ControlTypePointInner {
	fn from(value: ControlTypePoint) -> Self {
		value.0
	}
}

impl From<ControlTypePoint> for ControlValueSetter {
	fn from(value: ControlTypePoint) -> Self {
		ControlValueSetter::Point(value.x_value(), value.y_value())
	}
}

impl fmt::Display for ControlTypePoint {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "({}, {})", self.x_value(), self.y_value())
	}
}

/// Represents an RGB value used by [`ConValueRGB`] and by extention,
/// [`RGBControl`].
///
/// [`ConValueRGB`]: super::values::ConValueRGB
/// [`RGBControl`]: super::descriptions::RGBControl
#[derive(
	Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct ControlTypeRgb(ControlTypeRgbInner);

impl ControlTypeRgb {
	/// Get the red channel.
	pub fn red(&self) -> RgbPrimitive {
		self.0.0
	}

	/// Get the green channel.
	pub fn green(&self) -> RgbPrimitive {
		self.0.1
	}

	/// Get the blue channel.
	pub fn blue(&self) -> RgbPrimitive {
		self.0.2
	}
}

impl From<ControlTypeRgbInner> for ControlTypeRgb {
	fn from(value: ControlTypeRgbInner) -> Self {
		Self(value)
	}
}

impl From<ControlTypeRgb> for ControlTypeRgbInner {
	fn from(value: ControlTypeRgb) -> Self {
		value.0
	}
}

impl From<ControlTypeRgb> for ControlValueSetter {
	fn from(value: ControlTypeRgb) -> Self {
		ControlValueSetter::RGB(value.red(), value.green(), value.blue())
	}
}

impl fmt::Display for ControlTypeRgb {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "({}, {}, {})", self.red(), self.green(), self.blue())
	}
}

/// Represents a list of possible variants used by [`EnumControl`].
///
/// [`EnumControl`]: super::descriptions::EnumControl
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
pub struct ControlTypePossible(ControlTypePossibleInner);

impl Deref for ControlTypePossible {
	type Target = ControlTypePossibleInner;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for ControlTypePossible {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl From<ControlTypePossibleInner> for ControlTypePossible {
	fn from(value: ControlTypePossibleInner) -> Self {
		Self(value)
	}
}

impl From<ControlTypePossible> for ControlTypePossibleInner {
	fn from(value: ControlTypePossible) -> Self {
		value.0
	}
}

impl fmt::Display for ControlTypePossible {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		display_list(f, &self.0, false, None)
	}
}

/// Represents a list of [`MenuEntry`]s used by [`MenuControl`].
///
/// [`MenuControl`]: super::descriptions::MenuControl
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
pub struct ControlTypeItems(ControlTypeItemsInner);

impl Deref for ControlTypeItems {
	type Target = ControlTypeItemsInner;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for ControlTypeItems {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl From<ControlTypeItemsInner> for ControlTypeItems {
	fn from(value: ControlTypeItemsInner) -> Self {
		Self(value)
	}
}

impl From<ControlTypeItems> for ControlTypeItemsInner {
	fn from(value: ControlTypeItems) -> Self {
		value.0
	}
}

impl fmt::Display for ControlTypeItems {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		display_list(f, &self.0, true, None)
	}
}
