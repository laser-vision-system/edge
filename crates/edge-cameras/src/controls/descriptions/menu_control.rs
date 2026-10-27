//! Defines for a camera control that consists of a `value`, `default`, and list
//! of `items`.

use nokhwa::utils::{
	KnownCameraControl,
	MenuEntry,
};
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
use crate::error::{
	EdgeCamerasError,
	EdgeCamerasResult,
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
	items: Vec<MenuEntry>,
}

impl MenuControl {
	/// Create a new [`MenuControl`] instance.
	pub fn new(
		control_type: KnownCameraControl,
		value: ConValueMenu,
		default: ConValueMenuType,
		items: Vec<MenuEntry>,
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
	) -> EdgeCamerasResult {
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
	type ItemType = Vec<MenuEntry>;

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
