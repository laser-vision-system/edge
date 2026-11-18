//! Helpers for camera controls that switch between automatic and manual modes.

use nokhwa::utils::{
	ControlValueSetter,
	KnownCameraControl,
};

use super::{
	descriptions::CamControl,
	traits::{
		ControlDescription as _,
		ControlItems as _,
		ControlPossible as _,
	},
};

/// Return the control that governs a manually adjustable camera control.
#[must_use]
pub fn dependent_auto_control(
	id: KnownCameraControl
) -> Option<KnownCameraControl> {
	match id {
		KnownCameraControl::ExposureAbsolute => {
			Some(KnownCameraControl::AutoExposure)
		},
		KnownCameraControl::WhiteBalance => {
			Some(KnownCameraControl::AutoWhiteBalance)
		},
		_ => None,
	}
}

/// Determine a supported manual/off value without guessing unknown variants.
#[must_use]
pub fn manual_mode_value(
	id: KnownCameraControl,
	control: &CamControl,
) -> Option<ControlValueSetter> {
	let standard_mode = match id {
		KnownCameraControl::AutoExposure => Some(1_i64),
		KnownCameraControl::AutoWhiteBalance => None,
		_ => return None,
	};

	if matches!(control, CamControl::Boolean(_)) {
		return Some(ControlValueSetter::Boolean(false));
	}

	match control {
		CamControl::Menu(control) => {
			let words = if id == KnownCameraControl::AutoExposure {
				["manual"].as_slice()
			} else {
				["manual", "off", "disabled"].as_slice()
			};
			let items = control.control_items();
			let labelled = items.iter().find(|entry| {
				let label = entry.get_item().to_string().to_lowercase();
				words.iter().any(|word| label.contains(word))
			});
			let value = labelled
				.map(nokhwa::utils::MenuEntry::get_id)
				.or_else(|| {
					let standard = u32::try_from(standard_mode?).ok()?;
					items
						.iter()
						.any(|entry| entry.get_id() == standard)
						.then_some(standard)
				})?;
			Some(ControlValueSetter::Menu(value))
		},
		CamControl::Enum(control) => {
			let value = standard_mode?;
			control
				.control_possible()
				.contains(&value)
				.then_some(ControlValueSetter::EnumValue(value))
		},
		CamControl::Integer(control) => {
			let mut control = control.clone();
			control.try_set_control_value(standard_mode?.into()).ok()?;
			Some(control.control_value().into())
		},
		CamControl::IntegerRange(control) => {
			let mut control = control.clone();
			control.try_set_control_value(standard_mode?.into()).ok()?;
			Some(control.control_value().into())
		},
		_ => None,
	}
}

#[cfg(test)]
mod tests {
	use nokhwa::utils::{
		MenuEntry,
		MenuItem,
	};

	use super::*;
	use crate::controls::{
		descriptions::{
			BooleanControl,
			EnumControl,
			MenuControl,
			StringControl,
		},
		values::{
			ConValueBoolean,
			ConValueEnum,
			ConValueMenu,
		},
	};

	#[test]
	fn white_balance_boolean_selects_off() {
		let control = CamControl::Boolean(BooleanControl::new(
			KnownCameraControl::AutoWhiteBalance,
			ConValueBoolean::from(true),
			true,
			Vec::new(),
		));
		assert_eq!(
			manual_mode_value(KnownCameraControl::AutoWhiteBalance, &control),
			Some(ControlValueSetter::Boolean(false))
		);
	}

	#[test]
	fn exposure_menu_prefers_labelled_manual_item() {
		let control = CamControl::Menu(MenuControl::new(
			KnownCameraControl::AutoExposure,
			ConValueMenu::from(0),
			0,
			vec![
				MenuEntry::new(0, MenuItem::Name("Automatic".into())),
				MenuEntry::new(1, MenuItem::Name("Manual".into())),
			]
			.into(),
			Vec::new(),
		));
		assert_eq!(
			manual_mode_value(KnownCameraControl::AutoExposure, &control),
			Some(ControlValueSetter::Menu(1))
		);
	}

	#[test]
	fn exposure_menu_accepts_standard_manual_id() {
		let control = CamControl::Menu(MenuControl::new(
			KnownCameraControl::AutoExposure,
			ConValueMenu::from(3),
			3,
			vec![
				MenuEntry::new(1, MenuItem::Value(1)),
				MenuEntry::new(3, MenuItem::Value(3)),
			]
			.into(),
			Vec::new(),
		));
		assert_eq!(
			manual_mode_value(KnownCameraControl::AutoExposure, &control),
			Some(ControlValueSetter::Menu(1))
		);
	}

	#[test]
	fn white_balance_menu_recognises_off_label() {
		let control = CamControl::Menu(MenuControl::new(
			KnownCameraControl::AutoWhiteBalance,
			ConValueMenu::from(1),
			1,
			vec![
				MenuEntry::new(1, MenuItem::Name("Automatic".into())),
				MenuEntry::new(2, MenuItem::Name("Off".into())),
			]
			.into(),
			Vec::new(),
		));
		assert_eq!(
			manual_mode_value(KnownCameraControl::AutoWhiteBalance, &control),
			Some(ControlValueSetter::Menu(2))
		);
	}

	#[test]
	fn exposure_enum_requires_supported_manual_value() {
		let control = CamControl::Enum(EnumControl::new(
			KnownCameraControl::AutoExposure,
			ConValueEnum::from(1),
			1,
			vec![0, 1].into(),
			Vec::new(),
		));
		assert_eq!(
			manual_mode_value(KnownCameraControl::AutoExposure, &control),
			Some(ControlValueSetter::EnumValue(1))
		);
	}

	#[test]
	fn unknown_control_variant_is_not_guessed() {
		let control = CamControl::String(StringControl::new(
			KnownCameraControl::AutoExposure,
			"auto".to_owned().into(),
			None,
			Vec::new(),
		));
		assert_eq!(
			manual_mode_value(KnownCameraControl::AutoExposure, &control),
			None
		);
	}
}
