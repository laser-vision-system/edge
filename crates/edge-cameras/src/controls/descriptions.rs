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
	ControlValueSetter,
	KnownCameraControl,
	KnownCameraControlFlag,
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
		ControlFlags as _,
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
use crate::error::{
	EdgeCamerasError,
	EdgeCamerasResult,
};

// ===== Field relationship bounds =====
// (step, min, max, key): value
// default: value
// default: Option<value>
// possible: Vec<value>
// !!!default: (value, value)!!! Not anymore, see [`KeyValue`].

// Every variant has a value and default.
// There are no hard correllations for the other fields.

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
	/// Create a [`CamControl`] from a [`ControlValueDescription`],
	/// [`KnownCameraControl`], and [`KnownCameraControlFlag`]s.
	#[expect(
		clippy::too_many_lines,
		reason = "Necessary to map from one type to the other"
	)]
	pub fn from_description_id_and_flags(
		description: ControlValueDescription,
		id: KnownCameraControl,
		flags: Vec<KnownCameraControlFlag>,
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
					flags,
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
					flags,
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
					flags,
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
					flags,
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
					flags,
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
					flags,
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
					flags,
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
					flags,
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
					flags,
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
					flags,
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
					flags,
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
					flags,
				))
			},
		}
	}

	/// Create a [`CamControl`] from a [`ControlValueDescription`] and
	/// [`KnownCameraControl`] with no flags.
	pub fn from_description_and_id(
		description: ControlValueDescription,
		id: KnownCameraControl,
	) -> Self {
		Self::from_description_id_and_flags(description, id, Vec::new())
	}

	/// Get the camera control identifier for this control.
	///
	/// # Errors
	///
	/// Returns an error if this control has no description.
	pub fn control_id(&self) -> EdgeCamerasResult<KnownCameraControl> {
		match self {
			CamControl::Integer(control) => Ok(control.control_id()),
			CamControl::IntegerRange(control) => Ok(control.control_id()),
			CamControl::Float(control) => Ok(control.control_id()),
			CamControl::FloatRange(control) => Ok(control.control_id()),
			CamControl::Boolean(control) => Ok(control.control_id()),
			CamControl::String(control) => Ok(control.control_id()),
			CamControl::Bytes(control) => Ok(control.control_id()),
			CamControl::KeyValuePair(control) => Ok(control.control_id()),
			CamControl::Point(control) => Ok(control.control_id()),
			CamControl::Enum(control) => Ok(control.control_id()),
			CamControl::RGB(control) => Ok(control.control_id()),
			CamControl::Menu(control) => Ok(control.control_id()),
			CamControl::None => Err(EdgeCamerasError::NoControlDescription),
		}
	}

	/// Convert the control description to its corresponding Nokhwa setter.
	///
	/// # Errors
	///
	/// Returns an error when the control has no description.
	pub fn control_value_setter(
		&self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult<ControlValueSetter> {
		match &self {
			CamControl::Integer(control) => Ok(control.control_value().into()),
			CamControl::IntegerRange(control) => {
				Ok(control.control_value().into())
			},
			CamControl::Float(control) => Ok(control.control_value().into()),
			CamControl::FloatRange(control) => {
				Ok(control.control_value().into())
			},
			CamControl::Boolean(control) => Ok(control.control_value().into()),
			CamControl::String(control) => Ok(control.control_value().into()),
			CamControl::Bytes(control) => Ok(control.control_value().into()),
			CamControl::KeyValuePair(control) => {
				Ok(control.control_value().into())
			},
			CamControl::Point(control) => Ok(control.control_value().into()),
			CamControl::Enum(control) => Ok(control.control_value().into()),
			CamControl::RGB(control) => Ok(control.control_value().into()),
			CamControl::Menu(control) => Ok(control.control_value().into()),
			CamControl::None => Err(EdgeCamerasError::GetControlType(id)),
		}
	}

	/// Reset the control description to its default value.
	///
	/// # Errors
	///
	/// If the control has no default value, the default value is invalid for
	/// this control, or this control has no description.
	pub fn reset_to_default(&mut self) -> EdgeCamerasResult {
		match self {
			CamControl::Integer(control) => control.reset_to_default(),
			CamControl::IntegerRange(control) => control.reset_to_default(),
			CamControl::Float(control) => control.reset_to_default(),
			CamControl::FloatRange(control) => control.reset_to_default(),
			CamControl::Boolean(control) => control.reset_to_default(),
			CamControl::String(control) => control.reset_to_default(),
			CamControl::Bytes(control) => control.reset_to_default(),
			CamControl::KeyValuePair(control) => control.reset_to_default(),
			CamControl::Point(control) => control.reset_to_default(),
			CamControl::Enum(control) => control.reset_to_default(),
			CamControl::RGB(control) => control.reset_to_default(),
			CamControl::Menu(control) => control.reset_to_default(),
			CamControl::None => Err(EdgeCamerasError::NoControlDescription),
		}
	}

	/// Access the control flags for this description, if available.
	pub fn control_flags(&self) -> &[KnownCameraControlFlag] {
		match self {
			Self::None => &[],
			Self::Integer(control) => control.control_flags(),
			Self::IntegerRange(control) => control.control_flags(),
			Self::Float(control) => control.control_flags(),
			Self::FloatRange(control) => control.control_flags(),
			Self::Boolean(control) => control.control_flags(),
			Self::String(control) => control.control_flags(),
			Self::Bytes(control) => control.control_flags(),
			Self::KeyValuePair(control) => control.control_flags(),
			Self::Point(control) => control.control_flags(),
			Self::Enum(control) => control.control_flags(),
			Self::RGB(control) => control.control_flags(),
			Self::Menu(control) => control.control_flags(),
		}
	}

	/// Check whether the control has a specific flag.
	pub fn has_flag(
		&self,
		flag: KnownCameraControlFlag,
	) -> bool {
		self.control_flags().contains(&flag)
	}

	/// Whether the control is currently in automatic mode.
	pub fn is_auto(&self) -> bool {
		self.has_flag(KnownCameraControlFlag::Automatic)
	}

	/// Whether the control is currently in manual mode.
	pub fn is_manual(&self) -> bool {
		self.has_flag(KnownCameraControlFlag::Manual)
	}

	/// Whether the control is continuously updating or adjusting.
	pub fn is_continuous(&self) -> bool {
		self.has_flag(KnownCameraControlFlag::Continuous)
	}

	/// Whether the control is disabled or inactive.
	pub fn is_disabled(&self) -> bool {
		match self {
			Self::None => true,
			_ => self.has_flag(KnownCameraControlFlag::Disabled),
		}
	}

	/// Whether this control is readable by its mapped value.
	pub fn is_readable(&self) -> bool {
		match self {
			Self::None => false,
			Self::Integer(control) => control.is_readable(),
			Self::IntegerRange(control) => control.is_readable(),
			Self::Float(control) => control.is_readable(),
			Self::FloatRange(control) => control.is_readable(),
			Self::Boolean(control) => control.is_readable(),
			Self::String(control) => control.is_readable(),
			Self::Bytes(control) => control.is_readable(),
			Self::KeyValuePair(control) => control.is_readable(),
			Self::Point(control) => control.is_readable(),
			Self::Enum(control) => control.is_readable(),
			Self::RGB(control) => control.is_readable(),
			Self::Menu(control) => control.is_readable(),
		}
	}

	/// Whether this control may safely be written.
	pub fn is_writable(&self) -> bool {
		match self {
			Self::None => false,
			Self::Integer(control) => control.is_writable(),
			Self::IntegerRange(control) => control.is_writable(),
			Self::Float(control) => control.is_writable(),
			Self::FloatRange(control) => control.is_writable(),
			Self::Boolean(control) => control.is_writable(),
			Self::String(control) => control.is_writable(),
			Self::Bytes(control) => control.is_writable(),
			Self::KeyValuePair(control) => control.is_writable(),
			Self::Point(control) => control.is_writable(),
			Self::Enum(control) => control.is_writable(),
			Self::RGB(control) => control.is_writable(),
			Self::Menu(control) => control.is_writable(),
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

	/// Convert this control's flags to [`String`], if any.
	pub fn try_display_control_flags(&self) -> Option<String> {
		let flags = self.control_flags();
		if flags.is_empty() {
			None
		} else {
			let formatted = flags
				.iter()
				.map(ToString::to_string)
				.collect::<Vec<_>>()
				.join(", ");
			Some(formatted)
		}
	}
}

impl From<CameraControl> for CamControl {
	/// Convert a single [`nokhwa`] [`CameraControl`] into a custom type.
	fn from(value: CameraControl) -> Self {
		let control_type = value.control();
		let flags = value.flag().to_vec();
		// IDEA: Maybe add this?
		// if !value.active() &&
		// !flags.contains(&KnownCameraControlFlag::Disabled)
		//{
		// 	flags.push(KnownCameraControlFlag::Disabled);
		//}
		// TODO: Remove description clone.
		Self::from_description_id_and_flags(
			value.description().clone(),
			control_type,
			flags,
		)
	}
}

impl From<IntegerControl> for CamControl {
	fn from(value: IntegerControl) -> Self {
		CamControl::Integer(value)
	}
}

impl From<IntegerRangeControl> for CamControl {
	fn from(value: IntegerRangeControl) -> Self {
		CamControl::IntegerRange(value)
	}
}

impl From<FloatControl> for CamControl {
	fn from(value: FloatControl) -> Self {
		Self::Float(value)
	}
}

impl From<FloatRangeControl> for CamControl {
	fn from(value: FloatRangeControl) -> Self {
		Self::FloatRange(value)
	}
}

impl From<BooleanControl> for CamControl {
	fn from(value: BooleanControl) -> Self {
		Self::Boolean(value)
	}
}

impl From<StringControl> for CamControl {
	fn from(value: StringControl) -> Self {
		Self::String(value)
	}
}

impl From<BytesControl> for CamControl {
	fn from(value: BytesControl) -> Self {
		Self::Bytes(value)
	}
}

impl From<KeyValuePairControl> for CamControl {
	fn from(value: KeyValuePairControl) -> Self {
		Self::KeyValuePair(value)
	}
}

impl From<PointControl> for CamControl {
	fn from(value: PointControl) -> Self {
		Self::Point(value)
	}
}

impl From<EnumControl> for CamControl {
	fn from(value: EnumControl) -> Self {
		Self::Enum(value)
	}
}

impl From<RGBControl> for CamControl {
	fn from(value: RGBControl) -> Self {
		Self::RGB(value)
	}
}

impl From<MenuControl> for CamControl {
	fn from(value: MenuControl) -> Self {
		Self::Menu(value)
	}
}

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

	// Use `cargo test -p edge-cameras controls::descriptions --
	// --include-ignored --no-capture --test-threads 1` to check all controls

	pub fn display_with_different_formats(
		label: &'static str,
		control: &(impl ControlDescription + fmt::Display + fmt::Debug),
	) {
		println!(
			"{label} (dbg) | {control:?}\n{label} (std) | {control}\n{label} (alt) | {control:#}"
		);
	}
}
