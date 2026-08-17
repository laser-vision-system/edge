//! Types that map to [`nokhwa`]'s [`ControlValueSetter`].

use nokhwa::{
	self,
	utils::{ControlValueSetter, KnownCameraControl},
};

use super::{
	traits::ControlValue,
	types::{
		ConTypeBool, ConTypeBytes, ConTypeEnum, ConTypeFloat, ConTypeInt, ConTypeKeyValue,
		ConTypePoint, ConTypeRGB, ConTypeString,
	},
};

// ==================== READ HERE FIRST ====================
//
// It may seems silly to have `ControlValue...` over a
// matching `ControlType...`, but this is so that the
// underlying type matches in higher level implementations
// as well as make it easier to switch to a different type
// if necessary.
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

	fn set_value(
		&mut self,
		id: KnownCameraControl,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<(), nokhwa::NokhwaError> {
		let value = ControlValueSetter::Integer(self.0);
		cam.set_control(id, value)
	}
}

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

	fn set_value(
		&mut self,
		id: KnownCameraControl,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<(), nokhwa::NokhwaError> {
		let value = ControlValueSetter::Float(self.0);
		cam.set_control(id, value)
	}
}

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

	fn set_value(
		&mut self,
		id: KnownCameraControl,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<(), nokhwa::NokhwaError> {
		let value = ControlValueSetter::Boolean(self.0);
		cam.set_control(id, value)
	}
}

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

	fn set_value(
		&mut self,
		id: KnownCameraControl,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<(), nokhwa::NokhwaError> {
		let value = ControlValueSetter::String(self.0.clone());
		cam.set_control(id, value)
	}
}

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

	fn set_value(
		&mut self,
		id: KnownCameraControl,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<(), nokhwa::NokhwaError> {
		let value = ControlValueSetter::Bytes(self.0.clone());
		cam.set_control(id, value)
	}
}

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

	fn set_value(
		&mut self,
		id: KnownCameraControl,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<(), nokhwa::NokhwaError> {
		let value = ControlValueSetter::KeyValue(self.0.get_key(), self.0.get_value());
		cam.set_control(id, value)
	}
}

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

	fn set_value(
		&mut self,
		id: KnownCameraControl,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<(), nokhwa::NokhwaError> {
		let value = ControlValueSetter::Point(self.0.get_x(), self.0.get_y());
		cam.set_control(id, value)
	}
}

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

	fn set_value(
		&mut self,
		id: KnownCameraControl,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<(), nokhwa::NokhwaError> {
		let value = ControlValueSetter::EnumValue(self.0);
		cam.set_control(id, value)
	}
}

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

	fn set_value(
		&mut self,
		id: KnownCameraControl,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<(), nokhwa::NokhwaError> {
		let value = ControlValueSetter::RGB(self.0.get_r(), self.0.get_g(), self.0.get_b());
		cam.set_control(id, value)
	}
}
