use std::hash::Hash;

use nokhwa::utils::{CameraControl, ControlValueDescription, KnownCameraControl};

use super::{
	traits::{
		ControlDefault, ControlDescription, ControlMax, ControlMin, ControlPossible, ControlStep,
		ControlValue,
	},
	types::{
		ConTypeBool, ConTypeBytes, ConTypeEnum, ConTypeFloat, ConTypeInt, ConTypeKeyValue,
		ConTypePoint, ConTypeRGB, ConTypeString,
	},
	values::{
		ConValueBoolean, ConValueBytes, ConValueEnum, ConValueFloat, ConValueInteger,
		ConValueKeyValue, ConValuePoint, ConValueRGB, ConValueString,
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
				ConValueInteger::from(*value),
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
				ConValueInteger::from(*value),
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
				ConValueFloat::from(*value),
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
				ConValueFloat::from(*value),
				*default,
				*step,
				*min,
				*max,
			)),
			ControlValueDescription::Boolean { value, default } => CamControl::Boolean(
				BooleanControl::new(control_type, ConValueBoolean::from(*value), *default),
			),
			ControlValueDescription::String { value, default } => {
				CamControl::String(StringControl::new(
					control_type,
					ConValueString::from(value.clone()),
					default.clone(),
				))
			},
			ControlValueDescription::Bytes { value, default } => {
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
			} => CamControl::KeyValuePair(KeyValuePairControl::new(
				control_type,
				ConValueKeyValue::from(ConTypeKeyValue::from((*key, *value))),
				(*default).into(),
			)),
			ControlValueDescription::Point { value, default } => {
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
			} => CamControl::Enum(EnumControl::new(
				control_type,
				ConValueEnum::from(*value),
				*default,
				possible.clone(),
			)),
			ControlValueDescription::RGB {
				value,
				max,
				default,
			} => CamControl::RGB(RGBControl::new(
				control_type,
				ConValueRGB::from(ConTypeRGB::from(*value)),
				ConTypeRGB::from(*default),
				ConTypeRGB::from(*max),
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
	value: ConValueInteger,
	default: <ConValueInteger as ControlValue>::ConType,
	step: <ConValueInteger as ControlValue>::ConType,
}

impl IntegerControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueInteger,
		default: <ConValueInteger as ControlValue>::ConType,
		step: <ConValueInteger as ControlValue>::ConType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			step,
		}
	}
}

impl ControlDescription for IntegerControl {
	type ValueType = ConValueInteger;
	type ConType = ConTypeInt;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(&mut self, value: Self::ValueType) -> Result<(), ()> {
		if self.valid(value.get_value()) {
			self.value = value;
			Ok(())
		} else {
			Err(())
		}
	}

	fn get_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for IntegerControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid")
	}
}

impl ControlStep for IntegerControl {
	// Default should be based on ValueType, so this makes sure they can be compared.
	type StepType = Self::ConType;

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
	value: ConValueInteger,
	default: <ConValueInteger as ControlValue>::ConType,
	step: <ConValueInteger as ControlValue>::ConType,
	min: <ConValueInteger as ControlValue>::ConType,
	max: <ConValueInteger as ControlValue>::ConType,
}

impl IntegerRangeControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueInteger,
		default: <ConValueInteger as ControlValue>::ConType,
		step: <ConValueInteger as ControlValue>::ConType,
		min: <ConValueInteger as ControlValue>::ConType,
		max: <ConValueInteger as ControlValue>::ConType,
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

impl ControlDescription for IntegerRangeControl {
	type ValueType = ConValueInteger;
	type ConType = ConTypeInt;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(&mut self, value: Self::ValueType) -> Result<(), ()> {
		if ControlMin::valid(self, value.get_value())
			&& ControlMax::valid(self, value.get_value())
			&& ControlStep::valid(self, value.get_value())
		{
			self.value = value;
			Ok(())
		} else {
			Err(())
		}
	}

	fn get_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for IntegerRangeControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid")
	}
}

impl ControlStep for IntegerRangeControl {
	// Default should be based on ValueType, so this makes sure they can be compared.
	type StepType = Self::ConType;

	fn get_step(&self) -> Self::StepType {
		self.step
	}

	fn valid(&self, value: Self::StepType) -> bool {
		(value - self.get_default()).abs() % self.get_step() == 0
			|| (value - self.get_value().get_value()).abs() % self.get_step() == 0
	}
}

impl ControlMin for IntegerRangeControl {
	type MinType = Self::ConType;

	fn get_min(&self) -> Self::MinType {
		self.min
	}
}

impl ControlMax for IntegerRangeControl {
	type MaxType = Self::ConType;

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
	value: ConValueFloat,
	default: <ConValueFloat as ControlValue>::ConType,
	step: <ConValueFloat as ControlValue>::ConType,
}

impl FloatControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueFloat,
		default: <ConValueFloat as ControlValue>::ConType,
		step: <ConValueFloat as ControlValue>::ConType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			step,
		}
	}
}

impl ControlDescription for FloatControl {
	type ValueType = ConValueFloat;
	type ConType = ConTypeFloat;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(&mut self, value: Self::ValueType) -> Result<(), ()> {
		if self.valid(value.get_value()) {
			self.value = value;
			Ok(())
		} else {
			Err(())
		}
	}

	fn get_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for FloatControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid")
	}
}

impl ControlStep for FloatControl {
	// Default should be based on ValueType, so this makes sure they can be compared.
	type StepType = Self::ConType;

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
	value: ConValueFloat,
	default: <ConValueFloat as ControlValue>::ConType,
	step: <ConValueFloat as ControlValue>::ConType,
	min: <ConValueFloat as ControlValue>::ConType,
	max: <ConValueFloat as ControlValue>::ConType,
}

impl FloatRangeControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueFloat,
		default: <ConValueFloat as ControlValue>::ConType,
		step: <ConValueFloat as ControlValue>::ConType,
		min: <ConValueFloat as ControlValue>::ConType,
		max: <ConValueFloat as ControlValue>::ConType,
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

impl ControlDescription for FloatRangeControl {
	type ValueType = ConValueFloat;
	type ConType = ConTypeFloat;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(&mut self, value: Self::ValueType) -> Result<(), ()> {
		if ControlMin::valid(self, value.get_value())
			&& ControlMax::valid(self, value.get_value())
			&& ControlStep::valid(self, value.get_value())
		{
			self.value = value;
			Ok(())
		} else {
			Err(())
		}
	}

	fn get_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for FloatRangeControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid")
	}
}

impl ControlStep for FloatRangeControl {
	// Default should be based on ValueType, so this makes sure they can be compared.
	type StepType = Self::ConType;

	fn get_step(&self) -> Self::StepType {
		self.step
	}

	fn valid(&self, value: Self::StepType) -> bool {
		(value - self.get_default()).abs() % self.get_step() == 0.0
			|| (value - self.get_value().get_value()).abs() % self.get_step() == 0.0
	}
}

impl ControlMin for FloatRangeControl {
	type MinType = Self::ConType;

	fn get_min(&self) -> Self::MinType {
		self.min
	}
}

impl ControlMax for FloatRangeControl {
	type MaxType = Self::ConType;

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
	value: ConValueBoolean,
	default: <ConValueBoolean as ControlValue>::ConType,
}

impl BooleanControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueBoolean,
		default: <ConValueBoolean as ControlValue>::ConType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlDescription for BooleanControl {
	type ValueType = ConValueBoolean;
	type ConType = ConTypeBool;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(&mut self, value: Self::ValueType) -> Result<(), ()> {
		self.value = value;
		Ok(())
	}

	fn get_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for BooleanControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid")
	}
}

/// A camera control that consists of a value and an optional default.
///
/// Maps to [`ControlValueDescription::String`]

#[derive(Clone, Debug)]
pub struct StringControl {
	control_type: KnownCameraControl,
	value: ConValueString,
	default: Option<<ConValueString as ControlValue>::ConType>,
}

impl StringControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueString,
		default: Option<<ConValueString as ControlValue>::ConType>,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlDescription for StringControl {
	type ValueType = ConValueString;
	type ConType = ConTypeString;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(&mut self, value: Self::ValueType) -> Result<(), ()> {
		self.value = value;
		Ok(())
	}

	fn get_value(&self) -> Self::ValueType {
		self.value.clone()
	}
}

impl ControlDefault for StringControl {
	type DefaultType = Option<Self::ConType>;

	fn get_default(&self) -> Self::DefaultType {
		self.default.clone()
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().unwrap_or("".to_string()).into())
			.expect("default value should never be invalid")
	}
}

/// A camera control that consists of a value and default.
///
/// Maps to [`ControlValueDescription::Bytes`]

#[derive(Clone, Debug)]
pub struct BytesControl {
	control_type: KnownCameraControl,
	value: ConValueBytes,
	default: <ConValueBytes as ControlValue>::ConType,
}

impl BytesControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueBytes,
		default: <ConValueBytes as ControlValue>::ConType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlDescription for BytesControl {
	type ValueType = ConValueBytes;
	type ConType = ConTypeBytes;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(&mut self, value: Self::ValueType) -> Result<(), ()> {
		self.value = value;
		Ok(())
	}

	fn get_value(&self) -> Self::ValueType {
		self.value.clone()
	}
}

impl ControlDefault for BytesControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default.clone()
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid")
	}
}

/// A camera control that consists of a value and default.
///
/// Maps to [`ControlValueDescription::KeyValuePair`]

#[derive(Clone, Copy, Debug)]
pub struct KeyValuePairControl {
	control_type: KnownCameraControl,
	value: ConValueKeyValue,
	default: <ConValueKeyValue as ControlValue>::ConType,
}

impl KeyValuePairControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueKeyValue,
		default: <ConValueKeyValue as ControlValue>::ConType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlDescription for KeyValuePairControl {
	type ValueType = ConValueKeyValue;
	type ConType = ConTypeKeyValue;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(&mut self, value: Self::ValueType) -> Result<(), ()> {
		self.value = value;
		Ok(())
	}

	fn get_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for KeyValuePairControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid")
	}
}

/// A camera control that consists of a value and default.
///
/// Maps to [`ControlValueDescription::Point`]

#[derive(Clone, Copy, Debug)]
pub struct PointControl {
	control_type: KnownCameraControl,
	value: ConValuePoint,
	default: <ConValuePoint as ControlValue>::ConType,
}

impl PointControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValuePoint,
		default: <ConValuePoint as ControlValue>::ConType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
		}
	}
}

impl ControlDescription for PointControl {
	type ValueType = ConValuePoint;
	type ConType = ConTypePoint;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(&mut self, value: Self::ValueType) -> Result<(), ()> {
		self.value = value;
		Ok(())
	}

	fn get_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for PointControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid")
	}
}

/// A camera control that consists of a value, default, and a vector of possible options.
///
/// Maps to [`ControlValueDescription::Enum`]

#[derive(Clone, Debug)]
pub struct EnumControl {
	control_type: KnownCameraControl,
	value: ConValueEnum,
	default: <ConValueEnum as ControlValue>::ConType,
	possible: Vec<<ConValueEnum as ControlValue>::ConType>,
}

impl EnumControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueEnum,
		default: <ConValueEnum as ControlValue>::ConType,
		possible: Vec<<ConValueEnum as ControlValue>::ConType>,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			possible,
		}
	}
}

impl ControlDescription for EnumControl {
	type ValueType = ConValueEnum;
	type ConType = ConTypeEnum;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(&mut self, value: Self::ValueType) -> Result<(), ()> {
		self.value = value;
		Ok(())
	}

	fn get_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for EnumControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid")
	}
}

impl ControlPossible for EnumControl {
	type PossibleType = Vec<Self::ConType>;

	fn get_possible(&self) -> Self::PossibleType {
		self.possible.clone()
	}

	fn valid(&self, value: &Self::ConType) -> bool {
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
	value: ConValueRGB,
	default: <ConValueRGB as ControlValue>::ConType,
	max: <ConValueRGB as ControlValue>::ConType,
}

impl RGBControl {
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueRGB,
		default: <ConValueRGB as ControlValue>::ConType,
		max: <ConValueRGB as ControlValue>::ConType,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			max,
		}
	}
}

impl ControlDescription for RGBControl {
	type ValueType = ConValueRGB;
	type ConType = ConTypeRGB;

	fn get_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn set_value(&mut self, value: Self::ValueType) -> Result<(), ()> {
		if self.valid(value.get_value()) {
			self.value = value;
			Ok(())
		} else {
			Err(())
		}
	}

	fn get_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for RGBControl {
	type DefaultType = Self::ConType;

	fn get_default(&self) -> Self::DefaultType {
		self.default
	}

	fn reset_to_default(&mut self) {
		self.set_value(self.get_default().into())
			.expect("default value should never be invalid")
	}
}

impl ControlMax for RGBControl {
	type MaxType = Self::ConType;

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
