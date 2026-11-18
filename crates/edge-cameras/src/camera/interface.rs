//! An abstraction over the camera device for test fixtures to use.

use edge_core::image::FrameBuffer;
use nokhwa::utils::{
	CameraIndex,
	CameraInfo,
	KnownCameraControl,
};

use crate::{
	controls::{
		ControlMap,
		descriptions::CamControl,
	},
	error::EdgeCamerasResult,
};

/// An abstraction over the functionalities that a camera would provide.
///
/// This is used to allow for testing fixtures to be used in place of real
/// cameras for unit tests.
pub trait CameraLike {
	/// Get the [`CameraInfo`] struct.
	fn get_info(&self) -> &CameraInfo;

	/// Get the [`CameraIndex`] enum.
	fn get_index(&self) -> &CameraIndex;

	/// Request an image of the current camera frame.
	///
	/// Latency is based on the buffer count.
	///
	/// # Errors
	///
	/// If the stream hasn't been opened yet.
	fn get_frame_buffer(&mut self) -> EdgeCamerasResult<FrameBuffer>;

	/// Returns a list of control IDs that this camera has.
	///
	/// # Errors
	///
	/// If the backend cannot read its controls or panics while reading them.
	fn get_all_supported_controls(
		&self
	) -> EdgeCamerasResult<Vec<KnownCameraControl>>;

	/// Returns a full map of this camera's controls.
	///
	/// # Errors
	///
	/// If the backend cannot read its controls or panics while reading them.
	fn get_controls(&self) -> EdgeCamerasResult<ControlMap>;

	/// Returns a camera control by ID.
	///
	/// # Errors
	///
	/// If the control ID could not be found on this camera or the backend
	/// panics while reading it.
	fn get_control(
		&self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult<CamControl>;

	/// Update a camera control by ID.
	///
	/// # Errors
	///
	/// If the control is unavailable, the value is invalid, the backend rejects
	/// the operation, or the backend panics while setting the control.
	fn set_control(
		&mut self,
		control: CamControl,
	) -> EdgeCamerasResult;

	/// Update multiple of camera controls.
	///
	/// This operation is not atomic.
	///
	/// # Errors
	///
	/// If a control is unavailable, a value is invalid, the backend rejects an
	/// operation, or the backend panics while setting a control.
	fn set_controls(
		&mut self,
		controls: ControlMap,
	) -> EdgeCamerasResult;

	/// Revert a camera control to its default value by ID.
	///
	/// # Errors
	///
	/// If the control is unavailable, has no default, its default is invalid,
	/// or the nokhwa backend rejects the operation or panics.
	fn set_control_to_default(
		&mut self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult;

	/// Revert all camera controls to default. Controls without a reported
	/// default are left unchanged. Automatic controls are temporarily switched
	/// to manual when required to reset their dependent controls.
	///
	/// # Errors
	///
	/// If controls cannot be read, a dependent control cannot be switched, or
	/// the backend rejects an operation or panics.
	fn reset_controls_to_default(&mut self) -> EdgeCamerasResult;
}

#[cfg(test)]
mod tests {
	use std::{
		cell::RefCell,
		rc::Rc,
	};

	use nokhwa::utils::ControlValueSetter;

	use super::*;
	use crate::{
		controls::{
			descriptions::IntegerControl,
			values::IntegerControlValue,
		},
		error::EdgeCamerasError,
	};

	type Calls = Vec<(KnownCameraControl, ControlValueSetter)>;

	struct FakeCamera {
		info: CameraInfo,
		set_calls: Rc<RefCell<Calls>>,
	}

	impl FakeCamera {
		fn new() -> (Self, Rc<RefCell<Calls>>) {
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

		fn get_frame_buffer(&mut self) -> EdgeCamerasResult<FrameBuffer> {
			Err(EdgeCamerasError::General(
				"fake camera does not provide frames".into(),
			))
		}

		fn get_all_supported_controls(
			&self
		) -> EdgeCamerasResult<Vec<KnownCameraControl>> {
			Ok(Vec::new())
		}

		fn get_controls(&self) -> EdgeCamerasResult<ControlMap> {
			Ok(ControlMap::empty())
		}

		fn get_control(
			&self,
			id: KnownCameraControl,
		) -> EdgeCamerasResult<CamControl> {
			Err(EdgeCamerasError::GetControl(
				id,
				String::from("fake camera has no controls"),
			))
		}

		fn set_control(
			&mut self,
			control: CamControl,
		) -> EdgeCamerasResult {
			let id = control.control_id()?;
			let value = control.control_value_setter(id)?;
			self.set_calls.borrow_mut().push((id, value));
			Ok(())
		}

		fn set_controls(
			&mut self,
			_controls: ControlMap,
		) -> EdgeCamerasResult {
			Ok(())
		}

		fn set_control_to_default(
			&mut self,
			_id: KnownCameraControl,
		) -> EdgeCamerasResult {
			Ok(())
		}

		fn reset_controls_to_default(&mut self) -> EdgeCamerasResult {
			Ok(())
		}
	}

	#[test]
	fn camera_trait_is_testable_without_a_device() {
		let (fake_camera, calls) = FakeCamera::new();
		let mut camera: Box<dyn CameraLike> = Box::new(fake_camera);
		let id = KnownCameraControl::Brightness;
		let control = CamControl::Integer(IntegerControl::new(
			id,
			IntegerControlValue::from(18),
			0,
			1,
			Vec::new(),
		));

		let value = control
			.control_value_setter(id)
			.expect("control should map to setter");

		camera.set_control(control).unwrap();

		let recorded_calls = calls.borrow();
		assert_eq!(recorded_calls.len(), 1);
		assert_eq!(recorded_calls[0].0, KnownCameraControl::Brightness);
		assert_eq!(recorded_calls[0].1, value);
	}
}
