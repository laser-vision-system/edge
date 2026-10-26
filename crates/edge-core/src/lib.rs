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

/// Helper function for writing optional values for display.
///
/// # Errors
///
/// If the formatter fails to write to the standard out.
pub fn write_optional<V>(
	fmtr: &mut std::fmt::Formatter<'_>,
	value: &Option<V>,
) -> std::fmt::Result
where
	V: std::fmt::Display,
{
	match value {
		Some(value) => value.fmt(fmtr),
		None => fmtr.write_str("None"),
	}
}
