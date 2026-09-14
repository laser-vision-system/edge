//! Provides connection, identification, management, and configuration for
//! camera devices.

mod camera_device;
pub mod controls;
mod identifier;
mod metadata;
mod persistence;

pub use camera_device::CameraDevice;
pub use identifier::Identifier;
pub use metadata::Metadata;
pub use persistence::Persistence;
use server_edge_datatypes::shared::CameraId;

#[derive(Clone, Debug)]
pub enum WhyNo {
	Disconnected(CameraId),
	Broken(CameraId),
}
