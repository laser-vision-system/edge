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
	clippy::print_stderr,
	clippy::print_stdout,
	reason = "This is the logger module, where this is expected behaviour"
)]
#![expect(clippy::inline_modules, reason = "Simple unit tests")]

use std::sync::{Mutex, OnceLock};

use chrono::{Duration, Local, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::error::EdgeError;

// Declare a type alias for easy imports
pub type LoggerRef = &'static Mutex<Logger>;

static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

pub fn logger() -> LoggerRef {
	LOGGER.get_or_init(|| Mutex::new(Logger::new()))
}

#[derive(Debug, Serialize, Deserialize)]
enum LogLevel {
	Info,
	Warn,
	Error,
}

// #[derive(Serialize, Deserialize)]
// /// A single log entry.
// struct LogEntry {
// 	count: u32,
// 	timestamp: String,
// 	level: LogLevel,
// 	message: String,
// }

// impl LogEntry {
// 	fn new(count: u32, level: LogLevel, message: String) -> Self {
// 		let timestamp = Local::now().to_rfc3339();

// 		LogEntry {
// 			count,
// 			timestamp,
// 			level,
// 			message,
// 		}
// 	}
// }

// The logger struct that holds each unsent (buffered) log entry and the log
// count.
//
// Each log entry is stored in a Vector.
//
// All log entries are consumed upon calling the send_logs function.
pub struct Logger {
	logs: Vec<String>,
	counter: LogCounter,
}

impl Logger {
	fn new() -> Self {
		Logger {
			logs: Vec::new(),
			counter: LogCounter::new(),
		}
	}

	pub fn log_info(&mut self, message: &str) {
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

	pub fn log_error(&mut self, message: &str, error: &EdgeError) {
		let log_entry = format!(
			"[{:05}] {}\n{:?}\n{:?}: {}\n{}\n----------",
			self.counter.next(),
			Local::now().to_rfc3339(),
			LogLevel::Error,
			error,
			error,
			message,
		);

		#[cfg(debug_assertions)]
		println!("{log_entry}");

		self.logs.push(log_entry);
	}

	pub fn send_logs(&mut self) {
		let logs = std::mem::take(&mut self.logs);

		for log in logs {
			// TODO: Actually send the logs to the server.
			// TODO: When this happens, the IO bound test must be ignored.
			println!("{log}");
		}
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

	fn _reset_counter(&mut self) {
		let yesterday = Local::now().date_naive() - Duration::days(1);
		self.date = yesterday;
	}

	fn _back_one_hour(&mut self) {
		let hour_ago = Local::now().date_naive() - Duration::hours(1);
		self.date = hour_ago;
	}

	fn _forward_one_hour(&mut self) {
		let hour_forward = Local::now().date_naive() + Duration::hours(1);
		self.date = hour_forward;
	}
}

#[cfg(test)]
mod test {
	#![expect(clippy::unwrap_used, reason = "Tests are allowed to unwrap")]

	use super::logger;
	use crate::{
		error::EdgeError, network::NetworkError,
		tcam_interface::CameraDeviceError,
	};

	#[test]
	fn info() {
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
	fn all_error_types() {
		let error = NetworkError::AutoConnect.into();
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", &error);

		let error = NetworkError::WebSocketConnection.into();
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", &error);

		let error = NetworkError::ServerConnection.into();
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", &error);

		let error = CameraDeviceError::NokhwaNoBackend.into();
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", &error);

		let error = CameraDeviceError::NokhwaInitialisation {
			index: "69".into(),
			reason: "*Generic reason*".to_owned(),
		}
		.into();
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", &error);

		let error = CameraDeviceError::StreamOpen {
			reason: "*Generic reason*".to_owned(),
		}
		.into();
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", &error);

		let error = CameraDeviceError::FrameRead.into();
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", &error);

		let error = CameraDeviceError::FrameDecodeCorruption.into();
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", &error);

		let error = CameraDeviceError::FrameDecodeOutOfMemory.into();
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", &error);

		let error = CameraDeviceError::SetControl.into();
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", &error);

		let error = EdgeError::General {
			reason: "*Generic reason*".to_owned(),
		};
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", &error);

		logger().lock().unwrap().send_logs();
		logger().lock().unwrap().send_logs();
	}

	#[test]
	fn new_day() {
		logger().lock().unwrap().log_info("Logging today.");

		logger().lock().unwrap().counter._reset_counter();

		logger().lock().unwrap().log_info("Reset counter.");
		logger().lock().unwrap().log_info("Logging again today.");
		logger().lock().unwrap().log_info("Logging again today.");

		logger().lock().unwrap().counter._reset_counter();

		logger().lock().unwrap().log_info("Reset counter.");
		logger().lock().unwrap().log_info("Logging again today.");
		logger().lock().unwrap().log_info("Logging again today.");

		logger().lock().unwrap().send_logs();
	}

	#[test]
	fn daylight_savings() {
		logger()
			.lock()
			.unwrap()
			.log_info("First log. Now going back an hour.");

		logger().lock().unwrap().counter._back_one_hour();

		logger()
			.lock()
			.unwrap()
			.log_info("Back one hour. Now going forward one hour.");

		logger().lock().unwrap().counter._forward_one_hour();

		logger()
			.lock()
			.unwrap()
			.log_info("Back to normal. Going forward one hour again.");

		logger().lock().unwrap().counter._forward_one_hour();

		logger().lock().unwrap().log_info("Now forward one hour.");

		logger().lock().unwrap().send_logs();
	}
}
