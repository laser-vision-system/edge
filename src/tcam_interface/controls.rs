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

use std::collections::HashMap;

#[allow(unused_imports, reason = "Needed for module docstring")]
use nokhwa::utils::ControlValueSetter;

use nokhwa::utils::{CameraControl, ControlValueDescription, KnownCameraControl};

pub mod traits;
pub mod types;
pub mod values;

use traits::{ControlDefault, ControlMax, ControlMin, ControlPossible, ControlStep, ControlValue};

use types::{KeyValue, Point, RGB};

use values::{
	ControlValueBoolean, ControlValueBytes, ControlValueEnum, ControlValueFloat,
	ControlValueInteger, ControlValueKeyValue, ControlValuePoint, ControlValueRGB,
	ControlValueString,
};

use self::types::{
	ControlTypeBool, ControlTypeBytes, ControlTypeEnum, ControlTypeFloat, ControlTypeInteger,
	ControlTypeString,
};

pub type ControlMap = HashMap<KnownCameraControl, CamControl>;

/// Convert a collection of [`nokhwa`] [`CameraControl`] into a custom type.
pub fn extract_controls(nokhwa_controls: Vec<CameraControl>) -> ControlMap {
	nokhwa_controls
		.into_iter()
		.map(|control| (control.control(), control.description().into()))
		.collect::<ControlMap>()
}

/// Convert a single [`nokhwa`] [`CameraControl`] into a custom type.
pub fn extract_control(nokhwa_control: CameraControl) -> CamControl {
	nokhwa_control.description().into()
}

// ===== Field relationship bounds =====
// (step, min, max, key): value
// default: value
// default: Option<value>
// possible: Vec<value>
// !!!default: (value, value)!!! Not anymore, see [`KeyValue`].

// TODO: The inner types could be composed.
// Control Value should be read-only and internally managed.
/// Represents one type of camera control.
///
/// Maps to [`ControlValueDescription`].
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

impl From<&ControlValueDescription> for CamControl {
	fn from(value: &ControlValueDescription) -> Self {
		match value {
			ControlValueDescription::None => CamControl::None,
			ControlValueDescription::Integer {
				value,
				default,
				step,
			} => CamControl::Integer(IntegerControl::new(
				ControlValueInteger::from(*value),
				*default,
				*step,
			)),
			ControlValueDescription::IntegerRange {
				min,
				max,
				value,
				step,
				default,
			} => CamControl::IntegerRange(IntegerRangeControl::new(
				ControlValueInteger::from(*value),
				*default,
				*step,
				*min,
				*max,
			)),
			ControlValueDescription::Float {
				value,
				default,
				step,
			} => CamControl::Float(FloatControl::new(
				ControlValueFloat::from(*value),
				*default,
				*step,
			)),
			ControlValueDescription::FloatRange {
				min,
				max,
				value,
				step,
				default,
			} => CamControl::FloatRange(FloatRangeControl::new(
				ControlValueFloat::from(*value),
				*default,
				*step,
				*min,
				*max,
			)),
			ControlValueDescription::Boolean { value, default } => CamControl::Boolean(
				BooleanControl::new(ControlValueBoolean::from(*value), *default),
			),
			ControlValueDescription::String { value, default } => CamControl::String(
				StringControl::new(ControlValueString::from(value.clone()), default.clone()),
			),
			ControlValueDescription::Bytes { value, default } => CamControl::Bytes(
				BytesControl::new(ControlValueBytes::from(value.clone()), default.clone()),
			),
			ControlValueDescription::KeyValuePair {
				key,
				value,
				default,
			} => CamControl::KeyValuePair(KeyValuePairControl::new(
				ControlValueKeyValue::from(KeyValue::from((*key, *value))),
				(*default).into(),
			)),
			ControlValueDescription::Point { value, default } => {
				CamControl::Point(PointControl::new(
					ControlValuePoint::from(Point::from(*value)),
					Point::from(*default),
				))
			},
			ControlValueDescription::Enum {
				value,
				possible,
				default,
			} => CamControl::Enum(EnumControl::new(
				ControlValueEnum::from(*value),
				*default,
				possible.clone(),
			)),
			ControlValueDescription::RGB {
				value,
				max,
				default,
			} => CamControl::RGB(RGBControl::new(
				ControlValueRGB::from(RGB::from(*value)),
				RGB::from(*default),
				RGB::from(*max),
			)),
		}
	}
}

// Every variant has a value and default.
// There are no hard correllations for the other fields.

/// A camera control that consists of nothing.
///
/// Maps to [`ControlValueDescription::None`].
//#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
//pub struct NoneControl;

/// A camera control that consists of a value, default, and step.
///
/// Maps to [`ControlValueDescription::Integer`]
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
	type ValueType = ControlTypeInteger;

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

/// A camera control that consists of a value, default, step, min, and max.
///
/// Maps to [`ControlValueDescription::IntegerRange`]

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
	type ValueType = ControlTypeInteger;

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

/// A camera control that consists of a value, default, and step.
///
/// Maps to [`ControlValueDescription::Float`]

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
	type ValueType = ControlTypeFloat;

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

/// A camera control that consists of a value, default, step, min, and max.
///
/// Maps to [`ControlValueDescription::FloatRange`]

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
	type ValueType = ControlTypeFloat;

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

/// A camera control that consists of a value and default.
///
/// Maps to [`ControlValueDescription::Boolean`]

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
	type ValueType = ControlTypeBool;

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

/// A camera control that consists of a value and an optional default.
///
/// Maps to [`ControlValueDescription::String`]

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
	type ValueType = ControlTypeString;

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

/// A camera control that consists of a value and default.
///
/// Maps to [`ControlValueDescription::Bytes`]

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
	type ValueType = ControlTypeBytes;

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

/// A camera control that consists of a value and default.
///
/// Maps to [`ControlValueDescription::KeyValuePair`]

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

/// A camera control that consists of a value and default.
///
/// Maps to [`ControlValueDescription::Point`]

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
	type ValueType = Point;

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

/// A camera control that consists of a value, default, and a vector of possible options.
///
/// Maps to [`ControlValueDescription::Enum`]

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
	type ValueType = ControlTypeEnum;

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

/// A camera control that consists of a value, default, and max.
///
/// The assumption is that the min will always be 0.
///
/// Maps to [`ControlValueDescription::RGB`]

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
	type ValueType = RGB;

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
