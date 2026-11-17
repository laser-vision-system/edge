//! An abstraction over the camera device for test fixtures to use.

use edge_core::image::FrameBuffer;
use nokhwa::utils::{
	CameraIndex,
	CameraInfo,
	ControlValueSetter,
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
	/// If this camera has no controls.
	fn get_all_supported_controls(
		&self
	) -> EdgeCamerasResult<Vec<KnownCameraControl>>;

	/// Returns a full map of this camera's controls.
	///
	/// # Errors
	///
	/// If this camera has no controls.
	fn get_controls(&self) -> EdgeCamerasResult<ControlMap>;

	/// Returns a camera control by ID.
	///
	/// # Errors
	///
	/// If the control id could not be found on this camera.
	fn get_control(
		&self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult<CamControl>;

	/// Update a camera control by ID.
	///
	/// # Errors
	///
	/// If attempting to set the controls to an invalid value.
	fn set_control(
		&mut self,
		id: KnownCameraControl,
		value: ControlValueSetter,
	) -> EdgeCamerasResult;

	/// Update multiple of camera controls.
	///
	/// # Errors
	///
	/// If attempting to set the controls to an invalid value.
	fn set_controls(
		&mut self,
		controls: ControlMap,
	) -> EdgeCamerasResult;
}

#[cfg(test)]
mod tests {
	use std::{
		cell::RefCell,
		rc::Rc,
	};

	use super::*;
	use crate::error::EdgeCamerasError;

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
			Ok(ControlMap::from(Vec::new()))
		}

		fn get_control(
			&self,
			id: KnownCameraControl,
		) -> EdgeCamerasResult<CamControl> {
			Err(EdgeCamerasError::GetControl(id))
		}

		fn set_control(
			&mut self,
			id: KnownCameraControl,
			value: ControlValueSetter,
		) -> EdgeCamerasResult {
			self.set_calls.borrow_mut().push((id, value));
			Ok(())
		}

		fn set_controls(
			&mut self,
			_controls: ControlMap,
		) -> EdgeCamerasResult {
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
}
