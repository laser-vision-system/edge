//! Reusable camera and control diagnostic operations.

use std::fmt;

use edge_core::logger::logger;
use nokhwa::utils::{
	CameraIndex,
	CameraInfo,
	KnownCameraControl,
};

use super::{
	CameraDevice,
	query_for_available_cameras,
};
use crate::{
	camera::CameraLike as _,
	controls::{
		ControlMap,
		automatic::dependent_auto_control,
		descriptions::CamControl,
		traits::{
			ControlDescription as _,
			ControlMax as _,
			ControlMin as _,
			ControlValue as _,
		},
	},
	error::{
		EdgeCamerasError,
		EdgeCamerasResult,
	},
};

/// A snapshot of a camera's current state.
#[derive(Clone, Debug)]
pub struct CameraDiagnostics {
	/// Device metadata reported by Nokhwa.
	pub info: CameraInfo,
	/// Controls mapped to the edge-cameras control types.
	pub controls: ControlMap,
}

impl fmt::Display for CameraDiagnostics {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		writeln!(f, "Camera: {}", self.info.human_name())?;
		writeln!(f, "Index: {}", self.info.index())?;
		writeln!(f, "Description: {}", self.info.description())?;
		writeln!(f, "Mapped controls:")?;
		writeln!(f, "{}", self.controls)
	}
}

/// Inspect one device's metadata and control descriptions.
///
/// # Errors
///
/// If the device cannot be opened, its controls cannot be read, or Nokhwa
/// panics while reading controls.
pub fn inspect_camera(
	index: CameraIndex
) -> EdgeCamerasResult<CameraDiagnostics> {
	let camera = CameraDevice::try_new(index)?;
	Ok(CameraDiagnostics {
		info: camera.get_info().clone(),
		controls: camera.get_controls()?,
	})
}

/// Inspect all enumerated physical camera devices, excluding metadata nodes.
///
/// # Errors
///
/// If any physical camera cannot be opened or its controls cannot be read.
///
/// # Panics
///
/// If no Nokhwa camera backend is available.
pub fn inspect_connected_cameras() -> EdgeCamerasResult<Vec<CameraDiagnostics>>
{
	query_for_available_cameras()
		.into_iter()
		.filter(|info| !info.is_metadata())
		.map(|info| inspect_camera(info.index().clone()))
		.collect()
}

/// Result of probing a dependent control while preserving the original state.
#[derive(Clone, Debug)]
pub struct DependentControlProbe {
	/// The dependent control that was probed.
	pub control: KnownCameraControl,
	/// The automatic control that governs the dependent control.
	pub automatic_control: KnownCameraControl,
	/// Whether the alternate value was accepted by `set_control`.
	pub alternate_write_succeeded: bool,
	/// Whether the alternate value was observed on read-back after the write.
	pub alternate_value_observed: Option<bool>,
	/// Backend error from the alternate write, if it was rejected.
	pub alternate_write_error: Option<String>,
	/// Whether the requested default was observed after reset.
	pub default_value_applied: Option<bool>,
	/// Whether the automatic setting remained unchanged through the probe.
	pub automatic_setting_preserved: Option<bool>,
}

/// Probe a dependent integer-range control and restore its original value.
///
/// The function attempts an alternate value, resets to the reported default,
/// checks that the associated automatic mode was preserved, and restores the
/// original control value before returning.
///
/// # Errors
///
/// If the control or automatic mode cannot be read, reset/restoration fails,
/// or a backend operation panics.
pub fn probe_dependent_control(
	camera: &mut CameraDevice,
	id: KnownCameraControl,
) -> EdgeCamerasResult<Option<DependentControlProbe>> {
	let Some(auto_id) = dependent_auto_control(id) else {
		return Ok(None);
	};
	let original = camera.get_control(id)?;
	let Some(alternate) = alternate_integer_range_value(&original) else {
		return Ok(None);
	};
	let automatic_before = match camera.get_control(auto_id) {
		Ok(control) => Some(control),
		Err(error @ EdgeCamerasError::GetControl(..)) => {
			logger().log_error(&error);
			None
		},
		Err(error) => return Err(error),
	};

	let probe_result = run_probe(
		camera,
		id,
		auto_id,
		&original,
		alternate,
		automatic_before.as_ref(),
	);

	let restore = camera.set_control(original);
	match (probe_result, restore) {
		(Err(error), _) | (Ok(_), Err(error)) => Err(error),
		(Ok(report), Ok(())) => Ok(Some(report)),
	}
}

fn run_probe(
	camera: &mut CameraDevice,
	id: KnownCameraControl,
	auto_id: KnownCameraControl,
	original: &CamControl,
	alternate: CamControl,
	automatic_before: Option<&CamControl>,
) -> Result<DependentControlProbe, EdgeCamerasError> {
	let alternate_value = alternate.try_display_control_value();
	let write = camera.set_control(alternate);
	let write_succeeded = write.is_ok();
	let write_error = write.err().map(|error| error.to_string());
	let observed_value = camera.get_control(id)?.try_display_control_value();
	let alternate_observed = alternate_value
		.as_ref()
		.map(|value| observed_value.as_ref() == Some(value));
	let default = original.try_display_control_default();
	let reset = camera.set_control_to_default(id);
	let (default_applied, automatic_preserved) = match reset {
		Ok(()) => {
			let actual = camera.get_control(id)?.try_display_control_value();
			let applied = default
				.as_ref()
				.map(|default| actual.as_deref() == Some(default.as_str()));
			let preserved = if let Some(before) = &automatic_before {
				Some(camera.get_control(auto_id)? == **before)
			} else {
				None
			};
			(applied, preserved)
		},
		Err(EdgeCamerasError::ControlDefaultUnavailable(_)) => (None, None),
		Err(error) => return Err(error),
	};
	Ok(DependentControlProbe {
		control: id,
		automatic_control: auto_id,
		alternate_write_succeeded: write_succeeded,
		alternate_value_observed: alternate_observed,
		alternate_write_error: write_error,
		default_value_applied: default_applied,
		automatic_setting_preserved: automatic_preserved,
	})
}

fn alternate_integer_range_value(control: &CamControl) -> Option<CamControl> {
	let CamControl::IntegerRange(control) = control else {
		return None;
	};
	let mut control = control.clone();
	let current = control.control_value().get();
	let candidate = if current != control.control_min() {
		control.control_min()
	} else if current != control.control_max() {
		control.control_max()
	} else {
		return None;
	};
	control.try_set_control_value(candidate.into()).ok()?;
	Some(CamControl::from(control))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore = "IO bound; for manual inspection"]
	fn display_devices() {
		let cameras = query_for_available_cameras();

		println!("Available devices: [");
		for cam in cameras {
			if cam.is_metadata() {
				println!("\tindex `{}` (metadata):", cam.index().as_string());
			} else {
				println!("\tindex `{}`:", cam.index().as_string());
			}
			println!(
				"\t\tname: {}\n\t\tdesc: {}",
				cam.human_name(),
				cam.description()
			);
			let misc = cam.misc();
			if !misc.is_empty() {
				println!("\t\tmisc: {misc}");
			}
		}
		println!("]");
	}

	#[test]
	#[ignore = "IO bound; for manual inspection"]
	fn display_camera_details() -> EdgeCamerasResult {
		for diagnostic in inspect_connected_cameras()? {
			println!("{:#?}", diagnostic.info);
		}
		Ok(())
	}

	#[test]
	#[ignore = "IO bound; for manual inspection"]
	fn display_all_available_controls() -> EdgeCamerasResult {
		for diagnostic in inspect_connected_cameras()? {
			println!("{:#?}", diagnostic.controls);
		}
		Ok(())
	}

	#[test]
	#[ignore = "IO bound; for manual inspection"]
	fn get_stripped_controls() -> EdgeCamerasResult {
		for diagnostic in inspect_connected_cameras()? {
			println!("{}", diagnostic.controls.clone().to_stripped());
		}
		Ok(())
	}
}
