//! Edge computing system for laser measurement on a μ-com.

#![cfg_attr(feature = "cnn-backend-wgpu", recursion_limit = "256")]

mod conductor;
mod logger;
mod network;
mod settings;

// Used by rerun example or dataset assistant.
pub mod error;
pub mod tcam_interface;
pub mod vision_processor;

pub use conductor::run;
