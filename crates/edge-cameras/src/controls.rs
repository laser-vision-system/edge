//! Abstracts over [`nokhwa`]'s camera controls, providing a simplier interface
//! for use.
//!
//! ## Design Notes
//!
//! - The output needs to be stored in a collection of sorts.
//!
//! - The order of the collection does not matter; however, it is useful for
//!   other systems.
//!
//! - Each control will be unique to a [`KnownCameraControl`] (ID), condusive to
//!   a [`HashMap`]/[`BTreeMap`], or [`HashSet`].
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
//! [`HashMap`]: std::collections::HashMap
//! [`HashSet`]: std::collections::HashSet
//! [`ControlValueDescription`]: nokhwa::utils::ControlValueDescription
//! [`ControlValueSetter`]: nokhwa::utils::ControlValueSetter
//! [`ControlValue`]: self::traits::ControlValue
//!
//! ## TODO: Oh god fix the naming of the types.

use std::{
	collections::BTreeMap,
	fmt,
	ops::{
		Deref,
		DerefMut,
	},
};

use nokhwa::utils::CameraControl;
pub use nokhwa::utils::{
	ControlValueSetter,
	KnownCameraControl,
	KnownCameraControlFlag,
};

pub mod automatic;
pub mod descriptions;
pub mod traits;
pub mod types;
pub mod values;

use descriptions::CamControl;
use serde::{
	Deserialize,
	Serialize,
};

type MapType = BTreeMap<KnownCameraControl, CamControl>;

/// Every control description that a camera possesses, mapped by its control
/// ID.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ControlMap {
	inner: MapType,
}

impl ControlMap {
	/// Create an empty control map.
	#[must_use]
	pub fn empty() -> Self {
		Self {
			inner: MapType::new(),
		}
	}

	/// Consume this type to produce the inner map type.
	#[must_use]
	pub fn into_inner(self) -> MapType {
		self.inner
	}

	/// Get the reported flags for a control ID.
	#[must_use]
	pub fn control_flags(
		&self,
		id: KnownCameraControl,
	) -> Option<&[KnownCameraControlFlag]> {
		self.inner.get(&id).map(CamControl::control_flags)
	}

	/// Check whether a control has a specific flag.
	#[must_use]
	pub fn has_flag(
		&self,
		id: KnownCameraControl,
		flag: KnownCameraControlFlag,
	) -> bool {
		self.inner
			.get(&id)
			.is_some_and(|control| control.has_flag(flag))
	}

	/// Whether a control is reported as automatic.
	#[must_use]
	pub fn is_auto(
		&self,
		id: KnownCameraControl,
	) -> bool {
		self.inner.get(&id).is_some_and(CamControl::is_auto)
	}

	/// Whether a control is reported as manual.
	#[must_use]
	pub fn is_manual(
		&self,
		id: KnownCameraControl,
	) -> bool {
		self.inner.get(&id).is_some_and(CamControl::is_manual)
	}

	/// Whether a control is active and readable.
	#[must_use]
	pub fn is_readable(
		&self,
		id: KnownCameraControl,
	) -> bool {
		self.inner.get(&id).is_some_and(CamControl::is_readable)
	}

	/// Whether a control is active and writable.
	#[must_use]
	pub fn is_writable(
		&self,
		id: KnownCameraControl,
	) -> bool {
		self.inner.get(&id).is_some_and(CamControl::is_writable)
	}

	/// Consume this type to produce one with only certain control IDs.
	///
	/// The list of IDs that are retained:
	///
	/// - [`KnownCameraControl::Brightness`]
	/// - [`KnownCameraControl::Contrast`]
	/// - [`KnownCameraControl::Saturation`]
	/// - [`KnownCameraControl::Hue`]
	/// - [`KnownCameraControl::Gain`]
	/// - [`KnownCameraControl::Gamma`]
	#[must_use]
	pub fn to_stripped(self) -> Self {
		let retained = [
			KnownCameraControl::Brightness,
			KnownCameraControl::Contrast,
			KnownCameraControl::Saturation,
			KnownCameraControl::Hue,
			KnownCameraControl::Gain,
			KnownCameraControl::Gamma,
		];
		let inner = self
			.inner
			.into_iter()
			.filter(|(key, _)| retained.contains(key))
			.collect();
		Self {
			inner,
		}
	}

	/// Serialise the control map.
	///
	/// # Panics
	///
	/// If [`serde_json`] could not parse the underlying [`BTreeMap`].
	#[expect(clippy::expect_used, reason = "Low priority edge case")]
	pub fn serialise(&self) -> String {
		serde_json::to_string(&self.inner).expect(
			"everything in the inner map should be correctly serialisable",
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
		Self {
			inner,
		}
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
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "{}", self.serialise())
	}
}

#[cfg(test)]
mod tests {
	use super::{
		descriptions::IntegerControl,
		values::ConValueInteger,
		*,
	};

	#[test]
	fn control_map_serialisation_round_trip_preserves_flags() {
		let mut map = ControlMap::empty();
		let control = CamControl::Integer(IntegerControl::new(
			KnownCameraControl::Brightness,
			ConValueInteger::from(42),
			0,
			1,
			vec![
				KnownCameraControlFlag::Manual,
				KnownCameraControlFlag::ReadOnly,
			],
		));
		map.insert(KnownCameraControl::Brightness, control);

		let serialised = map.serialise();
		let deserialised: ControlMap = serde_json::from_str(&serialised)
			.expect("deserialisation should succeed");

		assert_eq!(
			deserialised.control_flags(KnownCameraControl::Brightness),
			Some(
				[
					KnownCameraControlFlag::Manual,
					KnownCameraControlFlag::ReadOnly,
				]
				.as_slice()
			)
		);
		assert!(deserialised.is_manual(KnownCameraControl::Brightness));
		assert!(!deserialised.is_auto(KnownCameraControl::Brightness));
		assert!(deserialised.is_readable(KnownCameraControl::Brightness));
		assert!(!deserialised.is_writable(KnownCameraControl::Brightness));
	}

	#[test]
	fn control_permission_flags_govern_read_and_write_availability() {
		let mut map = ControlMap::empty();

		// Read-only control
		let read_only = CamControl::Integer(IntegerControl::new(
			KnownCameraControl::Pan,
			ConValueInteger::from(0),
			0,
			1,
			vec![KnownCameraControlFlag::ReadOnly],
		));
		map.insert(KnownCameraControl::Pan, read_only);
		assert!(map.is_readable(KnownCameraControl::Pan));
		assert!(!map.is_writable(KnownCameraControl::Pan));

		// Write-only control
		let write_only = CamControl::Integer(IntegerControl::new(
			KnownCameraControl::Tilt,
			ConValueInteger::from(0),
			0,
			1,
			vec![KnownCameraControlFlag::WriteOnly],
		));
		map.insert(KnownCameraControl::Tilt, write_only);
		assert!(!map.is_readable(KnownCameraControl::Tilt));
		assert!(map.is_writable(KnownCameraControl::Tilt));

		// Disabled control
		let disabled = CamControl::Integer(IntegerControl::new(
			KnownCameraControl::Zoom,
			ConValueInteger::from(0),
			0,
			1,
			vec![KnownCameraControlFlag::Disabled],
		));
		map.insert(KnownCameraControl::Zoom, disabled);
		assert!(!map.is_readable(KnownCameraControl::Zoom));
		assert!(!map.is_writable(KnownCameraControl::Zoom));

		// Volatile control
		let volatile = CamControl::Integer(IntegerControl::new(
			KnownCameraControl::Focus,
			ConValueInteger::from(0),
			0,
			1,
			vec![KnownCameraControlFlag::Volatile],
		));
		map.insert(KnownCameraControl::Focus, volatile);
		assert!(!map.is_readable(KnownCameraControl::Focus));
		assert!(!map.is_writable(KnownCameraControl::Focus));
	}
}
