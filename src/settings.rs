//! # Settings
//!
//! TODO: Replace constants with values from environment variables.

use std::time::Duration;

// ============================================================================
// Only constants that are necessary for connecting to the server should be
// defined here.
// ============================================================================

pub const API: &str = "/edge_ws";
pub const HOST: &str = "127.0.0.1";
pub const PORT: &str = "8080";

// ============================================================================
// All other settings values should be defined within this struct. ==
// ============================================================================

/// System-level default settings.
///
/// This will be filled in by the initial connection to the server.
#[derive(Clone, Debug, Default)]
pub struct Settings {
	pub update_delay: Duration,
	pub camera_err_decrement: u16,
	pub camera_err_increment: u16,
	pub camera_err_threshold: u16,
}
