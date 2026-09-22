//! Provides connection, identification, management, and configuration for
//! camera devices.

mod camera_device;
pub mod controls;
mod identifier;
mod persistence;

pub use camera_device::CameraDevice;
pub use identifier::Identifier;
pub use persistence::{
	MaybeBatch, MaybeImage, PerpetualInfo, PersistEntry, Persistence, WhyNo,
	polling,
};
