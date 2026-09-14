//! Provides connection, identification, management, and configuration for
//! camera devices.

mod camera_device;
pub mod controls;
mod identifier;
mod perpetual_info;
mod persistence;

pub use camera_device::CameraDevice;
pub use identifier::Identifier;
pub use perpetual_info::PerpetualInfo;
pub use persistence::Persistence;
use server_edge_datatypes::shared::CameraId;

#[derive(Clone, Debug)]
pub enum WhyNo {
	Disconnected(CameraId),
	Broken(CameraId),
}
