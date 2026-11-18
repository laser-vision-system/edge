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
		ConInnerTypeBytes,
		ConInnerTypeKeyValue,
		ConInnerTypePoint,
		ConInnerTypeRGB,
		ConInnerTypeString,
		ConTypeBool,
		ConTypeBytes,
		ConTypeEnum,
		ConTypeFloat,
		ConTypeInt,
		ConTypeKeyValue,
		ConTypeMenu,
		ConTypePoint,
		ConTypeRGB,
		ConTypeString,
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
pub struct IntegerControlValue(ConTypeInt);

impl From<ConTypeInt> for IntegerControlValue {
	fn from(value: ConTypeInt) -> Self {
		Self(value)
	}
}

impl ControlValue for IntegerControlValue {
	type ConType = ConTypeInt;

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
pub struct FloatControlValue(ConTypeFloat);

impl From<ConTypeFloat> for FloatControlValue {
	fn from(value: ConTypeFloat) -> Self {
		Self(value)
	}
}

impl ControlValue for FloatControlValue {
	type ConType = ConTypeFloat;

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
pub struct BooleanControlValue(ConTypeBool);

impl From<ConTypeBool> for BooleanControlValue {
	fn from(value: ConTypeBool) -> Self {
		Self(value)
	}
}

impl ControlValue for BooleanControlValue {
	type ConType = ConTypeBool;

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
pub struct StringControlValue(ConTypeString);

impl From<ConTypeString> for StringControlValue {
	fn from(value: ConTypeString) -> Self {
		Self(value)
	}
}

impl From<ConInnerTypeString> for StringControlValue {
	fn from(value: ConInnerTypeString) -> Self {
		ConTypeString::from(value).into()
	}
}

impl ControlValue for StringControlValue {
	type ConType = ConTypeString;

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
pub struct BytesControlValue(ConTypeBytes);

impl From<ConTypeBytes> for BytesControlValue {
	fn from(value: ConTypeBytes) -> Self {
		Self(value)
	}
}

impl From<ConInnerTypeBytes> for BytesControlValue {
	fn from(value: ConInnerTypeBytes) -> Self {
		ConTypeBytes::from(value).into()
	}
}

impl ControlValue for BytesControlValue {
	type ConType = ConTypeBytes;

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
pub struct KeyValueControlValue(ConTypeKeyValue);

impl From<ConTypeKeyValue> for KeyValueControlValue {
	fn from(value: ConTypeKeyValue) -> Self {
		Self(value)
	}
}

impl From<ConInnerTypeKeyValue> for KeyValueControlValue {
	fn from(value: ConInnerTypeKeyValue) -> Self {
		ConTypeKeyValue::from(value).into()
	}
}
impl ControlValue for KeyValueControlValue {
	type ConType = ConTypeKeyValue;

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
pub struct PointControlValue(ConTypePoint);

impl From<ConTypePoint> for PointControlValue {
	fn from(value: ConTypePoint) -> Self {
		Self(value)
	}
}

impl From<ConInnerTypePoint> for PointControlValue {
	fn from(value: ConInnerTypePoint) -> Self {
		ConTypePoint::from(value).into()
	}
}

impl ControlValue for PointControlValue {
	type ConType = ConTypePoint;

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
pub struct EnumControlValue(ConTypeEnum);

impl From<ConTypeEnum> for EnumControlValue {
	fn from(value: ConTypeEnum) -> Self {
		Self(value)
	}
}

impl ControlValue for EnumControlValue {
	type ConType = ConTypeEnum;

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
pub struct RgbControlValue(ConTypeRGB);

impl From<ConTypeRGB> for RgbControlValue {
	fn from(value: ConTypeRGB) -> Self {
		Self(value)
	}
}

impl From<ConInnerTypeRGB> for RgbControlValue {
	fn from(value: ConInnerTypeRGB) -> Self {
		ConTypeRGB::from(value).into()
	}
}

impl ControlValue for RgbControlValue {
	type ConType = ConTypeRGB;

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
pub struct MenuControlValue(ConTypeMenu);

impl From<ConTypeMenu> for MenuControlValue {
	fn from(value: ConTypeMenu) -> Self {
		Self(value)
	}
}

impl ControlValue for MenuControlValue {
	type ConType = ConTypeMenu;

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
