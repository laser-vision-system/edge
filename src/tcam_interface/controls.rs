//! Abstracts over [`nokhwa`]'s camera controls, providing a simplier interface for use.
//!
//! ## Design Notes
//!
//! - The output needs to be stored in a collection of sorts.
//!
//! - The order of the collection does not matter.
//!
//! - Each control will be unique to a [`KnownCameraControl`] (ID), condusive to a HashMap or Set.
//!
//! - Every [`CamControl`] ([`ControlValueDescription`]) maps non-commutatively to a [`ControlValue`] ([`ControlValueSetter`]), but cannot be determined at runtime.
//! This can be handled either by generic or trait objects (`dyn`).
//!
//! - There seems to be a weak and unclear mapping between [`KnownCameraControl`] and [`CamControl`] ([`ControlValueDescription`]).
//!
//! - What is stored in the collection either needs to be an enum or trait object.
//!
//! - The only important mapping that needs to be sent back to [`nokhwa`] is [`ControlValue`] ([`ControlValueSetter`]).
//!
//! ## TODO: Oh god fix the naming of the types.

use std::collections::HashMap;

#[allow(unused_imports, reason = "Needed for module docstring")]
use nokhwa::utils::ControlValueSetter;

use nokhwa::utils::{CameraControl, KnownCameraControl};

pub mod descriptions;
pub mod traits;
pub mod types;
pub mod values;

use descriptions::CamControl;

// TODO: Maybe make this into a struct itself.
pub type ControlMap = HashMap<KnownCameraControl, CamControl>;

/// Convert a collection of [`nokhwa`] [`CameraControl`] into a custom type.
pub fn extract_controls(nokhwa_controls: Vec<CameraControl>) -> ControlMap {
	nokhwa_controls
		.into_iter()
		.map(|control| (control.control(), extract_control(control)))
		.collect::<ControlMap>()
}

/// Convert a single [`nokhwa`] [`CameraControl`] into a custom type.
pub fn extract_control(nokhwa_control: CameraControl) -> CamControl {
	nokhwa_control.into()
}
