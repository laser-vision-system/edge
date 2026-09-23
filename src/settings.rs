//! # Settings
//!
//! TODO: Replace constants with values from environment variables.

// TODO: Resolve this.
#![allow(
	unused,
	reason = "Temporarily allow these until environment variables are implemented"
)]

use std::time::Duration;

// ============================================================================
// Only constants that are necessary for connecting to the server should be
// defined here.
// ============================================================================

pub const API: &str = "/edge_ws";
pub const HOST: &str = "127.0.0.1";
pub const PORT: &str = "8080";

pub const RETRY_DELAY: Duration = Duration::from_secs(1);
pub const AUTO_CONNECT_DELAY: Duration = Duration::from_secs(5);

/// How many buffers are used for camera device input.
///
/// High number increases latency and memory usage.
pub const CAMERA_BUFFER_COUNT: u32 = 2;

// ============================================================================
// All other settings values should be defined within this struct.
// ============================================================================

pub use server_edge_datatypes::edge::Settings;
