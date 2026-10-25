//! Edge computing system for laser measurement on a μ-com.

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

// Verify that only one backend feature flag is enabled at once.
mutually_exclusive_features::exactly_one_of!(
	"cnn-backend-flex",
	"cnn-backend-cuda",
	"cnn-backend-wgpu"
);

// Verify that only one model loading feature flag is enabled at once.
mutually_exclusive_features::exactly_one_of!(
	"cnn-model-embedded",
	"cnn-model-file",
	"cnn-model-zero-copy"
);

mod conductor;

// Used by rerun example or dataset assistant.
pub mod error;

pub use conductor::run;
