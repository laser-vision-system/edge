//! Physical device control actions.

use edge_core::{
	image::FrameBuffer,
	logger::logger,
};
use nokhwa::utils::{
	CameraIndex,
	CameraInfo,
	ControlValueSetter,
	KnownCameraControl,
};

use super::{
	super::{
		CameraLike,
		run_backend_operation,
	},
	CameraDevice,
};
use crate::{
	controls::{
		ControlMap,
		automatic::{
			dependent_auto_control,
			manual_mode_value,
		},
		descriptions::CamControl,
	},
	error::{
		EdgeCamerasError,
		EdgeCamerasResult,
	},
};

impl CameraDevice {
	/// Query the backend device for controls, mapping the type and error.
	///
	/// # Errors
	///
	/// If the device fails to report controls or panics.
	fn read_controls(&self) -> EdgeCamerasResult<ControlMap> {
		run_backend_operation("read controls", || {
			self.inner
				.controls()
				.map_err(EdgeCamerasError::CannotGetControls)
				.map(ControlMap::from)
		})
	}

	/// Apply a control value at an id.
	///
	/// # Errors
	///
	/// If attempting to set the control to an invalid value or the backend
	/// rejects the operation.
	fn apply_control(
		&mut self,
		id: KnownCameraControl,
		value: ControlValueSetter,
	) -> EdgeCamerasResult {
		let controls = self.read_controls()?;
		if let Some(control) = controls.get(&id)
			&& !control.is_writable()
		{
			return Err(EdgeCamerasError::ControlUnwritable(id));
		}
		let value_string = value.to_string();
		run_backend_operation("set control", || {
			self.inner.set_control(id, value).map_err(|source| {
				EdgeCamerasError::SetControl {
					id,
					value: value_string,
					source,
				}
			})
		})
	}

	/// Reset a single control description and apply its value to the backend.
	///
	/// # Errors
	///
	/// If the control has no default value, the default value is invalid for
	/// this control, or this control has no description.
	fn apply_control_default(
		&mut self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult {
		let mut control = self.get_control(id)?;
		control.reset_to_default()?;
		self.set_control(control)
	}

	/// Switch the requested automatic control to manual/off when its value is
	/// recognised by the reported control description.
	///
	/// # Errors
	///
	/// Returns an error if the control has an unexpected type or cannot be set.
	pub fn set_auto_control_manual(
		&mut self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult<Option<ControlValueSetter>> {
		let control = match self.get_control(id) {
			Ok(control) => control,
			Err(error @ EdgeCamerasError::GetControl(..)) => {
				logger().log_error(&error);
				return Ok(None);
			},
			Err(error) => return Err(error),
		};
		let previous = control
			.control_value_setter(id)
			.map_err(|_none| EdgeCamerasError::ManualModeUnavailable(id))?;
		let manual = manual_mode_value(id, &control)
			.ok_or(EdgeCamerasError::ManualModeUnavailable(id))?;
		self.apply_control(id, manual)?;
		Ok(Some(previous))
	}

	/// Restore a temporarily changed dependency after a control operation.
	///
	/// E.g. `ExposureAbsolute` depends on `AutoExposure`.
	///
	/// # Errors
	///
	/// Returns the operation error, or a restoration error if the operation
	/// itself succeeded.
	fn restore_dependency<T>(
		&mut self,
		id: KnownCameraControl,
		previous: Option<ControlValueSetter>,
		result: EdgeCamerasResult<T>,
	) -> EdgeCamerasResult<T> {
		let Some(previous) = previous else {
			return result;
		};
		let restore = self.apply_control(id, previous);
		match (result, restore) {
			(Err(error), _) | (Ok(_), Err(error)) => Err(error),
			(Ok(value), Ok(())) => Ok(value),
		}
	}

	/// Run an operation with auto exposure switched to manual.
	///
	/// # Errors
	///
	/// Returns an error if the dependency transition or operation fails.
	fn with_manual_exposure<T>(
		&mut self,
		operation: impl FnOnce(&mut Self) -> EdgeCamerasResult<T>,
	) -> EdgeCamerasResult<T> {
		let previous =
			self.set_auto_control_manual(KnownCameraControl::AutoExposure)?;
		let result = operation(self);
		self.restore_dependency(
			KnownCameraControl::AutoExposure,
			previous,
			result,
		)
	}

	/// Run an operation with auto white balance switched off.
	///
	/// # Errors
	///
	/// Returns an error if the dependency transition or operation fails.
	fn with_auto_white_balance_disabled<T>(
		&mut self,
		operation: impl FnOnce(&mut Self) -> EdgeCamerasResult<T>,
	) -> EdgeCamerasResult<T> {
		let previous =
			self.set_auto_control_manual(KnownCameraControl::AutoWhiteBalance)?;
		let result = operation(self);
		self.restore_dependency(
			KnownCameraControl::AutoWhiteBalance,
			previous,
			result,
		)
	}

	/// Reset one control, managing any automatic-control dependency.
	///
	/// # Errors
	///
	/// Returns an error if the control has no default or a backend operation
	/// fails.
	fn reset_one_control(
		&mut self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult {
		self.apply_control_default(id)
	}

	/// Set a control that depends on an automatic control mode.
	///
	/// If setting the control directly fails because the control is unavailable
	/// or rejected, this temporarily transitions the governing automatic
	/// control to manual mode before applying the value and restoring the
	/// previous automatic state.
	///
	/// # Errors
	///
	/// If the value is invalid, the manual transition fails, or the backend
	/// rejects setting the control.
	fn set_dependent_control(
		&mut self,
		id: KnownCameraControl,
		auto_id: KnownCameraControl,
		value: ControlValueSetter,
	) -> EdgeCamerasResult {
		match self.apply_control(id, value.clone()) {
			Ok(()) => Ok(()),
			Err(
				error @ (EdgeCamerasError::SetControl {
					..
				}
				| EdgeCamerasError::ControlUnwritable(_)),
			) => {
				let result = match auto_id {
					KnownCameraControl::AutoExposure => {
						self.with_manual_exposure(|camera| {
							camera.apply_control(id, value)
						})
					},
					KnownCameraControl::AutoWhiteBalance => {
						self.with_auto_white_balance_disabled(|camera| {
							camera.apply_control(id, value)
						})
					},
					_ => return Err(error),
				};
				match result {
					Err(
						EdgeCamerasError::ManualModeUnavailable(_)
						| EdgeCamerasError::SetControlType(_),
					) => Err(error),
					result => result,
				}
			},
			Err(error) => Err(error),
		}
	}

	/// Reset a control to its default value if a default is available.
	///
	/// Controls without a reported default or description are ignored.
	///
	/// # Errors
	///
	/// If resetting the control fails due to a backend error.
	fn reset_if_default_available(
		&mut self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult {
		match self.reset_one_control(id) {
			Err(
				EdgeCamerasError::ControlDefaultUnavailable(_)
				| EdgeCamerasError::NoControlDescription,
			) => Ok(()),
			result => result,
		}
	}
}

impl CameraLike for CameraDevice {
	fn get_info(&self) -> &CameraInfo {
		self.inner.info()
	}

	fn get_index(&self) -> &CameraIndex {
		self.inner.info().index()
	}

	fn get_frame_buffer(&mut self) -> EdgeCamerasResult<FrameBuffer> {
		let frame = self.inner.frame().map_err(EdgeCamerasError::FrameRead)?;
		Ok(FrameBuffer::from(frame))
	}

	fn get_all_supported_controls(
		&self
	) -> EdgeCamerasResult<Vec<KnownCameraControl>> {
		Ok(self.read_controls()?.into_inner().into_keys().collect())
	}

	fn get_controls(&self) -> EdgeCamerasResult<ControlMap> {
		self.read_controls()
	}

	fn get_control(
		&self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult<CamControl> {
		let controls = self.read_controls()?;
		let control =
			controls
				.get(&id)
				.cloned()
				.ok_or(EdgeCamerasError::GetControl(
					id,
					String::from("control not found"),
				))?;
		if !control.is_readable() {
			return Err(EdgeCamerasError::GetControl(
				id,
				String::from("control not readable"),
			));
		}
		Ok(control)
	}

	fn set_control(
		&mut self,
		control: CamControl,
	) -> EdgeCamerasResult {
		let id = control.control_id()?;
		let value = control.control_value_setter(id)?;
		match dependent_auto_control(id) {
			Some(auto_id) => self.set_dependent_control(id, auto_id, value),
			None => self.apply_control(id, value),
		}
	}

	fn set_controls(
		&mut self,
		controls: ControlMap,
	) -> EdgeCamerasResult {
		run_backend_operation("set controls", || {
			controls
				.into_inner()
				.into_values()
				.try_for_each(|control| self.set_control(control))
		})
	}

	fn set_control_to_default(
		&mut self,
		id: KnownCameraControl,
	) -> EdgeCamerasResult {
		run_backend_operation("reset control to default", || {
			self.reset_one_control(id)
		})
	}

	fn reset_controls_to_default(&mut self) -> EdgeCamerasResult {
		run_backend_operation("reset controls to default", || {
			let ids = self.get_all_supported_controls()?;
			let dependent = [
				KnownCameraControl::ExposureAbsolute,
				KnownCameraControl::WhiteBalance,
				KnownCameraControl::AutoExposure,
				KnownCameraControl::AutoWhiteBalance,
			];
			for id in ids.iter().copied().filter(|id| !dependent.contains(id)) {
				self.reset_if_default_available(id)?;
			}
			for id in [
				KnownCameraControl::ExposureAbsolute,
				KnownCameraControl::WhiteBalance,
				KnownCameraControl::AutoExposure,
				KnownCameraControl::AutoWhiteBalance,
			] {
				if ids.contains(&id) {
					self.reset_if_default_available(id)?;
				}
			}
			Ok(())
		})
	}
}
