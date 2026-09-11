//! Edge computing system for laser measurement on a μ-com.

mod auto_connect;
mod client;
mod conductor;
mod error;
mod logger;
mod network;
mod settings;
mod websocket;

// Used by rerun example.
pub mod tcam_interface;
pub mod vision_processor;

pub use conductor::run;
