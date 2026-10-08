use std::{
	cell::RefCell,
	process::{ExitCode, Termination},
	rc::Rc,
};

use nokhwa::utils::CameraControl;

use super::*;
use crate::{
	error::EdgeError,
	tcam_interface::controls::traits::{
		ControlDefault as _, ControlItems as _, ControlMax as _,
		ControlStep as _, ControlValue as _,
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

struct FakeCamera {
	info: CameraInfo,
	set_calls: Rc<RefCell<Vec<(KnownCameraControl, ControlValueSetter)>>>,
}

impl FakeCamera {
	fn new() -> (
		Self,
		Rc<RefCell<Vec<(KnownCameraControl, ControlValueSetter)>>>,
	) {
		let set_calls = Rc::new(RefCell::new(Vec::new()));
		let camera = Self {
			info: CameraInfo::new(
				"fake-camera".into(),
				"test".into(),
				String::new(),
				false,
				CameraIndex::Index(0),
			),
			set_calls: set_calls.clone(),
		};
		(camera, set_calls)
	}
}

impl CameraLike for FakeCamera {
	fn get_info(&self) -> &CameraInfo {
		&self.info
	}

	fn get_index(&self) -> &CameraIndex {
		self.info.index()
	}

	fn get_frame_buffer(&mut self) -> Result<FrameBuffer> {
		Err(EdgeError::General {
			reason: "fake camera does not provide frames".into(),
		})
	}

	fn get_all_supported_controls(&self) -> Result<Vec<KnownCameraControl>> {
		Ok(Vec::new())
	}

	fn get_controls(&self) -> Result<ControlMap> {
		Ok(ControlMap::from(Vec::new()))
	}

	fn get_control(&self, id: KnownCameraControl) -> Result<CamControl> {
		Err(CameraDeviceError::GetControl(id).into())
	}

	fn set_control(
		&mut self,
		id: KnownCameraControl,
		value: ControlValueSetter,
	) -> Result<()> {
		self.set_calls.borrow_mut().push((id, value));
		Ok(())
	}

	fn set_controls(&mut self, _controls: ControlMap) -> Result<()> {
		Ok(())
	}
}

#[test]
fn camera_trait_is_testable_without_a_device() {
	let (fake_camera, calls) = FakeCamera::new();
	let mut camera: Box<dyn CameraLike> = Box::new(fake_camera);
	let value = ControlValueSetter::Integer(42);

	camera
		.set_control(KnownCameraControl::Brightness, value.clone())
		.unwrap();

	let recorded_calls = calls.borrow();
	assert_eq!(recorded_calls.len(), 1);
	assert_eq!(recorded_calls[0].0, KnownCameraControl::Brightness);
	assert_eq!(recorded_calls[0].1, value);
}

#[test]
#[ignore = "IO bound"]
fn print_devices() {
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
fn get_raw_controls() {
	let cam = CameraDevice::try_new(CameraIndex::Index(0)).unwrap();
	let controls = cam.get_raw_controls();
	println!("{controls:#?}");
}

#[test]
#[ignore = "IO bound"]
fn get_all_available_controls() {
	let cam = CameraDevice::try_new(CameraIndex::Index(0)).unwrap();
	let controls = cam.get_controls().unwrap();
	println!("{controls}");
}

#[test]
#[ignore = "IO bound"]
fn get_stripped_controls() {
	let cam = CameraDevice::try_new(CameraIndex::Index(0)).unwrap();
	let controls = cam.get_controls().unwrap().stripped();
	println!("{controls}");
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
	let CamControl::IntegerRange(mut control) = cam.get_control(id)? else {
		return Err(EdgeError::General {
			reason: "unexpected control variant".into(),
		});
	};
	println!("Start: {}", control.get_value().get_value());

	let new = control.get_max() - (control.get_step() * 8);
	control
		.set_value(new.into())
		.expect("new value should be valid");
	cam.set_control_value(id, control.get_value())?;

	let CamControl::IntegerRange(mut control) = cam.get_control(id)? else {
		return Err(EdgeError::General {
			reason: "unexpected control variant".into(),
		});
	};
	println!("Modified: {}", control.get_value().get_value());

	control.reset_to_default();
	cam.set_control_value(id, control.get_value())?;

	let CamControl::IntegerRange(control) = cam.get_control(id)? else {
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
	let CamControl::Menu(mut control) = cam.get_control(id)? else {
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
	cam.set_control_value(id, control.get_value())?;

	let CamControl::Menu(mut control) = cam.get_control(id)? else {
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
	cam.set_control_value(id, control.get_value())?;

	let CamControl::Menu(control) = cam.get_control(id)? else {
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

//#[test]
//#[ignore = "IO bound"]
//fn reset_controls_to_default() {
//	let mut cam = CameraDevice::try_new(CameraIndex::Index(0))
//		.unwrap()
//		.open()
//		.unwrap();
//	cam.reset_controls_to_default();
//}
