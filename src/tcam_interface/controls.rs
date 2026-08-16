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
//!
//! ## TODO: Oh god fix the naming of the types.

use std::{collections::HashMap, hash::Hash};

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

use self::{
	traits::ControlStruct,
	types::{
		ControlTypeBool, ControlTypeBytes, ControlTypeEnum, ControlTypeFloat, ControlTypeInteger,
		ControlTypeString,
	},
};

// TODO: Maybe make this into a struct itself.
pub type ControlMap = HashMap<KnownCameraControl, CamControl>;

/// Convert a collection of [`nokhwa`] [`CameraControl`] into a custom type.
pub fn extract_controls(nokhwa_controls: Vec<CameraControl>) -> ControlMap {
	nokhwa_controls
		.into_iter()
		.map(|control| (control.control(), extract_control(control)))
		.collect::<ControlMap>()
}

/// Convert a single [`nokhwa`] [`CameraControl`] into a custom type.
pub fn extract_control(nokhwa_control: CameraControl) -> CamControl {
	nokhwa_control.into()
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
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<CameraControl> for CamControl {
	fn from(value: CameraControl) -> Self {
		let control_type = value.control();
		match value.description() {
			ControlValueDescription::None => CamControl::None,
			ControlValueDescription::Integer {
				value,
				default,
				step,
			} => CamControl::Integer(IntegerControl::new(
				control_type,
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
				control_type,
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
				control_type,
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
				control_type,
				ControlValueFloat::from(*value),
				*default,
				*step,
				*min,
				*max,
			)),
			ControlValueDescription::Boolean { value, default } => CamControl::Boolean(
				BooleanControl::new(control_type, ControlValueBoolean::from(*value), *default),
			),
			ControlValueDescription::String { value, default } => {
				CamControl::String(StringControl::new(
					control_type,
					ControlValueString::from(value.clone()),
					default.clone(),
				))
			},
			ControlValueDescription::Bytes { value, default } => {
				CamControl::Bytes(BytesControl::new(
					control_type,
					ControlValueBytes::from(value.clone()),
					default.clone(),
				))
			},
			ControlValueDescription::KeyValuePair {
				key,
				value,
				default,
			} => CamControl::KeyValuePair(KeyValuePairControl::new(
				control_type,
				ControlValueKeyValue::from(KeyValue::from((*key, *value))),
				(*default).into(),
			)),
			ControlValueDescription::Point { value, default } => {
				CamControl::Point(PointControl::new(
					control_type,
					ControlValuePoint::from(Point::from(*value)),
					Point::from(*default),
				))
			},
			ControlValueDescription::Enum {
				value,
				possible,
				default,
			} => CamControl::Enum(EnumControl::new(
				control_type,
				ControlValueEnum::from(*value),
				*default,
				possible.clone(),
			)),
			ControlValueDescription::RGB {
				value,
				max,
				default,
			} => CamControl::RGB(RGBControl::new(
				control_type,
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
#[derive(Clone, Copy, Debug)]
pub struct IntegerControl {
	control_type: KnownCameraControl,
	value: ControlValueInteger,
	default: <ControlValueInteger as ControlValue>::ValueType,
	step: <ControlValueInteger as ControlValue>::ValueType,
}

impl IntegerControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ControlValueInteger,
		default: <ControlValueInteger as ControlValue>::ValueType,
		step: <ControlValueInteger as ControlValue>::ValueType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			step,
		}
	}
}

impl ControlStruct for IntegerControl {
	type ControlValueType = ControlValueInteger;
	type ValueType = ControlTypeInteger;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn get_value(&self) -> Self::ControlValueType {
		self.value
	}

	fn set_value(
		&mut self,
		value: Self::ControlValueType,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<Result<(), nokhwa::NokhwaError>, ()> {
		if self.valid(value.get_value()) {
			self.value = value;
			Ok(self.value.set_value(self.get_id(), cam))
		} else {
			Err(())
		}
	}
}

impl ControlDefault for IntegerControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlStep for IntegerControl {
	// Default should be based on ValueType, so this makes sure they can be compared.
	type StepType = Self::ValueType;

	fn get_step(&self) -> Self::StepType {
		self.step
	}

	fn valid(&self, value: Self::StepType) -> bool {
		(value - self.get_default()).abs() % self.get_step() == 0
			|| (value - self.get_value().get_value()).abs() % self.get_step() == 0
	}
}

/// A camera control that consists of a value, default, step, min, and max.
///
/// Maps to [`ControlValueDescription::IntegerRange`]

#[derive(Clone, Copy, Debug)]
pub struct IntegerRangeControl {
	control_type: KnownCameraControl,
	value: ControlValueInteger,
	default: <ControlValueInteger as ControlValue>::ValueType,
	step: <ControlValueInteger as ControlValue>::ValueType,
	min: <ControlValueInteger as ControlValue>::ValueType,
	max: <ControlValueInteger as ControlValue>::ValueType,
}

impl IntegerRangeControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ControlValueInteger,
		default: <ControlValueInteger as ControlValue>::ValueType,
		step: <ControlValueInteger as ControlValue>::ValueType,
		min: <ControlValueInteger as ControlValue>::ValueType,
		max: <ControlValueInteger as ControlValue>::ValueType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			step,
			min,
			max,
		}
	}
}

impl ControlStruct for IntegerRangeControl {
	type ControlValueType = ControlValueInteger;
	type ValueType = ControlTypeInteger;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn get_value(&self) -> Self::ControlValueType {
		self.value
	}

	fn set_value(
		&mut self,
		value: Self::ControlValueType,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<Result<(), nokhwa::NokhwaError>, ()> {
		if ControlMin::valid(self, value.get_value())
			&& ControlMax::valid(self, value.get_value())
			&& ControlStep::valid(self, value.get_value())
		{
			self.value = value;
			Ok(self.value.set_value(self.get_id(), cam))
		} else {
			Err(())
		}
	}
}

impl ControlDefault for IntegerRangeControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlStep for IntegerRangeControl {
	// Default should be based on ValueType, so this makes sure they can be compared.
	type StepType = Self::ValueType;

	fn get_step(&self) -> Self::StepType {
		self.step
	}

	fn valid(&self, value: Self::StepType) -> bool {
		(value - self.get_default()).abs() % self.get_step() == 0
			|| (value - self.get_value().get_value()).abs() % self.get_step() == 0
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

#[derive(Clone, Copy, Debug)]
pub struct FloatControl {
	control_type: KnownCameraControl,
	value: ControlValueFloat,
	default: <ControlValueFloat as ControlValue>::ValueType,
	step: <ControlValueFloat as ControlValue>::ValueType,
}

impl FloatControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ControlValueFloat,
		default: <ControlValueFloat as ControlValue>::ValueType,
		step: <ControlValueFloat as ControlValue>::ValueType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			step,
		}
	}
}

impl ControlStruct for FloatControl {
	type ControlValueType = ControlValueFloat;
	type ValueType = ControlTypeFloat;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn get_value(&self) -> Self::ControlValueType {
		self.value
	}

	fn set_value(
		&mut self,
		value: Self::ControlValueType,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<Result<(), nokhwa::NokhwaError>, ()> {
		if self.valid(value.get_value()) {
			self.value = value;
			Ok(self.value.set_value(self.get_id(), cam))
		} else {
			Err(())
		}
	}
}

impl ControlDefault for FloatControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlStep for FloatControl {
	// Default should be based on ValueType, so this makes sure they can be compared.
	type StepType = Self::ValueType;

	fn get_step(&self) -> Self::StepType {
		self.step
	}

	fn valid(&self, value: Self::StepType) -> bool {
		(value - self.get_default()).abs() % self.get_step() == 0.0
			|| (value - self.get_value().get_value()).abs() % self.get_step() == 0.0
	}
}

/// A camera control that consists of a value, default, step, min, and max.
///
/// Maps to [`ControlValueDescription::FloatRange`]

#[derive(Clone, Copy, Debug)]
pub struct FloatRangeControl {
	control_type: KnownCameraControl,
	value: ControlValueFloat,
	default: <ControlValueFloat as ControlValue>::ValueType,
	step: <ControlValueFloat as ControlValue>::ValueType,
	min: <ControlValueFloat as ControlValue>::ValueType,
	max: <ControlValueFloat as ControlValue>::ValueType,
}

impl FloatRangeControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ControlValueFloat,
		default: <ControlValueFloat as ControlValue>::ValueType,
		step: <ControlValueFloat as ControlValue>::ValueType,
		min: <ControlValueFloat as ControlValue>::ValueType,
		max: <ControlValueFloat as ControlValue>::ValueType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			step,
			min,
			max,
		}
	}
}

impl ControlStruct for FloatRangeControl {
	type ControlValueType = ControlValueFloat;
	type ValueType = ControlTypeFloat;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn get_value(&self) -> Self::ControlValueType {
		self.value
	}

	fn set_value(
		&mut self,
		value: Self::ControlValueType,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<Result<(), nokhwa::NokhwaError>, ()> {
		if ControlMin::valid(self, value.get_value())
			&& ControlMax::valid(self, value.get_value())
			&& ControlStep::valid(self, value.get_value())
		{
			self.value = value;
			Ok(self.value.set_value(self.get_id(), cam))
		} else {
			Err(())
		}
	}
}

impl ControlDefault for FloatRangeControl {
	type DefaultType = Self::ValueType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlStep for FloatRangeControl {
	// Default should be based on ValueType, so this makes sure they can be compared.
	type StepType = Self::ValueType;

	fn get_step(&self) -> Self::StepType {
		self.step
	}

	fn valid(&self, value: Self::StepType) -> bool {
		(value - self.get_default()).abs() % self.get_step() == 0.0
			|| (value - self.get_value().get_value()).abs() % self.get_step() == 0.0
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

#[derive(Clone, Copy, Debug)]
pub struct BooleanControl {
	control_type: KnownCameraControl,
	value: ControlValueBoolean,
	default: <ControlValueBoolean as ControlValue>::ValueType,
}

impl BooleanControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ControlValueBoolean,
		default: <ControlValueBoolean as ControlValue>::ValueType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlStruct for BooleanControl {
	type ControlValueType = ControlValueBoolean;
	type ValueType = ControlTypeBool;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn get_value(&self) -> Self::ControlValueType {
		self.value
	}

	fn set_value(
		&mut self,
		value: Self::ControlValueType,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<Result<(), nokhwa::NokhwaError>, ()> {
		self.value = value;
		Ok(self.value.set_value(self.get_id(), cam))
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

#[derive(Clone, Debug)]
pub struct StringControl {
	control_type: KnownCameraControl,
	value: ControlValueString,
	default: Option<<ControlValueString as ControlValue>::ValueType>,
}

impl StringControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ControlValueString,
		default: Option<<ControlValueString as ControlValue>::ValueType>,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlStruct for StringControl {
	type ControlValueType = ControlValueString;
	type ValueType = ControlTypeString;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn get_value(&self) -> Self::ControlValueType {
		self.value.clone()
	}

	fn set_value(
		&mut self,
		value: Self::ControlValueType,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<Result<(), nokhwa::NokhwaError>, ()> {
		self.value = value;
		Ok(self.value.set_value(self.get_id(), cam))
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

#[derive(Clone, Debug)]
pub struct BytesControl {
	control_type: KnownCameraControl,
	value: ControlValueBytes,
	default: <ControlValueBytes as ControlValue>::ValueType,
}

impl BytesControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ControlValueBytes,
		default: <ControlValueBytes as ControlValue>::ValueType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlStruct for BytesControl {
	type ControlValueType = ControlValueBytes;
	type ValueType = ControlTypeBytes;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn get_value(&self) -> Self::ControlValueType {
		self.value.clone()
	}

	fn set_value(
		&mut self,
		value: Self::ControlValueType,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<Result<(), nokhwa::NokhwaError>, ()> {
		self.value = value;
		Ok(self.value.set_value(self.get_id(), cam))
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

#[derive(Clone, Copy, Debug)]
pub struct KeyValuePairControl {
	control_type: KnownCameraControl,
	value: ControlValueKeyValue,
	default: <ControlValueKeyValue as ControlValue>::ValueType,
}

impl KeyValuePairControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ControlValueKeyValue,
		default: <ControlValueKeyValue as ControlValue>::ValueType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlStruct for KeyValuePairControl {
	type ControlValueType = ControlValueKeyValue;
	type ValueType = KeyValue;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn get_value(&self) -> Self::ControlValueType {
		self.value
	}

	fn set_value(
		&mut self,
		value: Self::ControlValueType,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<Result<(), nokhwa::NokhwaError>, ()> {
		self.value = value;
		Ok(self.value.set_value(self.get_id(), cam))
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

#[derive(Clone, Copy, Debug)]
pub struct PointControl {
	control_type: KnownCameraControl,
	value: ControlValuePoint,
	default: <ControlValuePoint as ControlValue>::ValueType,
}

impl PointControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ControlValuePoint,
		default: <ControlValuePoint as ControlValue>::ValueType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlStruct for PointControl {
	type ControlValueType = ControlValuePoint;
	type ValueType = Point;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn get_value(&self) -> Self::ControlValueType {
		self.value
	}

	fn set_value(
		&mut self,
		value: Self::ControlValueType,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<Result<(), nokhwa::NokhwaError>, ()> {
		self.value = value;
		Ok(self.value.set_value(self.get_id(), cam))
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

#[derive(Clone, Debug)]
pub struct EnumControl {
	control_type: KnownCameraControl,
	value: ControlValueEnum,
	default: <ControlValueEnum as ControlValue>::ValueType,
	possible: Vec<<ControlValueEnum as ControlValue>::ValueType>,
}

impl EnumControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ControlValueEnum,
		default: <ControlValueEnum as ControlValue>::ValueType,
		possible: Vec<<ControlValueEnum as ControlValue>::ValueType>,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			possible,
		}
	}
}

impl ControlStruct for EnumControl {
	type ControlValueType = ControlValueEnum;
	type ValueType = ControlTypeEnum;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn get_value(&self) -> Self::ControlValueType {
		self.value
	}

	fn set_value(
		&mut self,
		value: Self::ControlValueType,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<Result<(), nokhwa::NokhwaError>, ()> {
		if self.valid(&value.get_value()) {
			self.value = value;
			Ok(self.value.set_value(self.get_id(), cam))
		} else {
			Err(())
		}
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

	fn valid(&self, value: &Self::ValueType) -> bool {
		self.possible.contains(value)
	}
}

/// A camera control that consists of a value, default, and max.
///
/// The assumption is that the min will always be 0.
///
/// Maps to [`ControlValueDescription::RGB`]

#[derive(Clone, Copy, Debug)]
pub struct RGBControl {
	control_type: KnownCameraControl,
	value: ControlValueRGB,
	default: <ControlValueRGB as ControlValue>::ValueType,
	max: <ControlValueRGB as ControlValue>::ValueType,
}

impl RGBControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ControlValueRGB,
		default: <ControlValueRGB as ControlValue>::ValueType,
		max: <ControlValueRGB as ControlValue>::ValueType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			max,
		}
	}
}

impl ControlStruct for RGBControl {
	type ControlValueType = ControlValueRGB;
	type ValueType = RGB;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn get_value(&self) -> Self::ControlValueType {
		self.value
	}

	fn set_value(
		&mut self,
		value: Self::ControlValueType,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<Result<(), nokhwa::NokhwaError>, ()> {
		if self.valid(value.get_value()) {
			self.value = value;
			Ok(self.value.set_value(self.get_id(), cam))
		} else {
			Err(())
		}
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

/// Automatically implement [`Hash`], [`PartialEq`], [`Eq`], [`PartialOrd`], and [`Ord`] on a struct by its control id.
macro_rules! impl_identifiable {
	($type:ty) => {
		impl Hash for $type {
			fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
				self.get_id().hash(state);
			}
		}

		impl PartialEq for $type {
			fn eq(&self, other: &Self) -> bool {
				self.get_id() == other.get_id()
			}
		}

		impl Eq for $type {}

		impl PartialOrd for $type {
			fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				self.get_id().partial_cmp(&other.get_id())
			}
		}

		impl Ord for $type {
			fn cmp(&self, other: &Self) -> std::cmp::Ordering {
				self.get_id().cmp(&other.get_id())
			}
		}
	};
}

impl_identifiable!(IntegerControl);
impl_identifiable!(IntegerRangeControl);
impl_identifiable!(FloatControl);
impl_identifiable!(FloatRangeControl);
impl_identifiable!(BooleanControl);
impl_identifiable!(StringControl);
impl_identifiable!(BytesControl);
impl_identifiable!(KeyValuePairControl);
impl_identifiable!(PointControl);
impl_identifiable!(EnumControl);
impl_identifiable!(RGBControl);
