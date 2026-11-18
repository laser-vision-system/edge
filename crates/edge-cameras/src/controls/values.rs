//! Types that map to [`nokhwa`]'s [`ControlValueSetter`].
//!
//! ==================== READ HERE FIRST ====================.
//!
//! This may seem like a needless middle layer between the
//! control types and the control descriptions, but it
//! provides an easy mapping to and from
//! `ControlValueSetter`.
//! Additionally, the lower-level type defines make it easy
//! to synchronise the types between this and higher-level
//! components.
//!
//! ==================== ^^^^^^^^^^^^^^^ ====================.

// TODO: Implement `Borrow` on everything.

use std::fmt;

use nokhwa::utils::ControlValueSetter;
use serde::{
	Deserialize,
	Serialize,
};

use super::{
	traits::ControlValue,
	types::{
		ControlTypeBoolean,
		ControlTypeBytes,
		ControlTypeBytesInner,
		ControlTypeEnum,
		ControlTypeFloat,
		ControlTypeInt,
		ControlTypeKeyValue,
		ControlTypeKeyValueInner,
		ControlTypeMenu,
		ControlTypePoint,
		ControlTypePointInner,
		ControlTypeRgb,
		ControlTypeRgbInner,
		ControlTypeString,
		ControlTypeStringInner,
	},
};

/// A control for integer values.
///
/// Maps to [`ControlValueSetter::Integer`].
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
pub struct IntegerControlValue(ControlTypeInt);

impl From<ControlTypeInt> for IntegerControlValue {
	fn from(value: ControlTypeInt) -> Self {
		Self(value)
	}
}

impl ControlValue for IntegerControlValue {
	type ConType = ControlTypeInt;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<IntegerControlValue> for ControlValueSetter {
	fn from(value: IntegerControlValue) -> Self {
		ControlValueSetter::Integer(value.0)
	}
}

impl fmt::Display for IntegerControlValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`IntegerControlValue`]'s internal type via [`ControlValue`].
pub type IntegerControlValueType =
	<IntegerControlValue as ControlValue>::ConType;

/// A control for floating-point values.
///
/// Maps to [`ControlValueSetter::Float`].
#[derive(
	Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct FloatControlValue(ControlTypeFloat);

impl From<ControlTypeFloat> for FloatControlValue {
	fn from(value: ControlTypeFloat) -> Self {
		Self(value)
	}
}

impl ControlValue for FloatControlValue {
	type ConType = ControlTypeFloat;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<FloatControlValue> for ControlValueSetter {
	fn from(value: FloatControlValue) -> Self {
		ControlValueSetter::Float(value.0)
	}
}

impl fmt::Display for FloatControlValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`FloatControlValue`]'s internal type via [`ControlValue`].
pub type FloatControlValueType = <FloatControlValue as ControlValue>::ConType;

/// A control for boolean values.
///
/// Maps to [`ControlValueSetter::Boolean`].
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
pub struct BooleanControlValue(ControlTypeBoolean);

impl From<ControlTypeBoolean> for BooleanControlValue {
	fn from(value: ControlTypeBoolean) -> Self {
		Self(value)
	}
}

impl ControlValue for BooleanControlValue {
	type ConType = ControlTypeBoolean;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<BooleanControlValue> for ControlValueSetter {
	fn from(value: BooleanControlValue) -> Self {
		ControlValueSetter::Boolean(value.0)
	}
}

impl fmt::Display for BooleanControlValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`BooleanControlValue`]'s internal type via [`ControlValue`].
pub type BooleanControlValueType =
	<BooleanControlValue as ControlValue>::ConType;

/// A control for string values.
///
/// Maps to [`ControlValueSetter::String`].
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
pub struct StringControlValue(ControlTypeString);

impl From<ControlTypeString> for StringControlValue {
	fn from(value: ControlTypeString) -> Self {
		Self(value)
	}
}

impl From<ControlTypeStringInner> for StringControlValue {
	fn from(value: ControlTypeStringInner) -> Self {
		ControlTypeString::from(value).into()
	}
}

impl ControlValue for StringControlValue {
	type ConType = ControlTypeString;

	fn get(&self) -> Self::ConType {
		self.0.clone()
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<StringControlValue> for ControlValueSetter {
	fn from(value: StringControlValue) -> Self {
		value.0.into()
	}
}

impl fmt::Display for StringControlValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`StringControlValue`]'s internal type via [`ControlValue`].
pub type StringControlValueType = <StringControlValue as ControlValue>::ConType;

/// A control for byte vector values.
///
/// Maps to [`ControlValueSetter::Bytes`].
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
pub struct BytesControlValue(ControlTypeBytes);

impl From<ControlTypeBytes> for BytesControlValue {
	fn from(value: ControlTypeBytes) -> Self {
		Self(value)
	}
}

impl From<ControlTypeBytesInner> for BytesControlValue {
	fn from(value: ControlTypeBytesInner) -> Self {
		ControlTypeBytes::from(value).into()
	}
}

impl ControlValue for BytesControlValue {
	type ConType = ControlTypeBytes;

	fn get(&self) -> Self::ConType {
		self.0.clone()
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<BytesControlValue> for ControlValueSetter {
	fn from(value: BytesControlValue) -> Self {
		value.0.into()
	}
}

impl fmt::Display for BytesControlValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`BytesControlValue`]'s internal type via [`ControlValue`].
pub type BytesControlValueType = <BytesControlValue as ControlValue>::ConType;

/// A control for key-value values.
///
/// Maps to [`ControlValueSetter::KeyValue`].
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
pub struct KeyValueControlValue(ControlTypeKeyValue);

impl From<ControlTypeKeyValue> for KeyValueControlValue {
	fn from(value: ControlTypeKeyValue) -> Self {
		Self(value)
	}
}

impl From<ControlTypeKeyValueInner> for KeyValueControlValue {
	fn from(value: ControlTypeKeyValueInner) -> Self {
		ControlTypeKeyValue::from(value).into()
	}
}
impl ControlValue for KeyValueControlValue {
	type ConType = ControlTypeKeyValue;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<KeyValueControlValue> for ControlValueSetter {
	fn from(value: KeyValueControlValue) -> Self {
		value.0.into()
	}
}

impl fmt::Display for KeyValueControlValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`KeyValueControlValue`]'s internal type via [`ControlValue`].
pub type KeyValueControlValueType =
	<KeyValueControlValue as ControlValue>::ConType;

/// A control for point values.
///
/// Maps to [`ControlValueSetter::Point`].
#[derive(
	Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct PointControlValue(ControlTypePoint);

impl From<ControlTypePoint> for PointControlValue {
	fn from(value: ControlTypePoint) -> Self {
		Self(value)
	}
}

impl From<ControlTypePointInner> for PointControlValue {
	fn from(value: ControlTypePointInner) -> Self {
		ControlTypePoint::from(value).into()
	}
}

impl ControlValue for PointControlValue {
	type ConType = ControlTypePoint;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<PointControlValue> for ControlValueSetter {
	fn from(value: PointControlValue) -> Self {
		value.0.into()
	}
}

impl fmt::Display for PointControlValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`PointControlValue`]'s internal type via [`ControlValue`].
pub type PointControlValueType = <PointControlValue as ControlValue>::ConType;

/// A control for enum values.
///
/// Maps to [`ControlValueSetter::EnumValue`].
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
pub struct EnumControlValue(ControlTypeEnum);

impl From<ControlTypeEnum> for EnumControlValue {
	fn from(value: ControlTypeEnum) -> Self {
		Self(value)
	}
}

impl ControlValue for EnumControlValue {
	type ConType = ControlTypeEnum;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<EnumControlValue> for ControlValueSetter {
	fn from(value: EnumControlValue) -> Self {
		ControlValueSetter::EnumValue(value.0)
	}
}

impl fmt::Display for EnumControlValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`EnumControlValue`]'s internal type via [`ControlValue`].
pub type EnumControlValueType = <EnumControlValue as ControlValue>::ConType;

/// A control for RGB values.
///
/// Maps to [`ControlValueSetter::RGB`].
#[derive(
	Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct RgbControlValue(ControlTypeRgb);

impl From<ControlTypeRgb> for RgbControlValue {
	fn from(value: ControlTypeRgb) -> Self {
		Self(value)
	}
}

impl From<ControlTypeRgbInner> for RgbControlValue {
	fn from(value: ControlTypeRgbInner) -> Self {
		ControlTypeRgb::from(value).into()
	}
}

impl ControlValue for RgbControlValue {
	type ConType = ControlTypeRgb;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<RgbControlValue> for ControlValueSetter {
	fn from(value: RgbControlValue) -> Self {
		value.0.into()
	}
}

impl fmt::Display for RgbControlValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`RgbControlValue`]'s internal type via [`ControlValue`].
pub type RgbControlValueType = <RgbControlValue as ControlValue>::ConType;

/// A control for menu values.
///
/// Maps to [`ControlValueSetter::Menu`].
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
pub struct MenuControlValue(ControlTypeMenu);

impl From<ControlTypeMenu> for MenuControlValue {
	fn from(value: ControlTypeMenu) -> Self {
		Self(value)
	}
}

impl ControlValue for MenuControlValue {
	type ConType = ControlTypeMenu;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<MenuControlValue> for ControlValueSetter {
	fn from(value: MenuControlValue) -> Self {
		ControlValueSetter::Menu(value.0)
	}
}

impl fmt::Display for MenuControlValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`MenuControlValue`]'s internal type via [`ControlValue`].
pub type MenuControlValueType = <MenuControlValue as ControlValue>::ConType;
