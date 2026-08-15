//! Core traits for camera controls.

#[allow(unused_imports, reason = "Needed for trait docstring")]
use std::hash::Hash;

use std::fmt::Debug;

use super::KeyValue;

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
impl ValidControlValue for i64 {} // Integer, IntegerRange, Enum
impl ValidControlValue for f64 {} // Float,  FloatRange
impl ValidControlValue for bool {} // Boolean
impl ValidControlValue for String {} // String
impl ValidControlValue for Vec<u8> {} // Bytes
impl ValidControlValue for KeyValue {} // KeyValuePair
// TODO: Would these be better than the implicit?
// TODO: Would it be better to use an array in either case?
impl ValidControlValue for (f64, f64) {} // Point
//impl ValidControlValue for [f64; 2] {} // Point
impl ValidControlValue for (f64, f64, f64) {} // RGB
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

/// The type that is use to report the control's current value as well as apply a new value.
///
/// Maps to [`nokhwa`]'s [`ControlValueSetter`].
pub trait ControlValue {
	type ValueType: ValidControlValue;

	fn get_value(&self) -> Self::ValueType;
}

/// A type that supports the `min` field.
///
/// Equal to [`ControlValue`].
pub trait ControlMin: ControlValue {
	//type MinType = Self::ValueType;
	type MinType;

	fn get_min(&self) -> Self::MinType;
}

/// A type that supports the `max` field.
///
/// Equal to [`ControlValue`].
pub trait ControlMax: ControlValue {
	//type MaxType = Self::ValueType;
	type MaxType;

	fn get_max(&self) -> Self::MaxType;
}

/// A type that supports the `step` field.
///
/// Equal to [`ControlValue`].
pub trait ControlStep: ControlValue {
	//type StepType = Self::ValueType;
	type StepType;

	fn get_step(&self) -> Self::StepType;
}

/// A type that supports the `key` field.
///
/// Equal to [`ControlValue`].
pub trait ControlKey: ControlValue {
	//type KeyType = Self::ValueType;
	type KeyType;

	fn get_key(&self) -> Self::KeyType;
}

/// A type that supports the `possible` field.
///
/// Bound by [`ControlValue`].
pub trait ControlPossible: ControlValue {
	//type PossibleType = Self::ValueType;
	type PossibleType;

	fn get_possible(&self) -> Self::PossibleType;
}

/// A type that supports the `default` field.
///
/// Bound by [`ControlValue`].
pub trait ControlDefault: ControlValue {
	//type DefaultType = Self::ValueType;
	type DefaultType;

	fn get_default(&self) -> Self::DefaultType;
}
