//! Edge computing system for laser measurement on a μ-com.

#![cfg_attr(feature = "cnn-backend-wgpu", recursion_limit = "256")]
#![cfg_attr(
	test,
	allow(
		clippy::print_stdout,
		clippy::print_stderr,
		clippy::use_debug,
		clippy::panic,
		clippy::unwrap_used,
		clippy::unwrap_in_result,
		clippy::expect_used,
		reason = "Tests are allowed to perform all of these operations"
	)
)]

mod conductor;
mod logger;
mod network;
mod settings;

// Used by rerun example or dataset assistant.
pub mod error;
pub mod tcam_interface;
pub mod vision_processor;

pub use conductor::run;
use mutually_exclusive_features::exactly_one_of;

// Verify that only one backend feature flag is enabled at once.
exactly_one_of!("cnn-backend-flex", "cnn-backend-cuda", "cnn-backend-wgpu");
