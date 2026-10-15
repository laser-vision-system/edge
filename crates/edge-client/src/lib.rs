//! Shared network utilities.

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

pub mod auto_connect;
pub mod error;
pub mod persistent_edge_id;
pub mod websocket;
mod client;
mod commands;
pub mod client_handler;
mod client_interface;
pub mod camera_requests;
