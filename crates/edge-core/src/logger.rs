//! # Logger
//!
//! Provides logging support for the edge.
//!
//! This module can only be accessed by calling importing the `logger()`
//! function.
//!
//! To create a log, call one of the following functions:
//!
//! - `logger().log_info(message: &str)`
//! - `logger().log_warn(message: &str)`
//! - `logger().log_error(message: &str, error: EdgeError)`
//!
//! To send all buffered logs to the server, use:
//!
//! - `logger().send_logs()`

#![expect(
	clippy::print_stdout,
	clippy::print_stderr,
	reason = "This is the logger module, where this is expected behaviour"
)]

use std::{
	cell::RefCell,
	fmt::Display,
	mem,
	sync::{
		Mutex,
		OnceLock,
	},
};

use chrono::{
	Local,
	NaiveDate,
};
use serde::{
	Deserialize,
	Serialize,
};

use crate::settings;

// Declare a type alias for easy imports
pub type LoggerRef = &'static LoggerGuard;

static LOGGER: OnceLock<LoggerGuard> = OnceLock::new();

pub fn logger() -> LoggerRef {
	LOGGER.get_or_init(LoggerGuard::new)
}

pub struct LoggerGuard {
	inner: Mutex<RefCell<Logger>>,
}

impl LoggerGuard {
	fn new() -> Self {
		Self {
			inner: Mutex::new(RefCell::new(Logger::new())),
		}
	}

	pub fn log_info(
		&self,
		message: &str,
	) {
		let guard = match self.inner.lock() {
			Ok(cell) => cell,
			Err(why) => {
				eprintln!("Logger was poisoned!");
				why.into_inner()
			},
		};
		let mut logger = guard.borrow_mut();

		logger.log_info(message);
	}

	pub fn log_warn(
		&self,
		message: &str,
	) {
		let guard = match self.inner.lock() {
			Ok(cell) => cell,
			Err(why) => {
				eprintln!("Logger was poisoned!");
				why.into_inner()
			},
		};
		let mut logger = guard.borrow_mut();

		logger.log_warn(message);
	}

	pub fn log_error<T>(
		&self,
		error: &T,
	) where
		T: Display + ?Sized,
	{
		let guard = match self.inner.lock() {
			Ok(cell) => cell,
			Err(why) => {
				eprintln!("Logger was poisoned!");
				why.into_inner()
			},
		};
		let mut logger = guard.borrow_mut();

		logger.log_error(error);
	}

	pub fn send_logs(&self) {
		let guard = match self.inner.lock() {
			Ok(cell) => cell,
			Err(why) => {
				eprintln!("Logger was poisoned!");
				why.into_inner()
			},
		};
		let mut logger = guard.borrow_mut();

		logger.send_logs();
	}
}

#[derive(Debug, Serialize, Deserialize)]
enum LogLevel {
	Info,
	Warn,
	Error,
}

/// The logger struct that holds each unsent (buffered) log entry and the log
/// count.
///
/// Each log entry is a string, stored in a `Vec<String>`.
///
/// All log entries are consumed upon calling the `send_logs` function.
///
/// ### Verbosity Levels
/// The logger supports different verbosity levels, which can be set via the
/// `VERBOSITY` environment variable. The levels are as follows:
/// - 0: No logs are recorded or sent.
/// - 1: Only error logs are recorded and sent.
/// - 2: Warning and error logs are recorded and sent.
/// - 3: Info, warning, and error logs are recorded and sent.
///
/// Any other inputs including non-numeric strings will default to level 1
/// (error logs only).
pub struct Logger {
	logs: Vec<String>,
	counter: LogCounter,
	verbosity: u8,
}

impl Logger {
	fn new() -> Self {
		let (logs, counter, verbosity) =
			match settings::retrieve_env_variable::<u8>("VERBOSITY") {
				Ok(verbosity) => {
					let logs = Vec::new();
					let counter = LogCounter::new();
					(logs, counter, verbosity)
				},
				Err(why) => {
					let mut logs = Vec::new();
					let mut counter = LogCounter::new();
					let log_entry = format!(
						"[{:05}] {}\n{:?}\n{}\n----------",
						counter.next(),
						Local::now().to_rfc3339(),
						LogLevel::Error,
						format!(
							"Could not read verbosity: {why}. Defaulting to 1."
						),
					);
					logs.push(log_entry);
					let verbosity = 1;
					(logs, counter, verbosity)
				},
			};
		Logger {
			logs,
			counter,
			verbosity,
		}
	}

	pub fn log_info(
		&mut self,
		message: &str,
	) {
		if self.verbosity < 3 {
			return;
		}
		let log_entry = format!(
			"[{:05}] {}\n{:?}\n{}\n----------",
			self.counter.next(),
			Local::now().to_rfc3339(),
			LogLevel::Info,
			message,
		);

		#[cfg(debug_assertions)]
		println!("{log_entry}");

		self.logs.push(log_entry);
	}

	pub fn log_warn(
		&mut self,
		message: &str,
	) {
		if self.verbosity < 2 {
			return;
		}
		let log_entry = format!(
			"[{:05}] {}\n{:?}\n{}\n----------",
			self.counter.next(),
			Local::now().to_rfc3339(),
			LogLevel::Warn,
			message,
		);

		#[cfg(debug_assertions)]
		println!("{log_entry}");

		self.logs.push(log_entry);
	}

	// HACK: Temporary workaround while restructuring the project.
	pub fn log_error<T>(
		&mut self,
		error: &T,
	) where
		T: Display + ?Sized,
	{
		if self.verbosity < 1 {
			return;
		}
		let log_entry = format!(
			"[{:05}] {}\n{:?}\n{}\n----------",
			self.counter.next(),
			Local::now().to_rfc3339(),
			LogLevel::Error,
			error,
		);

		#[cfg(debug_assertions)]
		println!("{log_entry}");

		self.logs.push(log_entry);
	}

	pub fn send_logs(&mut self) {
		let _logs = mem::take(&mut self.logs);

		// for log in logs {
		// 	// TODO: Actually send the logs to the server.
		// 	// TODO: When this happens, the IO bound test must be ignored.
		// 	println!("{log}");
		//}
	}
}

struct LogCounter {
	date: NaiveDate,
	counter: u32,
}

impl LogCounter {
	fn new() -> Self {
		LogCounter {
			date: Local::now().date_naive(),
			counter: 0,
		}
	}

	fn next(&mut self) -> u32 {
		let today = Local::now().date_naive();

		if today == self.date {
			self.counter += 1;
		} else {
			self.date = today;
			self.counter = 1;
		}

		self.counter
	}
}

#[cfg(test)]
mod test {
	use chrono::Duration;

	use super::*;

	impl LogCounter {
		fn reset_counter(&mut self) {
			let yesterday = Local::now().date_naive() - Duration::days(1);
			self.date = yesterday;
		}

		fn back_one_hour(&mut self) {
			let hour_ago = Local::now().date_naive() - Duration::hours(1);
			self.date = hour_ago;
		}

		fn forward_one_hour(&mut self) {
			let hour_forward = Local::now().date_naive() + Duration::hours(1);
			self.date = hour_forward;
		}
	}

	// Logger guard test

	#[test]
	fn info() {
		logger().log_info("This is INFO log test 1.");

		logger().log_info("This is INFO log test 2.");

		logger().log_info("This is INFO log test 3.");

		logger().send_logs();
		logger().send_logs();

		logger().log_info("This is INFO log test 4.");

		logger().log_info("This is INFO log test 5.");

		logger().log_info("This is INFO log test 6.");

		logger().send_logs();
		logger().send_logs();
	}

	#[test]
	fn warn() {
		logger().log_warn("This is WARN log test 1.");

		logger().log_warn("This is WARN log test 2.");

		logger().log_warn("This is WARN log test 3.");

		logger().send_logs();
		logger().send_logs();

		logger().log_warn("This is WARN log test 4.");

		logger().log_warn("This is WARN log test 5.");

		logger().log_warn("This is WARN log test 6.");

		logger().send_logs();
		logger().send_logs();
	}

	#[test]
	fn error() {
		logger().log_error("This is an ERROR log test 1.");

		logger().log_error("This is an ERROR log test 2.");

		logger().log_error("This is an ERROR log test 3.");

		logger().send_logs();
		logger().send_logs();

		logger().log_error("This is an ERROR log test 1.");

		logger().log_error("This is an ERROR log test 2.");

		logger().log_error("This is an ERROR log test 3.");

		logger().send_logs();
		logger().send_logs();
	}

	#[test]
	fn new_day() {
		let mut logger = Logger::new();

		logger.log_info("Logging today.");

		logger.counter.reset_counter();

		logger.log_info("Reset counter.");
		logger.log_info("Logging again today.");
		logger.log_info("Logging again today.");

		logger.counter.reset_counter();

		logger.log_info("Reset counter.");
		logger.log_info("Logging again today.");
		logger.log_info("Logging again today.");

		logger.send_logs();
	}

	#[test]
	fn daylight_savings() {
		let mut logger = Logger::new();

		logger.log_info("First log. Now going back an hour.");

		logger.counter.back_one_hour();

		logger.log_info("Back one hour. Now going forward one hour.");

		logger.counter.forward_one_hour();

		logger.log_info("Back to normal. Going forward one hour again.");

		logger.counter.forward_one_hour();

		logger.log_info("Now forward one hour.");

		logger.send_logs();
	}
}
