//! Types that map to [`nokhwa`]'s [`ControlValueSetter`].

use nokhwa::{
	self,
	utils::{ControlValueSetter, KnownCameraControl},
};

use super::{
	traits::ControlValue,
	types::{
		ControlTypeBool, ControlTypeBytes, ControlTypeEnum, ControlTypeFloat, ControlTypeInteger,
		ControlTypeString, KeyValue, Point, RGB,
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
pub struct ControlValueInteger(ControlTypeInteger);

impl From<ControlTypeInteger> for ControlValueInteger {
	fn from(value: ControlTypeInteger) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueInteger {
	type ValueType = ControlTypeInteger;

	fn get_value(&self) -> Self::ValueType {
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
pub struct ControlValueFloat(ControlTypeFloat);

impl From<ControlTypeFloat> for ControlValueFloat {
	fn from(value: ControlTypeFloat) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueFloat {
	type ValueType = ControlTypeFloat;

	fn get_value(&self) -> Self::ValueType {
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
pub struct ControlValueBoolean(ControlTypeBool);

impl From<ControlTypeBool> for ControlValueBoolean {
	fn from(value: ControlTypeBool) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueBoolean {
	type ValueType = ControlTypeBool;

	fn get_value(&self) -> Self::ValueType {
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
pub struct ControlValueString(ControlTypeString);

impl From<ControlTypeString> for ControlValueString {
	fn from(value: ControlTypeString) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueString {
	type ValueType = ControlTypeString;

	fn get_value(&self) -> Self::ValueType {
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
pub struct ControlValueBytes(ControlTypeBytes);

impl From<ControlTypeBytes> for ControlValueBytes {
	fn from(value: ControlTypeBytes) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueBytes {
	type ValueType = ControlTypeBytes;

	fn get_value(&self) -> Self::ValueType {
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
pub struct ControlValueKeyValue(KeyValue);

impl From<KeyValue> for ControlValueKeyValue {
	fn from(value: KeyValue) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueKeyValue {
	type ValueType = KeyValue;

	fn get_value(&self) -> Self::ValueType {
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
pub struct ControlValuePoint(Point);

impl From<Point> for ControlValuePoint {
	fn from(value: Point) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValuePoint {
	type ValueType = Point;

	fn get_value(&self) -> Self::ValueType {
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
pub struct ControlValueEnum(ControlTypeEnum);

impl From<ControlTypeEnum> for ControlValueEnum {
	fn from(value: ControlTypeEnum) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueEnum {
	type ValueType = ControlTypeEnum;

	fn get_value(&self) -> Self::ValueType {
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
pub struct ControlValueRGB(RGB);

impl From<RGB> for ControlValueRGB {
	fn from(value: RGB) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueRGB {
	type ValueType = RGB;

	fn get_value(&self) -> Self::ValueType {
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
