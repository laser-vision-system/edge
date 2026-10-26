//! Defines for a camera control that consists of a `value`, `default`, and list
//! of `items`.

use std::fmt;

use nokhwa::utils::KnownCameraControl;
use serde::{
	Deserialize,
	Serialize,
};

use super::super::{
	traits::{
		ControlDefault,
		ControlDescription,
		ControlItems,
		ControlValue as _,
	},
	types::ConTypeMenu,
	values::{
		ConValueMenu,
		ConValueMenuType,
	},
};
use crate::{
	controls::types::ConTypeItems,
	error::{
		EdgeCamerasError,
		EdgeCamerasResult,
	},
};

/// A camera control that consists of a `value`, `default`, and list of `items`.
///
/// Maps to [`ControlValueDescription::Menu`].
///
/// [`ControlValueDescription::Menu`]: nokhwa::utils::ControlValueDescription::Menu
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuControl {
	control_type: KnownCameraControl,
	value: ConValueMenu,
	default: ConValueMenuType,
	items: ConTypeItems,
}

impl MenuControl {
	/// Create a new [`MenuControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueMenu,
		default: ConValueMenuType,
		items: ConTypeItems,
	) -> Self {
		Self {
			control_type,
			value,
			default,
			items,
		}
	}
}

impl ControlDescription for MenuControl {
	type ConType = ConTypeMenu;
	type ValueType = ConValueMenu;

	fn control_id(&self) -> KnownCameraControl {
		self.control_type
	}

	fn try_set_control_value(
		&mut self,
		value: Self::ValueType,
	) -> EdgeCamerasResult<()> {
		if self.validate(value.get()) {
			self.value = value;
			Ok(())
		} else {
			Err(EdgeCamerasError::InvalidControlValue)
		}
	}

	fn control_value(&self) -> Self::ValueType {
		self.value
	}
}

impl ControlDefault for MenuControl {
	type DefaultType = Self::ConType;

	fn control_default(&self) -> Self::DefaultType {
		self.default
	}
}

impl ControlItems for MenuControl {
	type ItemType = ConTypeItems;

	fn control_items(&self) -> Self::ItemType {
		self.items.clone()
	}

	fn validate(
		&self,
		value: Self::ConType,
	) -> bool {
		self.items.iter().any(|entry| entry.get_id() == value)
	}
}

impl fmt::Display for MenuControl {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		if f.alternate() {
			write!(
				f,
				"{} {} *{} {}",
				self.control_type, self.value, self.default, self.items
			)
		} else {
			write!(
				f,
				"type: '{}' value: '{}' default: '{}' items: '{}'",
				self.control_type, self.value, self.default, self.items
			)
		}
	}
}

#[cfg(test)]
mod tests {
	use nokhwa::utils::{
		MenuEntry,
		MenuItem,
	};

	use super::*;

	#[test]
	#[ignore = "for manual inspection"]
	fn control_display() {
		let control_type = KnownCameraControl::Pan;
		let value = ConValueMenu::from(0);
		let default = 0;
		let possible =
			ConTypeItems::from(vec![MenuEntry::new(0, MenuItem::Value(0)); 5]);
		let control = MenuControl::new(control_type, value, default, possible);
		super::super::tests::display_with_different_formats(
			"MenuControl",
			control,
		);
	}
}
