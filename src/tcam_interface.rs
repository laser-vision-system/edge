//! Provides connection, identification, management, and configuration for camera devices.

mod camera;
pub use camera::Camera;

mod identifier;
pub use identifier::Identifier;

pub mod controls;
