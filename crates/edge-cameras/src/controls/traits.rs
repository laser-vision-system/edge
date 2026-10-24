//! Core traits for camera controls.

use std::fmt::Debug;
#[expect(unused_imports, reason = "Needed for trait docstring")]
use std::hash::Hash;

#[expect(unused_imports, reason = "Needed for trait docstring")]
use nokhwa::utils::ControlValueSetter;
use nokhwa::{
	self,
	utils::{
		KnownCameraControl,
		MenuEntry,
	},
};

use super::types::{
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
};
use crate::error::EdgeCamerasResult;

/// A marker trait that determines what is acceptable to use within a
/// [`ControlValue`].
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
impl ValidControlValue for ConTypeInt {
} // Integer, IntegerRange, Enum
impl ValidControlValue for ConTypeFloat {
} // Float,  FloatRange
impl ValidControlValue for ConTypeBool {
} // Boolean
impl ValidControlValue for ConTypeString {
} // String
impl ValidControlValue for ConTypeBytes {
} // Bytes
impl ValidControlValue for ConTypeKeyValue {
} // KeyValuePair
impl ValidControlValue for ConTypePoint {
} // KeyValuePair
impl ValidControlValue for ConTypeRGB {
} // KeyValuePair
impl ValidControlValue for ConTypeMenu {
} // Menu
// TODO: Would these be better than the implicit?
// TODO: Would it be better to use an array in either case?
// impl ValidControlValue for (f64, f64) {} // Point
// impl ValidControlValue for [f64; 2] {} // Point
// impl ValidControlValue for (f64, f64, f64) {} // RGB
// impl ValidControlValue for [f64; 3] {} // RGB

// ----- Implicit -----
// Guarantees that Point and RGB will be tuples of matching types.
// impl<T> ValidControlValue for (T, T) where T: ValidControlValue {} // Point
// impl<T> ValidControlValue for (T, T, T) where T: ValidControlValue {} // RGB
// impl<T> ValidControlValue for [T; 2] where T: ValidControlValue {} // Point
// impl<T> ValidControlValue for [T; 3] where T: ValidControlValue {} // RGB

// ===== Other fields =====
// impl ValidControlValue for Vec<i64> {} // Enum: possible
// impl ValidControlValue for Option<String> {} // String: default
// impl ValidControlValue for (i128, i128) {} // KeyValuePair: default

pub trait ValidDefaultValue: Clone + Debug + PartialEq + PartialOrd {}

impl<T> ValidDefaultValue for T where T: ValidControlValue
{
}

impl ValidDefaultValue for Option<ConTypeString> {
}

pub trait ValidPossibleValue: Clone + Debug + PartialEq + PartialOrd {}

impl<T> ValidPossibleValue for T where T: ValidControlValue
{
}

impl ValidPossibleValue for Vec<ConTypeEnum> {
}

pub trait ValidItemValue: Clone + Debug + PartialEq + PartialOrd {}

impl ValidItemValue for Vec<MenuEntry> {
}

/// The type that is use to report the control's current value as well as apply
/// a new value.
///
/// Maps to [`nokhwa`]'s [`ControlValueSetter`].
pub trait ControlValue {
	type ConType: ValidControlValue;

	fn get(&self) -> Self::ConType;
	fn get_mut(&mut self) -> &mut Self::ConType;
}

/// A type that wraps a [`ControlValue`] with other control fields.
pub trait ControlDescription {
	type ValueType;
	type ConType: ValidControlValue;

	fn control_id(&self) -> KnownCameraControl;
	/// Try to set the value for this control description.
	///
	/// # Errors
	///
	/// If the control is invalid, given the controls other requirements.
	fn try_set_control_value(
		&mut self,
		value: Self::ValueType,
	) -> EdgeCamerasResult<()>;
	fn control_value(&self) -> Self::ValueType;
}

/// A type that supports the `default` field.
///
/// Bound by [`ControlValue`].
pub trait ControlDefault: ControlDescription {
	// type DefaultType = Self::ValueType;
	type DefaultType: ValidDefaultValue;

	fn control_default(&self) -> Self::DefaultType;

	#[expect(
		clippy::expect_used,
		reason = "Default value should never be invalid"
	)]
	// TODO: Mark this as fallible.
	fn reset_to_default(&mut self)
	where
		<Self as ControlDescription>::ValueType:
			From<<Self as ControlDefault>::DefaultType>,
	{
		self.try_set_control_value(self.control_default().into())
			.expect("default value should never be invalid");
	}
}

/// A type that supports the `min` field.
///
/// Equal to [`ControlValue`].
pub trait ControlMin: ControlDescription {
	// type MinType = Self::ValueType;
	type MinType: ValidControlValue;

	fn control_min(&self) -> Self::MinType;

	fn validate(
		&self,
		value: Self::MinType,
	) -> bool {
		value >= self.control_min()
	}
}

/// A type that supports the `max` field.
///
/// Equal to [`ControlValue`].
pub trait ControlMax: ControlDescription {
	// type MaxType = Self::ValueType;
	type MaxType: ValidControlValue;

	fn control_max(&self) -> Self::MaxType;

	fn validate(
		&self,
		value: Self::MaxType,
	) -> bool {
		value <= self.control_max()
	}
}

/// A type that supports the `step` field.
///
/// Equal to [`ControlValue`].
pub trait ControlStep: ControlDescription {
	// type StepType = Self::ValueType;
	type StepType: ValidControlValue;

	fn control_step(&self) -> Self::StepType;
	// TODO: Default implementation.
	fn validate(
		&self,
		value: Self::StepType,
	) -> bool;
}

/// A type that supports the `possible` field.
///
/// Bound by [`ControlValue`].
pub trait ControlPossible: ControlDescription {
	// type PossibleType = Self::ValueType;
	type PossibleType: ValidPossibleValue;

	fn control_possible(&self) -> Self::PossibleType;
	// TODO: Default implementation.
	fn validate(
		&self,
		value: &Self::ConType,
	) -> bool;
}

/// A type that supports the `items` field.
pub trait ControlItems: ControlDescription {
	type ItemType: ValidItemValue;

	fn control_items(&self) -> Self::ItemType;
	fn validate(
		&self,
		value: Self::ConType,
	) -> bool;
}
