use std::collections::HashMap;

use nokhwa::{
	Camera,
	utils::{CameraControl, ControlValueDescription, KnownCameraControl},
};

use super::{
	BooleanControl, BytesControl, CamControl, ControlValueBoolean, ControlValueBytes,
	ControlValueEnum, ControlValueFloat, ControlValueInteger, ControlValueKeyValue,
	ControlValuePoint, ControlValueRGB, ControlValueString, EnumControl, FloatControl,
	FloatRangeControl, IntegerControl, IntegerRangeControl, KeyValue, KeyValuePairControl,
	PointControl, RGBControl, StringControl,
};

/// Returns a collection of controls.
pub fn extract_controls(camera: Camera) -> HashMap<KnownCameraControl, CamControl> {
	camera
		.camera_controls()
		.unwrap()
		.iter()
		.map(|control| {
			let key = control.control();
			let value = extract_control(control);
			(key, value)
		})
		.collect::<HashMap<KnownCameraControl, CamControl>>()
}

// The above function should iterate through the entries and called the below function.

/// Maps the internal nokhwa control to a custom implementation.
pub fn extract_control(nokhwa_control: &CameraControl) -> CamControl {
	match nokhwa_control.description() {
		ControlValueDescription::None => CamControl::None,
		ControlValueDescription::Integer {
			value,
			default,
			step,
		} => CamControl::Integer(IntegerControl::new(
			ControlValueInteger::new(*value),
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
			ControlValueInteger::new(*value),
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
			ControlValueFloat::new(*value),
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
			ControlValueFloat::new(*value),
			*default,
			*step,
			*min,
			*max,
		)),
		ControlValueDescription::Boolean { value, default } => CamControl::Boolean(
			BooleanControl::new(ControlValueBoolean::new(*value), *default),
		),
		ControlValueDescription::String { value, default } => CamControl::String(
			StringControl::new(ControlValueString::new(value.clone()), default.clone()),
		),
		ControlValueDescription::Bytes { value, default } => CamControl::Bytes(BytesControl::new(
			ControlValueBytes::new(value.clone()),
			default.clone(),
		)),
		ControlValueDescription::KeyValuePair {
			key,
			value,
			default,
		} => CamControl::KeyValuePair(KeyValuePairControl::new(
			ControlValueKeyValue::new(KeyValue::new((*key, *value))),
			KeyValue::new(*default),
		)),
		ControlValueDescription::Point { value, default } => {
			CamControl::Point(PointControl::new(ControlValuePoint::new(*value), *default))
		},
		ControlValueDescription::Enum {
			value,
			possible,
			default,
		} => CamControl::Enum(EnumControl::new(
			ControlValueEnum::new(*value),
			*default,
			possible.clone(),
		)),
		ControlValueDescription::RGB {
			value,
			max,
			default,
		} => CamControl::RGB(RGBControl::new(
			ControlValueRGB::new(*value),
			*default,
			*max,
		)),
	}
}
