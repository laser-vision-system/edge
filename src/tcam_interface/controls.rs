//! Abstracts over [`nokhwa`]'s camera controls, providing a simplier interface for use.
//!
//! ## Design Notes
//!
//! - The output needs to be stored in a collection of sorts.
//!
//! - The order of the collection does not matter.
//!
//! - Each control will be unique to a [`KnownCameraControl`] (ID), condusive to a HashMap or Set.
//!
//! - Every [`CamControl`] ([`ControlValueDescription`]) maps non-commutatively to a [`ControlValue`] ([`ControlValueSetter`]), but cannot be determined at runtime.
//! This can be handled either by generic or trait objects (`dyn`).
//!
//! - There seems to be a weak and unclear mapping between [`KnownCameraControl`] and [`CamControl`] ([`ControlValueDescription`]).
//!
//! - What is stored in the collection either needs to be an enum or trait object.
//!
//! - The only important mapping that needs to be sent back to [`nokhwa`] is [`ControlValue`] ([`ControlValueSetter`]).

#[allow(unused_imports, reason = "These imports are needed for docstrings.")]
use nokhwa::utils::{
	CameraControl, ControlValueDescription, ControlValueSetter, KnownCameraControl,
};

pub mod extraction;
pub mod types;
use types::{
	ControlDefault, ControlMax, ControlMin, ControlPossible, ControlStep, ControlValue, KeyValue,
};

// TODO: The inner types could be composed.
// Control Value should be read-only and internally managed.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub enum CamControl {
	#[default]
	None, //None(NoneControl),
	Integer(IntegerControl),
	IntegerRange(IntegerRangeControl),
	Float(FloatControl),
	FloatRange(FloatRangeControl),
	Boolean(BooleanControl),
	String(StringControl),
	Bytes(BytesControl),
	KeyValuePair(KeyValuePairControl),
	Point(PointControl),
	Enum(EnumControl),
	RGB(RGBControl),
}

//pub struct ControlValueNone;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlValueInteger(i64);

impl ControlValueInteger {
	pub fn new(value: i64) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueInteger {
	type ValueType = i64;

	fn get_value(&self) -> Self::ValueType {
		self.0
	}
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ControlValueFloat(f64);

impl ControlValueFloat {
	pub fn new(value: f64) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueFloat {
	type ValueType = f64;

	fn get_value(&self) -> Self::ValueType {
		self.0
	}
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlValueBoolean(bool);

impl ControlValueBoolean {
	pub fn new(value: bool) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueBoolean {
	type ValueType = bool;

	fn get_value(&self) -> Self::ValueType {
		self.0
	}
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlValueString(String);

impl ControlValueString {
	pub fn new(value: String) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueString {
	type ValueType = String;

	fn get_value(&self) -> Self::ValueType {
		self.0.clone()
	}
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlValueBytes(Vec<u8>);

impl ControlValueBytes {
	pub fn new(value: Vec<u8>) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueBytes {
	type ValueType = Vec<u8>;

	fn get_value(&self) -> Self::ValueType {
		self.0.clone()
	}
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlValueKeyValue(KeyValue);

impl ControlValueKeyValue {
	pub fn new(value: KeyValue) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueKeyValue {
	type ValueType = KeyValue;

	fn get_value(&self) -> Self::ValueType {
		self.0
	}
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ControlValuePoint(f64, f64);

impl ControlValuePoint {
	pub fn new(value: (f64, f64)) -> Self {
		Self(value.0, value.1)
	}
}

impl ControlValue for ControlValuePoint {
	type ValueType = (f64, f64);

	fn get_value(&self) -> Self::ValueType {
		(self.0, self.1)
	}
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlValueEnum(i64);

impl ControlValueEnum {
	pub fn new(value: i64) -> Self {
		Self(value)
	}
}

impl ControlValue for ControlValueEnum {
	type ValueType = i64;

	fn get_value(&self) -> Self::ValueType {
		self.0
	}
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ControlValueRGB(f64, f64, f64);

impl ControlValueRGB {
	pub fn new(value: (f64, f64, f64)) -> Self {
		Self(value.0, value.1, value.2)
	}
}

impl ControlValue for ControlValueRGB {
	type ValueType = (f64, f64, f64);

	fn get_value(&self) -> Self::ValueType {
		(self.0, self.1, self.2)
	}
}

// ===== ControlValueDescription Mappings =====

// Every variant has a value and default.
// There are no hard correllations for the other fields.

// ControlValueDescription::None

//#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
//pub struct NoneControl;

// ControlValueDescription::Integer

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerControl {
	value: ControlValueInteger,
	default: <ControlValueInteger as ControlValue>::ValueType,
	step: <ControlValueInteger as ControlValue>::ValueType,
}

impl IntegerControl {
	pub fn new(
		value: ControlValueInteger,
		default: <ControlValueInteger as ControlValue>::ValueType,
		step: <ControlValueInteger as ControlValue>::ValueType,
	) -> Self {
		Self {
			value,
			default,
			step,
		}
	}
}

impl ControlValue for IntegerControl {
	type ValueType = i64;

	fn get_value(&self) -> Self::ValueType {
		self.value.get_value()
	}
}

impl ControlDefault for IntegerControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlStep for IntegerControl {
	type StepType = Self::ValueType;

	fn get_step(&self) -> Self::StepType {
		self.step
	}
}

// ControlValueDescription::IntegerRange

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerRangeControl {
	value: ControlValueInteger,
	default: <ControlValueInteger as ControlValue>::ValueType,
	step: <ControlValueInteger as ControlValue>::ValueType,
	min: <ControlValueInteger as ControlValue>::ValueType,
	max: <ControlValueInteger as ControlValue>::ValueType,
}

impl IntegerRangeControl {
	pub fn new(
		value: ControlValueInteger,
		default: <ControlValueInteger as ControlValue>::ValueType,
		step: <ControlValueInteger as ControlValue>::ValueType,
		min: <ControlValueInteger as ControlValue>::ValueType,
		max: <ControlValueInteger as ControlValue>::ValueType,
	) -> Self {
		Self {
			value,
			default,
			step,
			min,
			max,
		}
	}
}

impl ControlValue for IntegerRangeControl {
	type ValueType = i64;

	fn get_value(&self) -> Self::ValueType {
		self.value.get_value()
	}
}

impl ControlDefault for IntegerRangeControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlStep for IntegerRangeControl {
	type StepType = Self::ValueType;

	fn get_step(&self) -> Self::StepType {
		self.step
	}
}

impl ControlMin for IntegerRangeControl {
	type MinType = Self::ValueType;

	fn get_min(&self) -> Self::MinType {
		self.min
	}
}

impl ControlMax for IntegerRangeControl {
	type MaxType = Self::ValueType;

	fn get_max(&self) -> Self::MaxType {
		self.max
	}
}

// ControlValueDescription::Float

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct FloatControl {
	value: ControlValueFloat,
	default: <ControlValueFloat as ControlValue>::ValueType,
	step: <ControlValueFloat as ControlValue>::ValueType,
}

impl FloatControl {
	pub fn new(
		value: ControlValueFloat,
		default: <ControlValueFloat as ControlValue>::ValueType,
		step: <ControlValueFloat as ControlValue>::ValueType,
	) -> Self {
		Self {
			value,
			default,
			step,
		}
	}
}

impl ControlValue for FloatControl {
	type ValueType = f64;

	fn get_value(&self) -> Self::ValueType {
		self.value.get_value()
	}
}

impl ControlDefault for FloatControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlStep for FloatControl {
	type StepType = Self::ValueType;

	fn get_step(&self) -> Self::StepType {
		self.step
	}
}

// ControlValueDescription::FloatRange

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct FloatRangeControl {
	value: ControlValueFloat,
	default: <ControlValueFloat as ControlValue>::ValueType,
	step: <ControlValueFloat as ControlValue>::ValueType,
	min: <ControlValueFloat as ControlValue>::ValueType,
	max: <ControlValueFloat as ControlValue>::ValueType,
}

impl FloatRangeControl {
	pub fn new(
		value: ControlValueFloat,
		default: <ControlValueFloat as ControlValue>::ValueType,
		step: <ControlValueFloat as ControlValue>::ValueType,
		min: <ControlValueFloat as ControlValue>::ValueType,
		max: <ControlValueFloat as ControlValue>::ValueType,
	) -> Self {
		Self {
			value,
			default,
			step,
			min,
			max,
		}
	}
}

impl ControlValue for FloatRangeControl {
	type ValueType = f64;

	fn get_value(&self) -> Self::ValueType {
		self.value.get_value()
	}
}

impl ControlDefault for FloatRangeControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlStep for FloatRangeControl {
	type StepType = Self::ValueType;

	fn get_step(&self) -> Self::StepType {
		self.step
	}
}

impl ControlMin for FloatRangeControl {
	type MinType = Self::ValueType;

	fn get_min(&self) -> Self::MinType {
		self.min
	}
}

impl ControlMax for FloatRangeControl {
	type MaxType = Self::ValueType;

	fn get_max(&self) -> Self::MaxType {
		self.max
	}
}

// ControlValueDescription::Boolean

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BooleanControl {
	value: ControlValueBoolean,
	default: <ControlValueBoolean as ControlValue>::ValueType,
}

impl BooleanControl {
	pub fn new(
		value: ControlValueBoolean,
		default: <ControlValueBoolean as ControlValue>::ValueType,
	) -> Self {
		Self { value, default }
	}
}

impl ControlValue for BooleanControl {
	type ValueType = bool;

	fn get_value(&self) -> Self::ValueType {
		self.value.get_value()
	}
}

impl ControlDefault for BooleanControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

// ControlValueDescription::String

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringControl {
	value: ControlValueString,
	default: Option<<ControlValueString as ControlValue>::ValueType>,
}

impl StringControl {
	pub fn new(
		value: ControlValueString,
		default: Option<<ControlValueString as ControlValue>::ValueType>,
	) -> Self {
		Self { value, default }
	}
}

impl ControlValue for StringControl {
	type ValueType = String;

	fn get_value(&self) -> Self::ValueType {
		self.value.get_value()
	}
}

impl ControlDefault for StringControl {
	type DefaultType = Option<Self::ValueType>;

	fn get_default(&self) -> Self::DefaultType {
		self.default.clone()
	}
}

// ControlValueDescription::Bytes

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BytesControl {
	value: ControlValueBytes,
	default: <ControlValueBytes as ControlValue>::ValueType,
}

impl BytesControl {
	pub fn new(
		value: ControlValueBytes,
		default: <ControlValueBytes as ControlValue>::ValueType,
	) -> Self {
		Self { value, default }
	}
}

impl ControlValue for BytesControl {
	type ValueType = Vec<u8>;

	fn get_value(&self) -> Self::ValueType {
		self.value.get_value()
	}
}

impl ControlDefault for BytesControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default.clone()
	}
}

// ControlValueDescription::KeyValuePair

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyValuePairControl {
	value: ControlValueKeyValue,
	default: <ControlValueKeyValue as ControlValue>::ValueType,
}

impl KeyValuePairControl {
	pub fn new(
		value: ControlValueKeyValue,
		default: <ControlValueKeyValue as ControlValue>::ValueType,
	) -> Self {
		Self { value, default }
	}
}

impl ControlValue for KeyValuePairControl {
	type ValueType = KeyValue;

	fn get_value(&self) -> Self::ValueType {
		self.value.get_value()
	}
}

impl ControlDefault for KeyValuePairControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

// ControlValueDescription::Point

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PointControl {
	value: ControlValuePoint,
	default: <ControlValuePoint as ControlValue>::ValueType,
}

impl PointControl {
	pub fn new(
		value: ControlValuePoint,
		default: <ControlValuePoint as ControlValue>::ValueType,
	) -> Self {
		Self { value, default }
	}
}

impl ControlValue for PointControl {
	type ValueType = (f64, f64);

	fn get_value(&self) -> Self::ValueType {
		self.value.get_value()
	}
}

impl ControlDefault for PointControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

// ControlValueDescription::Enum

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumControl {
	value: ControlValueEnum,
	default: <ControlValueEnum as ControlValue>::ValueType,
	possible: Vec<<ControlValueEnum as ControlValue>::ValueType>,
}

impl EnumControl {
	pub fn new(
		value: ControlValueEnum,
		default: <ControlValueEnum as ControlValue>::ValueType,
		possible: Vec<<ControlValueEnum as ControlValue>::ValueType>,
	) -> Self {
		Self {
			value,
			default,
			possible,
		}
	}
}

impl ControlValue for EnumControl {
	type ValueType = i64;

	fn get_value(&self) -> Self::ValueType {
		self.value.get_value()
	}
}

impl ControlDefault for EnumControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlPossible for EnumControl {
	type PossibleType = Vec<Self::ValueType>;

	fn get_possible(&self) -> Self::PossibleType {
		self.possible.clone()
	}
}

// ControlValueDescription::RGB

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RGBControl {
	value: ControlValueRGB,
	default: <ControlValueRGB as ControlValue>::ValueType,
	max: <ControlValueRGB as ControlValue>::ValueType,
}

impl RGBControl {
	pub fn new(
		value: ControlValueRGB,
		default: <ControlValueRGB as ControlValue>::ValueType,
		max: <ControlValueRGB as ControlValue>::ValueType,
	) -> Self {
		Self {
			value,
			default,
			max,
		}
	}
}

impl ControlValue for RGBControl {
	type ValueType = (f64, f64, f64);

	fn get_value(&self) -> Self::ValueType {
		self.value.get_value()
	}
}

impl ControlDefault for RGBControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlMax for RGBControl {
	type MaxType = Self::ValueType;

	fn get_max(&self) -> Self::MaxType {
		self.max
	}
}
