use std::{fmt::Debug, hash::Hash};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyValue(i128, i128);

impl From<(i128, i128)> for KeyValue {
	fn from(value: (i128, i128)) -> Self {
		Self(value.0, value.1)
	}
}

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

// ===== Field relationship bounds =====
// (step, min, max, key): value
// default: value
// default: Option<value>
// possible: Vec<value>
// !!!default: (value, value)!!! Not anymore, see [`KeyValue`].

//#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]

// ===== Control Value =====

/// The type that is use to report the control's current value as well as apply a new value.
///
/// Maps to [`nokhwa`]'s [`ControlValueSetter`].
pub trait ControlValue {
	type ValueType: ValidControlValue;

	fn get_value(&self) -> Self::ValueType;
}

// ===== Control Min =====

/// Type equal to [`ControlValue`].
pub trait ControlMin: ControlValue {
	type MinType;

	fn get_min(&self) -> Self::MinType;
}

// ===== Control Max =====

/// Type equal to [`ControlValue`].
pub trait ControlMax: ControlValue {
	type MaxType;

	fn get_max(&self) -> Self::MaxType;
}

// ===== Control Step =====

/// Type equal to [`ControlValue`].
pub trait ControlStep: ControlValue {
	type StepType;

	fn get_step(&self) -> Self::StepType;
}

// ===== Control Key =====

/// Type equal to [`ControlValue`].
pub trait ControlKey: ControlValue {
	type KeyType;

	fn get_key(&self) -> Self::KeyType;
}

// ===== Control Possible =====

/// Type bound by [`ControlValue`].
pub trait ControlPossible: ControlValue {
	type PossibleType;

	fn get_possible(&self) -> Self::PossibleType;
}

// ===== Control Default =====

/// Type bound by [`ControlValue`].
pub trait ControlDefault: ControlValue {
	type DefaultType;

	fn get_default(&self) -> Self::DefaultType;
}
