//! Types that map to [`nokhwa`]'s [`ControlValueSetter`].

// TODO: Implement `Borrow` on everything.

use nokhwa::utils::ControlValueSetter;

use super::{
	traits::ControlValue,
	types::{
		ConTypeBool, ConTypeBytes, ConTypeEnum, ConTypeFloat, ConTypeInt, ConTypeKeyValue,
		ConTypeMenu, ConTypePoint, ConTypeRGB, ConTypeString,
	},
};

// ==================== READ HERE FIRST ====================
//
// This may seem like a needless middle layer between the
// control types and the control descriptions, but it
// provides an easy mapping to and from
// `ControlValueSetter`.
// Additionally, the lower-level type defines make it easy
// to synchronise the types between this and higher-level
// components.
//
// ==================== ^^^^^^^^^^^^^^^ ====================

//pub struct ControlValueNone;

/// A control that has a value of the type [`ControlTypeInteger`].
///
/// Maps to [`ControlValueSetter::Integer`].
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConValueInteger(ConTypeInt);

impl From<ConTypeInt> for ConValueInteger {
	fn from(value: ConTypeInt) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueInteger {
	type ConType = ConTypeInt;

	fn get_value(&self) -> Self::ConType {
		self.0
	}

	fn set_value(&mut self, value: Self::ConType) {
		self.0 = value;
	}
}

impl From<ConValueInteger> for ControlValueSetter {
	fn from(value: ConValueInteger) -> Self {
		ControlValueSetter::Integer(value.0)
	}
}

pub type ConValueIntegerType = <ConValueInteger as ControlValue>::ConType;

/// A control that has a value of the type [`ControlTypeFloat`].
///
/// Maps to [`ControlValueSetter::Float`].
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ConValueFloat(ConTypeFloat);

impl From<ConTypeFloat> for ConValueFloat {
	fn from(value: ConTypeFloat) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueFloat {
	type ConType = ConTypeFloat;

	fn get_value(&self) -> Self::ConType {
		self.0
	}

	fn set_value(&mut self, value: Self::ConType) {
		self.0 = value;
	}
}

impl From<ConValueFloat> for ControlValueSetter {
	fn from(value: ConValueFloat) -> Self {
		ControlValueSetter::Float(value.0)
	}
}

pub type ConValueFloatType = <ConValueFloat as ControlValue>::ConType;

/// A control that has a value of the type [`ControlTypeBool`].
///
/// Maps to [`ControlValueSetter::Boolean`].
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConValueBoolean(ConTypeBool);

impl From<ConTypeBool> for ConValueBoolean {
	fn from(value: ConTypeBool) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueBoolean {
	type ConType = ConTypeBool;

	fn get_value(&self) -> Self::ConType {
		self.0
	}

	fn set_value(&mut self, value: Self::ConType) {
		self.0 = value;
	}
}

impl From<ConValueBoolean> for ControlValueSetter {
	fn from(value: ConValueBoolean) -> Self {
		ControlValueSetter::Boolean(value.0)
	}
}

pub type ConValueBooleanType = <ConValueBoolean as ControlValue>::ConType;

/// A control that has a value of the type [`ControlTypeString`].
///
/// Maps to [`ControlValueSetter::String`].
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConValueString(ConTypeString);

impl From<ConTypeString> for ConValueString {
	fn from(value: ConTypeString) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueString {
	type ConType = ConTypeString;

	fn get_value(&self) -> Self::ConType {
		self.0.clone()
	}

	fn set_value(&mut self, value: Self::ConType) {
		self.0 = value;
	}
}

impl From<&ConValueString> for ControlValueSetter {
	fn from(value: &ConValueString) -> Self {
		ControlValueSetter::String(value.0.clone())
	}
}

pub type ConValueStringType = <ConValueString as ControlValue>::ConType;

/// A control that has a value of the type [`ControlTypeBytes`].
///
/// Maps to [`ControlValueSetter::Bytes`].
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConValueBytes(ConTypeBytes);

impl From<ConTypeBytes> for ConValueBytes {
	fn from(value: ConTypeBytes) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueBytes {
	type ConType = ConTypeBytes;

	fn get_value(&self) -> Self::ConType {
		self.0.clone()
	}

	fn set_value(&mut self, value: Self::ConType) {
		self.0 = value;
	}
}

impl From<&ConValueBytes> for ControlValueSetter {
	fn from(value: &ConValueBytes) -> Self {
		ControlValueSetter::Bytes(value.0.clone())
	}
}

pub type ConValueBytesType = <ConValueBytes as ControlValue>::ConType;

/// A control that has a value of the type [`KeyValue`].
///
/// Maps to [`ControlValueSetter::KeyValue`].
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConValueKeyValue(ConTypeKeyValue);

impl From<ConTypeKeyValue> for ConValueKeyValue {
	fn from(value: ConTypeKeyValue) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueKeyValue {
	type ConType = ConTypeKeyValue;

	fn get_value(&self) -> Self::ConType {
		self.0
	}

	fn set_value(&mut self, value: Self::ConType) {
		self.0 = value;
	}
}

impl From<ConValueKeyValue> for ControlValueSetter {
	fn from(value: ConValueKeyValue) -> Self {
		value.0.into()
	}
}

pub type ConValueKeyValueType = <ConValueKeyValue as ControlValue>::ConType;

/// A control that has a value of the type [`Point`].
///
/// Maps to [`ControlValueSetter::Point`].
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ConValuePoint(ConTypePoint);

impl From<ConTypePoint> for ConValuePoint {
	fn from(value: ConTypePoint) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValuePoint {
	type ConType = ConTypePoint;

	fn get_value(&self) -> Self::ConType {
		self.0
	}

	fn set_value(&mut self, value: Self::ConType) {
		self.0 = value;
	}
}

impl From<ConValuePoint> for ControlValueSetter {
	fn from(value: ConValuePoint) -> Self {
		value.0.into()
	}
}

pub type ConValuePointType = <ConValuePoint as ControlValue>::ConType;

/// A control that has a value of the type [`ControlTypeEnum`].
///
/// Maps to [`ControlValueSetter::EnumValue`].
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConValueEnum(ConTypeEnum);

impl From<ConTypeEnum> for ConValueEnum {
	fn from(value: ConTypeEnum) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueEnum {
	type ConType = ConTypeEnum;

	fn get_value(&self) -> Self::ConType {
		self.0
	}

	fn set_value(&mut self, value: Self::ConType) {
		self.0 = value;
	}
}

impl From<ConValueEnum> for ControlValueSetter {
	fn from(value: ConValueEnum) -> Self {
		ControlValueSetter::EnumValue(value.0)
	}
}

pub type ConValueEnumType = <ConValueEnum as ControlValue>::ConType;

/// A control that has a value of the type [`RGB`].
///
/// Maps to [`ControlValueSetter::RGB`].
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ConValueRGB(ConTypeRGB);

impl From<ConTypeRGB> for ConValueRGB {
	fn from(value: ConTypeRGB) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueRGB {
	type ConType = ConTypeRGB;

	fn get_value(&self) -> Self::ConType {
		self.0
	}

	fn set_value(&mut self, value: Self::ConType) {
		self.0 = value;
	}
}

impl From<ConValueRGB> for ControlValueSetter {
	fn from(value: ConValueRGB) -> Self {
		value.0.into()
	}
}

pub type ConValueRGBType = <ConValueRGB as ControlValue>::ConType;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConValueMenu(ConTypeMenu);

impl From<ConTypeMenu> for ConValueMenu {
	fn from(value: ConTypeMenu) -> Self {
		Self(value)
	}
}

impl ControlValue for ConValueMenu {
	type ConType = ConTypeMenu;

	fn get_value(&self) -> Self::ConType {
		self.0
	}

	fn set_value(&mut self, value: Self::ConType) {
		self.0 = value;
	}
}

impl From<ConValueMenu> for ControlValueSetter {
	fn from(value: ConValueMenu) -> Self {
		ControlValueSetter::Menu(value.0)
	}
}

pub type ConValueMenuType = <ConValueMenu as ControlValue>::ConType;
