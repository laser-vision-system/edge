//! # Settings
//!
//! A collection of constant variables.
//!
//! TODO: The ability to change settings with environment variables.
use std::time::Duration;

pub const UPDATE_DELAY: Duration = Duration::from_millis(1000);
pub const PRINT_RESULTS: bool = true;

pub const HOST: &str = "127.0.0.1";
pub const PORT: &str = "8080";
pub const API: &str = "/edge_ws";
pub const ENDPOINT_CAMERA_READINGS: &str = "camera_info";
pub const ENDPOINT_CAMERA_CONFIG: &str = "configuration";
pub const ENDPOINT_GUI_UPDATE: &str = "gui_update";
pub const ENDPOINT_IMAGE: &str = "image";
