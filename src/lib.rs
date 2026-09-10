//! Edge computing system for laser measurement on a μ-com.

mod client;
mod conductor;
mod error;
mod settings;

// Used by rerun example.
pub mod tcam_interface;
pub mod vision_processor;
pub mod logger;

pub mod auto_connect; 

pub use conductor::run;
