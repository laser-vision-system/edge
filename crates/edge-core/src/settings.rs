//! # Settings

use std::{
	fmt::Display,
	str::FromStr,
	time::Duration,
};

// ============================================================================
// Only constants that are necessary for connecting to the server should be
// defined here.
// ============================================================================

pub const API: &str = "/edge_ws";
pub const HOST: &str = "127.0.0.1";
pub const PORT: &str = "8080";

pub const RETRY_DELAY: Duration = Duration::from_secs(1);
pub const AUTO_CONNECT_DELAY: Duration = Duration::from_secs(5);

// ============================================================================
// All other settings values should be defined within this struct.
// ============================================================================

// Staging area, move to Settings struct.

/// How many buffers are used for camera device input.
///
/// High number increases latency and memory usage.
pub const CAMERA_BUFFER_COUNT: u32 = 1;

pub const IMAGE_WIDTH: u32 = 640;
pub const IMAGE_HEIGHT: u32 = 480;
pub const IMAGE_PIXELS: usize = (IMAGE_WIDTH * IMAGE_HEIGHT) as usize;

use std::env;

pub use api::edge::Settings;

use crate::error::EdgeCoreError;

/// Retrieve an environemant variable and attempt to parse it.
///
/// # Errors
/// - If the environment variable cannot be retrieved.
/// - If the retrieved environment variable string cannot be parsed into the
///   provided type.
pub fn retrieve_env_variable<T>(name: &str) -> Result<T, EdgeCoreError>
where
	T: FromStr,
	T::Err: Display,
{
	match env::var(name) {
		Ok(value) => {
			match value.parse::<T>() {
				Ok(parsed_value) => Ok(parsed_value),
				Err(why) => {
					Err(EdgeCoreError::EnvVarParse {
						name: name.to_owned(),
						why: why.to_string(),
					})
				},
			}
		},
		Err(why) => {
			Err(EdgeCoreError::EnvVar {
				name: name.to_owned(),
				source: why,
			})
		},
	}
}
