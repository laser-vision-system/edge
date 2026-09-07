//! Provides connection, identification, management, and configuration for camera devices.

mod camera;
pub use camera::Camera;

mod identifier;
pub(crate) use identifier::Identifier;

mod status;
pub(crate) use status::Status;

mod metadata;
pub(crate) use metadata::Metadata;

mod persistence;
pub(crate) use persistence::Persistence;

mod new_camera_info;
pub(crate) use new_camera_info::NewCameraInfo;

pub mod controls;

mod setpoint;
pub(crate) use setpoint::Setpoint;