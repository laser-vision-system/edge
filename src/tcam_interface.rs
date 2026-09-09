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
pub use identifier::Identifier;
pub use metadata::Metadata;
pub use new_camera_info::NewCameraInfo;
pub use persistence::Persistence;
pub use status::Status;
