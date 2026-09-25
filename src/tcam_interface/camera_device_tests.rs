#![expect(clippy::print_stdout, reason = "Tests are allowed to print")]
#![expect(clippy::use_debug, reason = "Tests are a debug context")]
#![expect(clippy::panic, reason = "Tests are allowed to panic")]

use std::process::{ExitCode, Termination};

use nokhwa::utils::CameraControl;

use super::*;
use crate::{
	error::EdgeError,
	tcam_interface::controls::traits::{
		ControlItems as _, ControlMax as _, ControlStep as _,
	},
};

impl CameraDevice {
	fn get_raw_controls(&self) -> Vec<CameraControl> {
		self.inner.controls().unwrap()
	}
}

impl Termination for CameraDevice {
	fn report(self) -> ExitCode {
		ExitCode::SUCCESS
	}
}

#[test]
#[ignore = "IO bound"]
fn print_devices() {
	let cameras = CameraDevice::query_for_available_cameras();

	println!("Available devices: [");
	for cam in cameras {
		if cam.is_metadata() {
			println!("\tindex `{}` (metadata):", cam.index().as_string())
		} else {
			println!("\tindex `{}`:", cam.index().as_string())
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
#[ignore = "IO bound"]
fn new() -> Result<CameraDevice> {
	CameraDevice::try_new(CameraIndex::Index(0))
}

#[test]
#[ignore = "IO bound"]
fn print_camera_details() {
	let cam = CameraDevice::try_new(CameraIndex::Index(0)).unwrap();
	println!("{:#?}", cam.inner.info());
}

// Can't implement `Termination` for `RgbImage`.
#[test]
#[ignore = "IO bound"]
fn get_frame() {
	let mut cam = CameraDevice::try_new(CameraIndex::Index(0))
		.unwrap()
		.open()
		.unwrap();
	let _ = cam.get_frame_buffer().unwrap();
	let _ = cam.get_frame_buffer().unwrap();
}

#[test]
#[ignore = "IO bound"]
fn decode_image() {
	let mut cam = CameraDevice::try_new(CameraIndex::Index(0))
		.unwrap()
		.open()
		.unwrap();
	let frame = cam.get_frame_buffer().unwrap();
	let _ = cam.decode_buffer(frame);
}

#[test]
#[ignore = "IO bound"]
fn get_raw_controls() {
	let cam = CameraDevice::try_new(CameraIndex::Index(0)).unwrap();
	let c = cam.get_raw_controls();
	println!("{c:#?}");
}

#[test]
#[ignore = "IO bound"]
fn get_all_available_controls() {
	let cam = CameraDevice::try_new(CameraIndex::Index(0)).unwrap();
	let c = cam.get_controls();
	println!("{c}");
}

#[test]
#[ignore = "IO bound"]
fn get_stripped_controls() {
	let cam = CameraDevice::try_new(CameraIndex::Index(0)).unwrap();
	let c = cam.get_controls().stripped();
	println!("{c}");
}

#[test]
#[ignore = "IO bound"]
fn set_control() -> Result<()> {
	let mut cam = CameraDevice::try_new(CameraIndex::Index(0))
		.unwrap()
		.open()
		.unwrap();
	let id = KnownCameraControl::Brightness;

	// Assume it will always be this variant.
	let CamControl::IntegerRange(mut control) = cam.get_control(id) else {
		return Err(EdgeError::General {
			reason: "unexpected control variant".into(),
		});
	};
	println!("Start: {}", control.get_value().get_value());

	let new = control.get_max() - (control.get_step() * 8);
	control
		.set_value(new.into())
		.expect("new value should be valid");
	cam.set_control(id, control.get_value())?;

	let CamControl::IntegerRange(mut control) = cam.get_control(id) else {
		return Err(EdgeError::General {
			reason: "unexpected control variant".into(),
		});
	};
	println!("Modified: {}", control.get_value().get_value());

	control.reset_to_default();
	cam.set_control(id, control.get_value())?;

	let CamControl::IntegerRange(control) = cam.get_control(id) else {
		return Err(EdgeError::General {
			reason: "unexpected control variant".into(),
		});
	};
	println!("Default: {}", control.get_value().get_value());

	Ok(())
}

#[test]
#[ignore = "IO bound"]
fn set_control_exposure() -> Result<()> {
	let mut cam = CameraDevice::try_new(CameraIndex::Index(0))
		.unwrap()
		.open()
		.unwrap();
	let id = KnownCameraControl::AutoExposure;
	let desired = 1;

	// Assume it will always be this variant.
	let CamControl::Menu(mut control) = cam.get_control(id) else {
		return Err(EdgeError::General {
			reason: "unexpected control variant".into(),
		});
	};
	println!(
		"Start: {}, {:?}",
		control.get_value().get_value(),
		control.get_items()
	);

	control
		.set_value(desired.into())
		.expect("new value should be valid");
	cam.set_control(id, control.get_value())?;

	let CamControl::Menu(mut control) = cam.get_control(id) else {
		return Err(EdgeError::General {
			reason: "unexpected control variant".into(),
		});
	};
	println!(
		"Modified: {}, {:?}",
		control.get_value().get_value(),
		control.get_items()
	);

	control.reset_to_default();
	cam.set_control(id, control.get_value())?;

	let CamControl::Menu(control) = cam.get_control(id) else {
		return Err(EdgeError::General {
			reason: "unexpected control variant".into(),
		});
	};
	println!(
		"Default: {}, {:?}",
		control.get_value().get_value(),
		control.get_items()
	);

	Ok(())
}

#[test]
#[ignore = "IO bound"]
fn reset_controls_to_default() {
	let mut cam = CameraDevice::try_new(CameraIndex::Index(0))
		.unwrap()
		.open()
		.unwrap();
	cam.reset_controls_to_default();
}
