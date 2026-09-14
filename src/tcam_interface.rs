//! Provides connection, identification, management, and configuration for
//! camera devices.

mod camera_device;
pub mod controls;
mod identifier;
mod persistence;

pub use camera_device::CameraDevice;
pub use identifier::Identifier;
pub use persistence::{PerpetualInfo, Persistence};
use server_edge_datatypes::shared::CameraId;

#[derive(Clone, Debug)]
pub enum WhyNo {
	Disconnected(CameraId),
	Broken(CameraId),
}
