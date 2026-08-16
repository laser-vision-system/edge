//! Core traits for camera controls.

#[allow(unused_imports, reason = "Needed for trait docstring")]
use std::hash::Hash;

use std::fmt::Debug;

use nokhwa::{self, utils::KnownCameraControl};

use super::{
	KeyValue, Point, RGB,
	types::{
		ControlTypeBool, ControlTypeBytes, ControlTypeEnum, ControlTypeFloat, ControlTypeInteger,
		ControlTypeString,
	},
};

/// A marker trait that determines what is acceptable to use within a [`ControlValue`].
///
/// Cannot be bounded by ...; due to:
///
/// #### [`Copy`]
/// - [`String`], [`Vec`]
/// #### [`Hash`], [`Eq`], [`Ord`]
/// - [`f64`]
pub trait ValidControlValue: Clone + Debug + PartialEq + PartialOrd {}

// ===== Value fields (used in ControlValueSetter) =====
// ----- Explicit -----
impl ValidControlValue for ControlTypeInteger {} // Integer, IntegerRange, Enum
impl ValidControlValue for ControlTypeFloat {} // Float,  FloatRange
impl ValidControlValue for ControlTypeBool {} // Boolean
impl ValidControlValue for ControlTypeString {} // String
impl ValidControlValue for ControlTypeBytes {} // Bytes
impl ValidControlValue for KeyValue {} // KeyValuePair
impl ValidControlValue for Point {} // KeyValuePair
impl ValidControlValue for RGB {} // KeyValuePair
// TODO: Would these be better than the implicit?
// TODO: Would it be better to use an array in either case?
//impl ValidControlValue for (f64, f64) {} // Point
//impl ValidControlValue for [f64; 2] {} // Point
//impl ValidControlValue for (f64, f64, f64) {} // RGB
//impl ValidControlValue for [f64; 3] {} // RGB

// ----- Implicit -----
// Guarantees that Point and RGB will be tuples of matching types.
//impl<T> ValidControlValue for (T, T) where T: ValidControlValue {} // Point
//impl<T> ValidControlValue for (T, T, T) where T: ValidControlValue {} // RGB
//impl<T> ValidControlValue for [T; 2] where T: ValidControlValue {} // Point
//impl<T> ValidControlValue for [T; 3] where T: ValidControlValue {} // RGB

// ===== Other fields =====
//impl ValidControlValue for Vec<i64> {} // Enum: possible
//impl ValidControlValue for Option<String> {} // String: default
//impl ValidControlValue for (i128, i128) {} // KeyValuePair: default

pub trait ValidDefaultValue: Clone + Debug + PartialEq + PartialOrd {}

impl<T> ValidDefaultValue for T where T: ValidControlValue {}

impl ValidDefaultValue for Option<ControlTypeString> {}

pub trait ValidPossibleValue: Clone + Debug + PartialEq + PartialOrd {}

impl<T> ValidPossibleValue for T where T: ValidControlValue {}

impl ValidPossibleValue for Vec<ControlTypeEnum> {}

/// The type that is use to report the control's current value as well as apply a new value.
///
/// Maps to [`nokhwa`]'s [`ControlValueSetter`].
pub trait ControlValue {
	type ValueType: ValidControlValue;

	fn get_value(&self) -> Self::ValueType;
	fn set_value(
		&mut self,
		id: KnownCameraControl,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<(), nokhwa::NokhwaError>;
}

/// A type that wraps a [`ControlValue`] with other control fields.
pub trait ControlStruct {
	type ControlValueType;
	type ValueType: ValidControlValue;

	fn get_id(&self) -> KnownCameraControl;
	fn get_value(&self) -> Self::ControlValueType;
	fn set_value(
		&mut self,
		value: Self::ControlValueType,
		cam: &mut nokhwa::StreamCamera,
	) -> Result<Result<(), nokhwa::NokhwaError>, ()>;
}

/// A type that supports the `default` field.
///
/// Bound by [`ControlValue`].
pub trait ControlDefault: ControlStruct {
	//type DefaultType = Self::ValueType;
	type DefaultType: ValidDefaultValue;

	fn get_default(&self) -> Self::DefaultType;
	//fn reset_to_default(&mut self);
}

/// A type that supports the `min` field.
///
/// Equal to [`ControlValue`].
pub trait ControlMin: ControlStruct {
	//type MinType = Self::ValueType;
	type MinType: ValidControlValue;

	fn get_min(&self) -> Self::MinType;

	fn valid(&self, value: Self::MinType) -> bool {
		value >= self.get_min()
	}
}

/// A type that supports the `max` field.
///
/// Equal to [`ControlValue`].
pub trait ControlMax: ControlStruct {
	//type MaxType = Self::ValueType;
	type MaxType: ValidControlValue;

	fn get_max(&self) -> Self::MaxType;

	fn valid(&self, value: Self::MaxType) -> bool {
		value <= self.get_max()
	}
}

/// A type that supports the `step` field.
///
/// Equal to [`ControlValue`].
pub trait ControlStep: ControlStruct {
	//type StepType = Self::ValueType;
	type StepType: ValidControlValue;

	fn get_step(&self) -> Self::StepType;
	// TODO: Default implementation.
	fn valid(&self, value: Self::StepType) -> bool;
}

/// A type that supports the `possible` field.
///
/// Bound by [`ControlValue`].
pub trait ControlPossible: ControlStruct {
	//type PossibleType = Self::ValueType;
	type PossibleType: ValidPossibleValue;

	fn get_possible(&self) -> Self::PossibleType;
	// TODO: Default implementation.
	fn valid(&self, value: &Self::ValueType) -> bool;
}
