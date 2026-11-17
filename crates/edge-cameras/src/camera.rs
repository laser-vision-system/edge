//! A physical camera device, as made available by [`nokhwa`].
//!
//! Also provides [`CameraLike`] to abstract implemention details away.

mod device;
mod interface;

pub use self::{
	device::{
		CameraDevice,
		query_for_available_cameras,
	},
	interface::CameraLike,
};
