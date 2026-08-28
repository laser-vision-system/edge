//! Provides connection, identification, management, and configuration for camera devices.

mod camera;
pub use camera::Camera;

mod identifier;
pub use identifier::Identifier;

mod status;
pub use status::Status;

mod metadata;
pub use metadata::Metadata;

mod persistence;
pub use persistence::Persistence;

pub mod controls;
