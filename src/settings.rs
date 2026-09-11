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

pub const RETRY_DELAY: Duration = Duration::from_millis(1000);

// ============================================================================
// All other settings values should be defined within this struct.
// ============================================================================

pub use server_edge_datatypes::edge::Settings;
