//! The core components of the Edge system.

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
		clippy::indexing_slicing,
		reason = "Tests are allowed to perform all of these operations"
	)
)]

pub mod error;
pub mod image;
pub mod logger;
pub mod settings;
