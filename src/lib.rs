pub mod client;
pub mod conductor;
pub mod settings;
pub mod tcam_interface;
pub mod vision_processor;

// Re-exporting for easy access.
pub use tcam_interface::camera::Camera;
pub use vision_processor::Demonstration;
