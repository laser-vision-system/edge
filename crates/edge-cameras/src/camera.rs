//! Camera device related operations.

mod device;
pub mod diagnostics;
mod interface;

use std::{
	cmp::Ordering,
	panic::{
		AssertUnwindSafe,
		catch_unwind,
	},
};

pub use self::{
	device::*,
	interface::CameraLike,
};
use crate::error::{
	EdgeCamerasError,
	EdgeCamerasResult,
};

/// Query [`nokhwa`] for all available cameras and sort them by index.
///
/// # Panics
///
/// If no Nokhwa backend is available.
#[expect(clippy::expect_used, reason = "Low priority edge case")]
pub fn query_for_available_cameras() -> Vec<CameraInfo> {
	let mut cameras = nokhwa::query(nokhwa::utils::ApiBackend::Auto)
		.expect("The backend should be supported");
	cameras.sort_by(|left, right| -> Ordering {
		let left_index = left.index().as_index().unwrap_or(u32::MAX);
		let right_index = right.index().as_index().unwrap_or(u32::MAX);
		left_index.cmp(&right_index)
	});
	cameras
}

/// Run a backend control operation and translate unwinding panics to errors.
pub(crate) fn run_backend_operation<T>(
	operation: &'static str,
	action: impl FnOnce() -> EdgeCamerasResult<T>,
) -> EdgeCamerasResult<T> {
	match catch_unwind(AssertUnwindSafe(action)) {
		Ok(result) => result,
		Err(payload) => {
			let message = payload
				.downcast_ref::<String>()
				.cloned()
				.or_else(|| {
					payload
						.downcast_ref::<&str>()
						.map(|message| (*message).to_owned())
				})
				.unwrap_or_else(|| "non-string panic payload".into());
			Err(EdgeCamerasError::ControlOperationPanicked {
				operation,
				message,
			})
		},
	}
}
