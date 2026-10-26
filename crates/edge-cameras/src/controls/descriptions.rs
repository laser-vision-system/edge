//! The combination of information to represent a single camera control.

mod boolean_control;
mod bytes_control;
mod enum_control;
mod float_control;
mod float_range_control;
mod integer_control;
mod integer_range_control;
mod key_value_pair_control;
mod menu_control;
mod point_control;
mod rgb_control;
mod string_control;

use std::hash::Hash;

use nokhwa::utils::{
	CameraControl,
	ControlValueDescription,
	KnownCameraControl,
};
use serde::{
	Deserialize,
	Serialize,
};

pub use self::{
	boolean_control::BooleanControl,
	bytes_control::BytesControl,
	enum_control::EnumControl,
	float_control::FloatControl,
	float_range_control::FloatRangeControl,
	integer_control::IntegerControl,
	integer_range_control::IntegerRangeControl,
	key_value_pair_control::KeyValuePairControl,
	menu_control::MenuControl,
	point_control::PointControl,
	rgb_control::RGBControl,
	string_control::StringControl,
};
use super::{
	traits::{
		ControlDefault as _,
		ControlDescription as _,
		ControlItems as _,
		ControlMax as _,
		ControlMin as _,
		ControlPossible as _,
		ControlStep as _,
	},
	types::{
		ConTypeBytes,
		ConTypeMenu,
		ConTypePoint,
		ConTypeRGB,
		ConTypeString,
	},
};

// ===== Field relationship bounds =====
// (step, min, max, key): value
// default: value
// default: Option<value>
// possible: Vec<value>
// !!!default: (value, value)!!! Not anymore, see [`KeyValue`].

/// Represents one type of camera control.
///
/// Maps to [`ControlValueDescription`].
// TODO: The inner types could be composed.
// Control Value should be read-only and internally managed.
#[derive(
	Clone,
	Debug,
	Default,
	Hash,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Serialize,
	Deserialize,
)]
pub enum CamControl {
	/// For when a camera reports a control but none is provided.
	#[default]
	None, // None(NoneControl),
	/// Represents [`IntegerControl`].
	Integer(IntegerControl),
	/// Represents [`IntegerRangeControl`].
	IntegerRange(IntegerRangeControl),
	/// Represents [`FloatControl`].
	Float(FloatControl),
	/// Represents [`FloatRangeControl`].
	FloatRange(FloatRangeControl),
	/// Represents [`BooleanControl`].
	Boolean(BooleanControl),
	/// Represents [`StringControl`].
	String(StringControl),
	/// Represents [`BytesControl`].
	Bytes(BytesControl),
	/// Represents [`KeyValuePairControl`].
	KeyValuePair(KeyValuePairControl),
	/// Represents [`PointControl`].
	Point(PointControl),
	/// Represents [`EnumControl`].
	Enum(EnumControl),
	/// Represents [`RGBControl`].
	RGB(RGBControl),
	/// Represents [`MenuControl`].
	Menu(MenuControl),
}

impl CamControl {
	/// Create a [`CamControl`] from a [`ControlValueDescription`] and
	/// [`KnownCameraControl`].
	#[expect(
		clippy::too_many_lines,
		reason = "Necessary to map from one type to the other"
	)]
	pub fn from_description_and_id(
		description: ControlValueDescription,
		id: KnownCameraControl,
	) -> Self {
		match description {
			ControlValueDescription::None => CamControl::None,
			ControlValueDescription::Integer {
				value,
				default,
				step,
			} => {
				CamControl::Integer(IntegerControl::new(
					id,
					value.into(),
					default,
					step,
				))
			},
			ControlValueDescription::IntegerRange {
				min,
				max,
				value,
				step,
				default,
			} => {
				CamControl::IntegerRange(IntegerRangeControl::new(
					id,
					value.into(),
					default,
					step,
					min,
					max,
				))
			},
			ControlValueDescription::Float {
				value,
				default,
				step,
			} => {
				CamControl::Float(FloatControl::new(
					id,
					value.into(),
					default,
					step,
				))
			},
			ControlValueDescription::FloatRange {
				min,
				max,
				value,
				step,
				default,
			} => {
				CamControl::FloatRange(FloatRangeControl::new(
					id,
					value.into(),
					default,
					step,
					min,
					max,
				))
			},
			ControlValueDescription::Boolean {
				value,
				default,
			} => {
				CamControl::Boolean(BooleanControl::new(
					id,
					value.into(),
					default,
				))
			},
			ControlValueDescription::String {
				value,
				default,
			} => {
				CamControl::String(StringControl::new(
					id,
					value.into(),
					default.map(ConTypeString::from),
				))
			},
			ControlValueDescription::Bytes {
				value,
				default,
			} => {
				CamControl::Bytes(BytesControl::new(
					id,
					value.into(),
					ConTypeBytes::from(default),
				))
			},
			ControlValueDescription::KeyValuePair {
				key,
				value,
				default,
			} => {
				CamControl::KeyValuePair(KeyValuePairControl::new(
					id,
					(key, value).into(),
					(default).into(),
				))
			},
			ControlValueDescription::Point {
				value,
				default,
			} => {
				CamControl::Point(PointControl::new(
					id,
					value.into(),
					ConTypePoint::from(default),
				))
			},
			ControlValueDescription::Enum {
				value,
				possible,
				default,
			} => {
				CamControl::Enum(EnumControl::new(
					id,
					value.into(),
					default,
					possible.into(),
				))
			},
			ControlValueDescription::RGB {
				value,
				max,
				default,
			} => {
				CamControl::RGB(RGBControl::new(
					id,
					value.into(),
					ConTypeRGB::from(default),
					ConTypeRGB::from(max),
				))
			},
			ControlValueDescription::Menu {
				value,
				items,
				default,
			} => {
				CamControl::Menu(MenuControl::new(
					id,
					value.into(),
					ConTypeMenu::from(default),
					items.into(),
				))
			},
		}
	}

	/// Convert this control's value to [`String`], if it exists.
	pub fn try_display_control_value(&self) -> Option<String> {
		match self {
			Self::None => None,
			Self::Integer(control) => Some(control.control_value().to_string()),
			Self::IntegerRange(control) => {
				Some(control.control_value().to_string())
			},
			Self::Float(control) => Some(control.control_value().to_string()),
			Self::FloatRange(control) => {
				Some(control.control_value().to_string())
			},
			Self::Boolean(control) => Some(control.control_value().to_string()),
			Self::String(control) => Some(control.control_value().to_string()),
			Self::Bytes(control) => Some(control.control_value().to_string()),
			Self::KeyValuePair(control) => {
				Some(control.control_value().to_string())
			},
			Self::Point(control) => Some(control.control_value().to_string()),
			Self::Enum(control) => Some(control.control_value().to_string()),
			Self::RGB(control) => Some(control.control_value().to_string()),
			Self::Menu(control) => Some(control.control_value().to_string()),
		}
	}

	/// Convert this control's default to [`String`], if it exists.
	pub fn try_display_control_default(&self) -> Option<String> {
		match self {
			Self::None => None,
			Self::Integer(control) => {
				Some(control.control_default().to_string())
			},
			Self::IntegerRange(control) => {
				Some(control.control_default().to_string())
			},
			Self::Float(control) => Some(control.control_default().to_string()),
			Self::FloatRange(control) => {
				Some(control.control_default().to_string())
			},
			Self::Boolean(control) => {
				Some(control.control_default().to_string())
			},
			Self::String(control) => {
				Some(control.control_default()?.to_string())
			},
			Self::Bytes(control) => Some(control.control_default().to_string()),
			Self::KeyValuePair(control) => {
				Some(control.control_default().to_string())
			},
			Self::Point(control) => Some(control.control_default().to_string()),
			Self::Enum(control) => Some(control.control_default().to_string()),
			Self::RGB(control) => Some(control.control_default().to_string()),
			Self::Menu(control) => Some(control.control_default().to_string()),
		}
	}

	/// Convert this control's min to [`String`], if it exists.
	pub fn try_display_control_min(&self) -> Option<String> {
		match self {
			Self::IntegerRange(control) => {
				Some(control.control_min().to_string())
			},
			Self::FloatRange(control) => {
				Some(control.control_min().to_string())
			},
			_ => None,
		}
	}

	/// Convert this control's max to [`String`], if it exists.
	pub fn try_display_control_max(&self) -> Option<String> {
		match self {
			Self::IntegerRange(control) => {
				Some(control.control_max().to_string())
			},
			Self::FloatRange(control) => {
				Some(control.control_max().to_string())
			},
			Self::RGB(control) => Some(control.control_max().to_string()),
			_ => None,
		}
	}

	/// Convert this control's step to [`String`], if it exists.
	pub fn try_display_control_step(&self) -> Option<String> {
		match self {
			Self::Integer(control) => Some(control.control_step().to_string()),
			Self::IntegerRange(control) => {
				Some(control.control_step().to_string())
			},
			Self::Float(control) => Some(control.control_step().to_string()),
			Self::FloatRange(control) => {
				Some(control.control_step().to_string())
			},
			_ => None,
		}
	}

	/// Convert this control's possible to [`String`], if it exists.
	pub fn try_display_control_possible(&self) -> Option<String> {
		match self {
			Self::Enum(control) => Some(control.control_possible().to_string()),
			_ => None,
		}
	}

	/// Convert this control's items to [`String`], if it exists.
	pub fn try_display_control_items(&self) -> Option<String> {
		match self {
			Self::Menu(control) => Some(control.control_items().to_string()),
			_ => None,
		}
	}
}

impl From<CameraControl> for CamControl {
	/// Convert a single [`nokhwa`] [`CameraControl`] into a custom type.
	fn from(value: CameraControl) -> Self {
		let control_type = value.control();
		// TODO: Remove description clone.
		Self::from_description_and_id(value.description().clone(), control_type)
	}
}

// Every variant has a value and default.
// There are no hard correllations for the other fields.

/// Automatically implement [`Hash`], [`PartialEq`], [`Eq`], [`PartialOrd`], and
/// [`Ord`] on a struct by its control id.
macro_rules! impl_identifiable {
	($type:ty) => {
		impl Hash for $type {
			fn hash<H>(
				&self,
				state: &mut H,
			) where
				H: std::hash::Hasher,
			{
				self.control_id().hash(state);
			}
		}

		impl PartialEq for $type {
			fn eq(
				&self,
				other: &Self,
			) -> bool {
				self.control_id() == other.control_id()
			}
		}

		impl Eq for $type {
		}

		impl PartialOrd for $type {
			fn partial_cmp(
				&self,
				other: &Self,
			) -> Option<std::cmp::Ordering> {
				Some(self.cmp(other))
			}
		}

		impl Ord for $type {
			fn cmp(
				&self,
				other: &Self,
			) -> std::cmp::Ordering {
				self.control_id().cmp(&other.control_id())
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
impl_identifiable!(MenuControl);

#[cfg(test)]
mod tests {
	use std::fmt;

	use crate::controls::traits::ControlDescription;

	pub fn display_with_different_formats(
		label: &'static str,
		control: impl ControlDescription + fmt::Display + fmt::Debug,
	) {
		println!(
			"{label} (dbg) | {control:?}\n{label} (std) | {control}\n{label} (alt) | {control:#}"
		);
	}
}
