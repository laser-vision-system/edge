//! Abstracts over [`nokhwa`]'s camera controls, providing a simplier interface
//! for use.
//!
//! ## Design Notes
//!
//! - The output needs to be stored in a collection of sorts.
//!
//! - The order of the collection does not matter.
//!
//! - Each control will be unique to a [`KnownCameraControl`] (ID), condusive to
//!   a `HashMap` or Set.
//!
//! - Every [`CamControl`] ([`ControlValueDescription`]) maps non-commutatively
//!   to a [`ControlValue`] ([`ControlValueSetter`]), but cannot be determined
//!   at runtime. This can be handled either by generic or trait objects
//!   (`dyn`).
//!
//! - There seems to be a weak and unclear mapping between
//!   [`KnownCameraControl`] and [`CamControl`] ([`ControlValueDescription`]).
//!
//! - What is stored in the collection either needs to be an enum or trait
//!   object.
//!
//! - The only important mapping that needs to be sent back to [`nokhwa`] is
//!   [`ControlValue`] ([`ControlValueSetter`]).
//!
//! ## TODO: Oh god fix the naming of the types.

use std::{
	collections::BTreeMap,
	fmt,
	ops::{Deref, DerefMut},
};

#[expect(unused_imports, reason = "Needed for module docstring")]
use nokhwa::utils::ControlValueSetter;
use nokhwa::utils::{CameraControl, KnownCameraControl};

pub mod descriptions;
pub mod traits;
pub mod types;
pub mod values;

use descriptions::CamControl;
use serde::{Deserialize, Serialize};

type MapType = BTreeMap<KnownCameraControl, CamControl>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ControlMap {
	inner: MapType,
}

impl ControlMap {
	pub fn into_inner(self) -> MapType {
		self.inner
	}

	#[must_use]
	pub fn stripped(self) -> Self {
		let inner = self
			.inner
			.into_iter()
			.filter(|(key, _)| {
				[
					KnownCameraControl::Brightness,
					KnownCameraControl::Contrast,
					KnownCameraControl::Saturation,
					KnownCameraControl::Hue,
					KnownCameraControl::Gain,
					KnownCameraControl::Gamma,
				]
				.contains(key)
			})
			.collect();

		Self { inner }
	}

	pub fn serialise(&self) -> String {
		serde_json::to_string(&self.inner).expect(
			"everything in the inner map should be correcly serialisable",
		)
	}
}

impl From<Vec<CameraControl>> for ControlMap {
	/// Convert a collection of [`nokhwa`] [`CameraControl`] into a custom type.
	fn from(nokhwa_controls: Vec<CameraControl>) -> Self {
		let inner = nokhwa_controls
			.into_iter()
			.map(|control| (control.control(), control.into()))
			.collect::<MapType>();
		Self { inner }
	}
}

impl Deref for ControlMap {
	type Target = MapType;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for ControlMap {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl fmt::Display for ControlMap {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.serialise())
	}
}
