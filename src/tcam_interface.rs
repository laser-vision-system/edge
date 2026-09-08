//! Provides connection, identification, management, and configuration for
//! camera devices.

mod camera;
pub mod controls;
mod identifier;
mod metadata;
mod new_camera_info;
mod persistence;
mod status;

pub use camera::Camera;
pub(crate) use identifier::Identifier;
pub(crate) use metadata::Metadata;
pub(crate) use new_camera_info::NewCameraInfo;
pub(crate) use persistence::Persistence;
pub(crate) use status::Status;
