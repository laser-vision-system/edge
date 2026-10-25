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
pub struct ConValueInteger(ConTypeInt);

impl From<ConTypeInt> for ConValueInteger {
	fn from(value: ConTypeInt) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueInteger {
	type ConType = ConTypeInt;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<ConValueInteger> for ControlValueSetter {
	fn from(value: ConValueInteger) -> Self {
		ControlValueSetter::Integer(value.0)
	}
}

impl fmt::Display for ConValueInteger {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`ConValueInteger`]'s internal type via [`ControlValue`].
pub type ConValueIntegerType = <ConValueInteger as ControlValue>::ConType;

/// A control for floating-point values.
///
/// Maps to [`ControlValueSetter::Float`].
#[derive(
	Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct ConValueFloat(ConTypeFloat);

impl From<ConTypeFloat> for ConValueFloat {
	fn from(value: ConTypeFloat) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueFloat {
	type ConType = ConTypeFloat;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<ConValueFloat> for ControlValueSetter {
	fn from(value: ConValueFloat) -> Self {
		ControlValueSetter::Float(value.0)
	}
}

impl fmt::Display for ConValueFloat {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`ConValueFloat`]'s internal type via [`ControlValue`].
pub type ConValueFloatType = <ConValueFloat as ControlValue>::ConType;

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
pub struct ConValueBoolean(ConTypeBool);

impl From<ConTypeBool> for ConValueBoolean {
	fn from(value: ConTypeBool) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueBoolean {
	type ConType = ConTypeBool;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<ConValueBoolean> for ControlValueSetter {
	fn from(value: ConValueBoolean) -> Self {
		ControlValueSetter::Boolean(value.0)
	}
}

impl fmt::Display for ConValueBoolean {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`ConValueBoolean`]'s internal type via [`ControlValue`].
pub type ConValueBooleanType = <ConValueBoolean as ControlValue>::ConType;

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
pub struct ConValueString(ConTypeString);

impl From<ConTypeString> for ConValueString {
	fn from(value: ConTypeString) -> Self {
		Self(value)
	}
}

impl From<ConInnerTypeString> for ConValueString {
	fn from(value: ConInnerTypeString) -> Self {
		ConTypeString::from(value).into()
	}
}

impl ControlValue for ConValueString {
	type ConType = ConTypeString;

	fn get(&self) -> Self::ConType {
		self.0.clone()
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<ConValueString> for ControlValueSetter {
	fn from(value: ConValueString) -> Self {
		value.0.into()
	}
}

impl fmt::Display for ConValueString {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`ConValueString`]'s internal type via [`ControlValue`].
pub type ConValueStringType = <ConValueString as ControlValue>::ConType;

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
pub struct ConValueBytes(ConTypeBytes);

impl From<ConTypeBytes> for ConValueBytes {
	fn from(value: ConTypeBytes) -> Self {
		Self(value)
	}
}

impl From<ConInnerTypeBytes> for ConValueBytes {
	fn from(value: ConInnerTypeBytes) -> Self {
		ConTypeBytes::from(value).into()
	}
}

impl ControlValue for ConValueBytes {
	type ConType = ConTypeBytes;

	fn get(&self) -> Self::ConType {
		self.0.clone()
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<ConValueBytes> for ControlValueSetter {
	fn from(value: ConValueBytes) -> Self {
		value.0.into()
	}
}

impl fmt::Display for ConValueBytes {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`ConValueBytes`]'s internal type via [`ControlValue`].
pub type ConValueBytesType = <ConValueBytes as ControlValue>::ConType;

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
pub struct ConValueKeyValue(ConTypeKeyValue);

impl From<ConTypeKeyValue> for ConValueKeyValue {
	fn from(value: ConTypeKeyValue) -> Self {
		Self(value)
	}
}

impl From<ConInnerTypeKeyValue> for ConValueKeyValue {
	fn from(value: ConInnerTypeKeyValue) -> Self {
		ConTypeKeyValue::from(value).into()
	}
}
impl ControlValue for ConValueKeyValue {
	type ConType = ConTypeKeyValue;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<ConValueKeyValue> for ControlValueSetter {
	fn from(value: ConValueKeyValue) -> Self {
		value.0.into()
	}
}

impl fmt::Display for ConValueKeyValue {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`ConValueKeyValue`]'s internal type via [`ControlValue`].
pub type ConValueKeyValueType = <ConValueKeyValue as ControlValue>::ConType;

/// A control for point values.
///
/// Maps to [`ControlValueSetter::Point`].
#[derive(
	Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct ConValuePoint(ConTypePoint);

impl From<ConTypePoint> for ConValuePoint {
	fn from(value: ConTypePoint) -> Self {
		Self(value)
	}
}

impl From<ConInnerTypePoint> for ConValuePoint {
	fn from(value: ConInnerTypePoint) -> Self {
		ConTypePoint::from(value).into()
	}
}

impl ControlValue for ConValuePoint {
	type ConType = ConTypePoint;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<ConValuePoint> for ControlValueSetter {
	fn from(value: ConValuePoint) -> Self {
		value.0.into()
	}
}

impl fmt::Display for ConValuePoint {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`ConValuePoint`]'s internal type via [`ControlValue`].
pub type ConValuePointType = <ConValuePoint as ControlValue>::ConType;

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
pub struct ConValueEnum(ConTypeEnum);

impl From<ConTypeEnum> for ConValueEnum {
	fn from(value: ConTypeEnum) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueEnum {
	type ConType = ConTypeEnum;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<ConValueEnum> for ControlValueSetter {
	fn from(value: ConValueEnum) -> Self {
		ControlValueSetter::EnumValue(value.0)
	}
}

impl fmt::Display for ConValueEnum {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`ConValueEnum`]'s internal type via [`ControlValue`].
pub type ConValueEnumType = <ConValueEnum as ControlValue>::ConType;

/// A control for RGB values.
///
/// Maps to [`ControlValueSetter::RGB`].
#[derive(
	Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct ConValueRGB(ConTypeRGB);

impl From<ConTypeRGB> for ConValueRGB {
	fn from(value: ConTypeRGB) -> Self {
		Self(value)
	}
}

impl From<ConInnerTypeRGB> for ConValueRGB {
	fn from(value: ConInnerTypeRGB) -> Self {
		ConTypeRGB::from(value).into()
	}
}

impl ControlValue for ConValueRGB {
	type ConType = ConTypeRGB;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<ConValueRGB> for ControlValueSetter {
	fn from(value: ConValueRGB) -> Self {
		value.0.into()
	}
}

impl fmt::Display for ConValueRGB {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}
/// [`ConValueRGB`]'s internal type via [`ControlValue`].
pub type ConValueRGBType = <ConValueRGB as ControlValue>::ConType;

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
pub struct ConValueMenu(ConTypeMenu);

impl From<ConTypeMenu> for ConValueMenu {
	fn from(value: ConTypeMenu) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueMenu {
	type ConType = ConTypeMenu;

	fn get(&self) -> Self::ConType {
		self.0
	}

	fn get_mut(&mut self) -> &mut Self::ConType {
		&mut self.0
	}
}

impl From<ConValueMenu> for ControlValueSetter {
	fn from(value: ConValueMenu) -> Self {
		ControlValueSetter::Menu(value.0)
	}
}

impl fmt::Display for ConValueMenu {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// [`ConValueMenu`]'s internal type via [`ControlValue`].
pub type ConValueMenuType = <ConValueMenu as ControlValue>::ConType;
