//! Camera device unit tests and diagnostics.
//!
//! Make sure to run tests with `--test-threads 1` to avoid tests fighting over
//! camera connections. E.g.: `cargo test -p edge-cameras camera --
//! --include-ignored --no-capture --test-threads 1`

use super::{
	super::run_backend_operation,
	CameraDevice,
	KnownCameraControl,
};
use crate::{
	camera::{
		CameraLike,
		diagnostics::{
			inspect_connected_cameras,
			probe_dependent_control,
		},
	},
	controls::{
		ControlMap,
		descriptions::CamControl,
	},
	error::{
		EdgeCamerasError,
		EdgeCamerasResult,
	},
};

fn assert_condition(
	condition: bool,
	message: String,
) -> EdgeCamerasResult {
	if condition {
		Ok(())
	} else {
		Err(EdgeCamerasError::General(message))
	}
}

#[test]
fn control_operation_panics_are_returned_as_errors() {
	let result: EdgeCamerasResult =
		run_backend_operation("test control operation", || {
			std::panic::panic_any("test panic")
		});

	assert!(matches!(
		result,
		Err(EdgeCamerasError::ControlOperationPanicked {
			operation: "test control operation",
			message,
		}) if message == "test panic"
	));
}

#[test]
#[ignore = "IO bound; exercises enumerated physical cameras"]
fn all_control_operations_work_on_connected_cameras() -> EdgeCamerasResult {
	let diagnostics = inspect_connected_cameras()?;
	if diagnostics.is_empty() {
		println!("No physical cameras found; skipping hardware checks.");
		return Ok(());
	}

	for diagnostic in diagnostics {
		let index = diagnostic.info.index().clone();
		let mut camera = CameraDevice::try_new(index.clone())?;
		let controls = diagnostic.controls;
		let supported = camera.get_all_supported_controls()?;
		assert_condition(
			supported == controls.keys().copied().collect::<Vec<_>>(),
			format!("control enumeration mismatch at {index}"),
		)?;

		for id in supported.iter().copied() {
			let control = camera.get_control(id)?;
			if control != CamControl::None {
				assert_condition(
					control.control_id()? == id,
					format!("control description mismatch at {index}: {id}"),
				)?;
			}
		}

		let mut bulk = ControlMap::empty();
		for id in supported.iter().copied() {
			if !controls.is_readable(id) || !controls.is_writable(id) {
				continue;
			}
			let control = camera.get_control(id)?;
			if control == CamControl::None {
				continue;
			}
			camera.set_control(control.clone())?;
			assert_condition(
				camera.get_control(id)?.try_display_control_value()
					== control.try_display_control_value(),
				format!("control read-back mismatch at {index}: {id}"),
			)?;
			if bulk.is_empty() {
				bulk.insert(id, control);
			}
		}
		if !bulk.is_empty() {
			camera.set_controls(bulk)?;
		}
		for id in supported {
			match camera.set_control_to_default(id) {
				Ok(())
				| Err(
					EdgeCamerasError::ControlDefaultUnavailable(_)
					| EdgeCamerasError::NoControlDescription,
				) => (),
				Err(error) => return Err(error),
			}
		}
		camera.reset_controls_to_default()?;
		camera.get_controls()?;
	}
	Ok(())
}

#[test]
#[ignore = "IO bound; verify auto control dependency management"]
fn dependent_control_resets_restore_automatic_modes() -> EdgeCamerasResult {
	let infos = inspect_connected_cameras()?;
	if infos.is_empty() {
		println!("No physical cameras found; skipping dependency checks.");
		return Ok(());
	}

	for info in infos {
		let index = info.info.index().clone();
		let mut camera = CameraDevice::try_new(index.clone())?;
		for (id, auto_id) in [
			(
				KnownCameraControl::ExposureAbsolute,
				KnownCameraControl::AutoExposure,
			),
			(
				KnownCameraControl::WhiteBalance,
				KnownCameraControl::AutoWhiteBalance,
			),
		] {
			let Some(report) = probe_dependent_control(&mut camera, id)? else {
				continue;
			};
			assert_condition(
				report.default_value_applied.unwrap_or(true),
				format!("{index}: {id}: default was not applied"),
			)?;
			assert_condition(
				report.automatic_setting_preserved.unwrap_or(true),
				format!("{index}: {auto_id} was not preserved"),
			)?;
			if report.alternate_write_succeeded {
				assert_condition(
					report.alternate_value_observed.unwrap_or(false),
					format!("{index}: {id}: alternate value was not observed"),
				)?;
			}
			println!("{index}: {report:#?}");
		}
	}
	Ok(())
}

#[test]
#[ignore = "IO bound"]
fn reset_defaults_on_each_connected_camera() -> EdgeCamerasResult {
	for info in inspect_connected_cameras()? {
		let mut camera =
			CameraDevice::try_new(info.info.index().clone())?.open()?;
		camera.reset_controls_to_default()?;
		camera.get_controls()?;
	}
	Ok(())
}
