//! # Settings
//!
//! TODO: Replace constants with values from environment variables.

#![expect(
	clippy::print_stderr,
	reason = "Logger can not be initialised until this module is initialised, so this is expected behaviour."
)]
// TODO: Resolve this.
#![allow(
	unused,
	reason = "Temporarily allow these until environment variables are implemented"
)]

use std::{fmt::Display, str::FromStr, time::Duration};

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

pub use server_edge_datatypes::edge::Settings;

use std::env;

use crate::error::EdgeError;

/// Contains all environment variables that are used to configure the application.
pub struct EnvVariables {
	// Verbosity level for logging and debugging:
	// - 0: No logs
	// - 1: Error logs only
	// - 2: Warning and error logs
	// - 3: Info, warning, and error logs
	// Any other strings will be treated as 1.
	verbosity: u8,
}

impl EnvVariables {
	pub fn initialise() -> Self {
		let verbosity = match Self::get_env_variable::<u8>("VERBOSITY") {
			Ok(v) => v,
			Err(e) => {
				eprintln!("{}", e);
				1
			}, // Default to error logs only if the variable is not set or invalid
		};

		Self { verbosity }
	}

	fn get_env_variable<T>(name: &str) -> Result<T, EdgeError>
	where
		T: FromStr,
		T::Err: Display,
	{
		match env::var(name) {
			Ok(value) => match value.parse::<T>() {
				Ok(parsed_value) => Ok(parsed_value),
				Err(e) => Err(EdgeError::General {
					reason: format!(
						"Could not parse {} environment variable to the expected type: {}",
						name, e
					),
				}),
			},
			Err(e) => Err(EdgeError::General {
				reason: format!(
					"Error reading {} environment variable: {}",
					name, e
				),
			}),
		}
	}

	// fn get_verbosity() -> Result<u8, EdgeError> {
	// 	match env::var("VERBOSITY") {
	// 		Ok(v) => match v.parse::<u8>() {
	// 			Ok(v) => Ok(v),
	// 			Err(e) => Err(EdgeError::General {
	// 				reason: format!(
	// 					"Could not parse VERBOSITY environment variable to u8: {}",
	// 					e
	// 				),
	// 			}),
	// 		},
	// 		Err(e) => {
	// 			Err(EdgeError::General {
	// 				reason: format!(
	// 					"Error reading VERBOSITY environment variable: {}",
	// 					e
	// 				),
	// 			})
	// 		},
	// 	}
	// }

	pub fn verbosity(&self) -> u8 {
		self.verbosity
	}
}

impl Default for EnvVariables {
	fn default() -> Self {
		EnvVariables {
			verbosity: 1, // Default to error logs only
		}
	}
}
