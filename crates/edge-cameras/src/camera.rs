//! Camera device related operations.

mod device;
pub mod diagnostics;
mod interface;

use std::panic::{
	AssertUnwindSafe,
	catch_unwind,
};

pub use self::{
	device::*,
	interface::CameraLike,
};
use crate::error::{
	EdgeCamerasError,
	EdgeCamerasResult,
};

/// Run a backend control operation and translate unwinding panics to errors.
fn run_backend_operation<T>(
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
