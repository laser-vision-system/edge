//! Core traits used to build camera control descriptions.

use std::fmt::Debug;

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
///
/// #### [`Hash`], [`Eq`], [`Ord`]
/// - [`f64`]
///
/// [`Hash`]: std::hash::Hash
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

/// A marker trait that determines what is acceptable to use within a
/// [`ControlDefault`].
pub trait ValidDefaultValue: Clone + Debug + PartialEq + PartialOrd {}

impl<T> ValidDefaultValue for T where T: ValidControlValue
{
}

impl ValidDefaultValue for Option<ConTypeString> {
}

/// A marker trait that determines what is acceptable to use within a
/// [`ControlPossible`].
pub trait ValidPossibleValue: Clone + Debug + PartialEq + PartialOrd {}

impl<T> ValidPossibleValue for T where T: ValidControlValue
{
}

impl ValidPossibleValue for Vec<ConTypeEnum> {
}

/// A marker trait that determines what is acceptable to use within a
/// [`ControlItems`].
pub trait ValidItemValue: Clone + Debug + PartialEq + PartialOrd {}

impl ValidItemValue for Vec<MenuEntry> {
}

/// The type that is used to report the control's current value as well as apply
/// a new value.
///
/// Maps to [`nokhwa`]'s [`ControlValueSetter`].
///
/// [`ControlValueSetter`]: nokhwa::utils::ControlValueSetter
pub trait ControlValue {
	/// The type that this control uses.
	type ConType: ValidControlValue;

	/// Get the current control value.
	fn get(&self) -> Self::ConType;
	/// Get the current control value as mutable.
	fn get_mut(&mut self) -> &mut Self::ConType;
}

/// A type that represents a camera's control in its entirely.
///
/// This wraps a [`ControlValue`] with other control fields.
pub trait ControlDescription {
	/// The underlying control type of this control.
	type ValueType;
	/// The type that this control uses.
	type ConType: ValidControlValue;

	/// Get the camera control identifier for this control.
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
	/// Get the current value of this control.
	fn control_value(&self) -> Self::ValueType;
}

/// A description that supports the `default` field.
///
/// Bound by [`ControlValue`].
pub trait ControlDefault: ControlDescription {
	/// The value type of the `default` value.
	///
	/// In most cases, this should be the same as
	/// [`ControlDescription::ValueType`].
	type DefaultType: ValidDefaultValue;
	// type DefaultType = Self::ValueType;

	/// Get the `default` value for this control description.
	fn control_default(&self) -> Self::DefaultType;

	/// Change the [`ControlDescription`] back to the default value.
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

/// A description that supports the `min` field.
///
/// Equal to [`ControlValue`].
pub trait ControlMin: ControlDescription {
	/// The value type of the `min` value.
	///
	/// In most cases, this should be the same as
	/// [`ControlDescription::ValueType`].
	type MinType: ValidControlValue;
	// type MinType = Self::ValueType;

	/// Get the `min` value for this control description.
	fn control_min(&self) -> Self::MinType;

	/// Validate a control value against this type.
	fn validate(
		&self,
		value: Self::MinType,
	) -> bool {
		value >= self.control_min()
	}
}

/// A description that supports the `max` field.
///
/// Equal to [`ControlValue`].
pub trait ControlMax: ControlDescription {
	/// The value type of the `max` value.
	///
	/// In most cases, this should be the same as
	/// [`ControlDescription::ValueType`].
	type MaxType: ValidControlValue;
	// type MaxType = Self::ValueType;

	/// Get the `max` value for this control description.
	fn control_max(&self) -> Self::MaxType;

	/// Validate a control value against this type.
	fn validate(
		&self,
		value: Self::MaxType,
	) -> bool {
		value <= self.control_max()
	}
}

/// A description that supports the `step` field.
///
/// Equal to [`ControlValue`].
pub trait ControlStep: ControlDescription {
	/// The value type of the `step` value.
	///
	/// In most cases, this should be the same as
	/// [`ControlDescription::ValueType`].
	type StepType: ValidControlValue;
	// type StepType = Self::ValueType;

	/// Get the `step` value for this control description.
	fn control_step(&self) -> Self::StepType;

	/// Validate a control value against this type.
	// TODO: Default implementation.
	fn validate(
		&self,
		value: Self::StepType,
	) -> bool;
}

/// A description that supports the `possible` field.
///
/// Bound by [`ControlValue`].
pub trait ControlPossible: ControlDescription {
	/// The value type of the `possible` values.
	///
	/// In most cases, this should be the same as
	/// [`ControlDescription::ValueType`].
	type PossibleType: ValidPossibleValue;
	// type PossibleType = Self::ValueType;

	/// Get the `possible` values for this control description.
	fn control_possible(&self) -> Self::PossibleType;

	/// Validate a control value against this type.
	// TODO: Default implementation.
	fn validate(
		&self,
		value: &Self::ConType,
	) -> bool;
}

/// A description that supports the `items` field.
pub trait ControlItems: ControlDescription {
	/// The value type of the `items` values.
	///
	/// In most cases, this should be the same as
	/// [`ControlDescription::ValueType`].
	type ItemType: ValidItemValue;

	/// Get the `items` values for this control description.
	fn control_items(&self) -> Self::ItemType;

	/// Validate a control value against this type.
	// TODO: Default implementation.
	fn validate(
		&self,
		value: Self::ConType,
	) -> bool;
}
