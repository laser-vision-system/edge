//! Provides connection, identification, management, and configuration for
//! camera devices.

mod camera;
pub mod controls;
mod identifier;
mod metadata;
mod persistence;

pub use camera::Camera;
pub use identifier::Identifier;
pub use metadata::Metadata;
pub use persistence::Persistence;
use server_edge_datatypes::shared::CameraId;

#[derive(Clone, Debug)]
pub enum WhyNo {
	Disconnected(CameraId),
	Broken(CameraId),
}
