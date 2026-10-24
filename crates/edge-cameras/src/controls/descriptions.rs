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
	traits::ControlDescription as _,
	types::{
		ConTypeKeyValue,
		ConTypeMenu,
		ConTypePoint,
		ConTypeRGB,
	},
	values::{
		ConValueBoolean,
		ConValueBytes,
		ConValueEnum,
		ConValueFloat,
		ConValueInteger,
		ConValueKeyValue,
		ConValueMenu,
		ConValuePoint,
		ConValueRGB,
		ConValueString,
	},
};

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
	#[default]
	None, // None(NoneControl),
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
	Menu(MenuControl),
}

impl From<CameraControl> for CamControl {
	/// Convert a single [`nokhwa`] [`CameraControl`] into a custom type.
	#[expect(
		clippy::too_many_lines,
		reason = "Necessary to map from one type to the other"
	)]
	fn from(value: CameraControl) -> Self {
		let control_type = value.control();
		match value.description() {
			ControlValueDescription::None => CamControl::None,
			ControlValueDescription::Integer {
				value,
				default,
				step,
			} => {
				CamControl::Integer(IntegerControl::new(
					control_type,
					ConValueInteger::from(*value),
					*default,
					*step,
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
					control_type,
					ConValueInteger::from(*value),
					*default,
					*step,
					*min,
					*max,
				))
			},
			ControlValueDescription::Float {
				value,
				default,
				step,
			} => {
				CamControl::Float(FloatControl::new(
					control_type,
					ConValueFloat::from(*value),
					*default,
					*step,
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
					control_type,
					ConValueFloat::from(*value),
					*default,
					*step,
					*min,
					*max,
				))
			},
			ControlValueDescription::Boolean {
				value,
				default,
			} => {
				CamControl::Boolean(BooleanControl::new(
					control_type,
					ConValueBoolean::from(*value),
					*default,
				))
			},
			ControlValueDescription::String {
				value,
				default,
			} => {
				CamControl::String(StringControl::new(
					control_type,
					ConValueString::from(value.clone()),
					default.clone(),
				))
			},
			ControlValueDescription::Bytes {
				value,
				default,
			} => {
				CamControl::Bytes(BytesControl::new(
					control_type,
					ConValueBytes::from(value.clone()),
					default.clone(),
				))
			},
			ControlValueDescription::KeyValuePair {
				key,
				value,
				default,
			} => {
				CamControl::KeyValuePair(KeyValuePairControl::new(
					control_type,
					ConValueKeyValue::from(ConTypeKeyValue::from((
						*key, *value,
					))),
					(*default).into(),
				))
			},
			ControlValueDescription::Point {
				value,
				default,
			} => {
				CamControl::Point(PointControl::new(
					control_type,
					ConValuePoint::from(ConTypePoint::from(*value)),
					ConTypePoint::from(*default),
				))
			},
			ControlValueDescription::Enum {
				value,
				possible,
				default,
			} => {
				CamControl::Enum(EnumControl::new(
					control_type,
					ConValueEnum::from(*value),
					*default,
					possible.clone(),
				))
			},
			ControlValueDescription::RGB {
				value,
				max,
				default,
			} => {
				CamControl::RGB(RGBControl::new(
					control_type,
					ConValueRGB::from(ConTypeRGB::from(*value)),
					ConTypeRGB::from(*default),
					ConTypeRGB::from(*max),
				))
			},
			ControlValueDescription::Menu {
				value,
				items,
				default,
			} => {
				CamControl::Menu(MenuControl::new(
					control_type,
					ConValueMenu::from(*value),
					ConTypeMenu::from(*default),
					items.clone(),
				))
			},
		}
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
