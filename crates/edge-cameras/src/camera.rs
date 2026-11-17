//! Camera device related operations.

mod device;
mod interface;

pub use self::{
	device::*,
	interface::CameraLike,
};
