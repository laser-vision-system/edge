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
	reason = "This is the logger module, where this is expected behaviour"
)]

use std::{
	mem,
	sync::{Mutex, OnceLock},
};

use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::error::EdgeError;

// Declare a type alias for easy imports
pub type LoggerRef = &'static Mutex<Logger>;

static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

pub fn logger() -> LoggerRef {
	LOGGER.get().expect("Logger has not been initialised.")
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
/// All log entries are consumed upon calling the send_logs function.
///
/// ### Verbosity Levels
/// The logger supports different verbosity levels, which can be set via the
/// `VERBOSITY` environment variable. The levels are as follows:
/// - 0: No logs are recorded or sent.
/// - 1: Only error logs are recorded and sent.
/// - 2: Warning and error logs are recorded and sent.
/// - 3: Info, warning, and error logs are recorded and sent.
///
/// Any other inputs including non-numeric strings will default to level 1 (error logs only).
pub struct Logger {
	logs: Vec<String>,
	counter: LogCounter,
	verbosity: u8,
}

impl Logger {
	pub fn initialise_logger(verbosity: u8) {
		match LOGGER.set(Mutex::new(Logger::new(verbosity))) {
			Ok(_) => {},
			Err(_) => {
				println!("Logger cannot be initialised more than once.")
			},
		}
	}

	fn new(verbosity: u8) -> Self {
		Logger {
			logs: Vec::new(),
			counter: LogCounter::new(),
			verbosity,
		}
	}

	pub fn log_info(&mut self, message: &str) {
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

	pub fn log_warn(&mut self, message: &str) {
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

	pub fn log_error(&mut self, error: &EdgeError) {
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
		let logs = mem::take(&mut self.logs);

		//for log in logs {
		//	// TODO: Actually send the logs to the server.
		//	// TODO: When this happens, the IO bound test must be ignored.
		//	println!("{log}");
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

	#[test]
	fn info() {
		Logger::initialise_logger(3);

		logger()
			.lock()
			.unwrap()
			.log_info("This is INFO log test 1.");

		logger()
			.lock()
			.unwrap()
			.log_info("This is INFO log test 2.");

		logger()
			.lock()
			.unwrap()
			.log_info("This is INFO log test 3.");

		logger().lock().unwrap().send_logs();
		logger().lock().unwrap().send_logs();

		logger()
			.lock()
			.unwrap()
			.log_info("This is INFO log test 4.");

		logger()
			.lock()
			.unwrap()
			.log_info("This is INFO log test 5.");

		logger()
			.lock()
			.unwrap()
			.log_info("This is INFO log test 6.");

		logger().lock().unwrap().send_logs();
		logger().lock().unwrap().send_logs();
	}

	#[test]
	fn warn() {
		Logger::initialise_logger(3);

		logger()
			.lock()
			.unwrap()
			.log_warn("This is WARN log test 1.");

		logger()
			.lock()
			.unwrap()
			.log_warn("This is WARN log test 2.");

		logger()
			.lock()
			.unwrap()
			.log_warn("This is WARN log test 3.");

		logger().lock().unwrap().send_logs();
		logger().lock().unwrap().send_logs();

		logger()
			.lock()
			.unwrap()
			.log_warn("This is WARN log test 4.");

		logger()
			.lock()
			.unwrap()
			.log_warn("This is WARN log test 5.");

		logger()
			.lock()
			.unwrap()
			.log_warn("This is WARN log test 6.");

		logger().lock().unwrap().send_logs();
		logger().lock().unwrap().send_logs();
	}

	#[test]
	fn error() {
		Logger::initialise_logger(3);

		logger()
			.lock()
			.unwrap()
			.log_error(&"This is an ERROR log test 1.".to_owned().into());

		logger()
			.lock()
			.unwrap()
			.log_error(&"This is an ERROR log test 2.".to_owned().into());

		logger()
			.lock()
			.unwrap()
			.log_error(&"This is an ERROR log test 3.".to_owned().into());

		logger().lock().unwrap().send_logs();
		logger().lock().unwrap().send_logs();

		logger()
			.lock()
			.unwrap()
			.log_error(&"This is an ERROR log test 1.".to_owned().into());

		logger()
			.lock()
			.unwrap()
			.log_error(&"This is an ERROR log test 2.".to_owned().into());

		logger()
			.lock()
			.unwrap()
			.log_error(&"This is an ERROR log test 3.".to_owned().into());

		logger().lock().unwrap().send_logs();
		logger().lock().unwrap().send_logs();
	}

	#[test]
	fn new_day() {
		Logger::initialise_logger(3);

		logger().lock().unwrap().log_info("Logging today.");

		logger().lock().unwrap().counter.reset_counter();

		logger().lock().unwrap().log_info("Reset counter.");
		logger().lock().unwrap().log_info("Logging again today.");
		logger().lock().unwrap().log_info("Logging again today.");

		logger().lock().unwrap().counter.reset_counter();

		logger().lock().unwrap().log_info("Reset counter.");
		logger().lock().unwrap().log_info("Logging again today.");
		logger().lock().unwrap().log_info("Logging again today.");

		logger().lock().unwrap().send_logs();
	}

	#[test]
	fn daylight_savings() {
		Logger::initialise_logger(3);

		logger()
			.lock()
			.unwrap()
			.log_info("First log. Now going back an hour.");

		logger().lock().unwrap().counter.back_one_hour();

		logger()
			.lock()
			.unwrap()
			.log_info("Back one hour. Now going forward one hour.");

		logger().lock().unwrap().counter.forward_one_hour();

		logger()
			.lock()
			.unwrap()
			.log_info("Back to normal. Going forward one hour again.");

		logger().lock().unwrap().counter.forward_one_hour();

		logger().lock().unwrap().log_info("Now forward one hour.");

		logger().lock().unwrap().send_logs();
	}
}
