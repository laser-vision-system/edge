use std::collections::HashMap;

use nokhwa::utils::{CameraControl, KnownCameraControl};

use super::CamControl;

/// Convert a collection of [`nokhwa`] [`CameraControl`] into a custom type.
pub fn extract_controls(
	nokhwa_controls: Vec<CameraControl>,
) -> HashMap<KnownCameraControl, CamControl> {
	nokhwa_controls
		.into_iter()
		.map(|control| (control.control(), control.description().into()))
		.collect::<HashMap<KnownCameraControl, CamControl>>()
}

/// Convert a single [`nokhwa`] [`CameraControl`] into a custom type.
pub fn extract_control(nokhwa_control: CameraControl) -> CamControl {
	nokhwa_control.description().into()
}
