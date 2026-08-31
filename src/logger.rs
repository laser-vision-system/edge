//! # Logger
//!
//! Provides logging support for the edge.
//!
//! This module can only be accessed by calling importing the logger() function.
//!
//! To create a log, call one of the following functions:
//!
//! - logger().log_info(message: &str)
//! - logger().log_warn(message: &str)
//! - logger().log_error(message: &str, error: EdgeError)
//!
//! To send all buffered logs to the server, use:
//!
//! - logger().send_logs()

use std::sync::{Mutex, OnceLock};

use chrono::{Duration, NaiveDate, Utc};

use crate::error::EdgeError::{self};

use serde::{Deserialize, Serialize};

static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

pub fn logger() -> &'static Mutex<Logger> {
	LOGGER.get_or_init(|| Mutex::new(Logger::new()))
}

#[derive(Debug, Serialize, Deserialize)]
enum LogLevel {
	INFO,
	WARN,
	ERROR,
}

#[derive(Serialize, Deserialize)]
/// A single log entry.
struct LogEntry {
	count: u32,
	timestamp: String,
	level: LogLevel,
	message: String,
}

impl LogEntry {
	fn new(count: u32, level: LogLevel, message: String) -> Self {
		let timestamp = Utc::now().to_rfc3339();

		LogEntry {
			count,
			timestamp,
			level,
			message,
		}
	}
}

/// The logger struct that holds each unsent (buffered) log entry and the log count.
///
/// Each log entry is stored in a Vector.
///
/// All log entries are consumed upon calling the send_logs function.
pub struct Logger {
	logs: Vec<LogEntry>,
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
		let log_entry = LogEntry::new(
			// TODO: Handle unwrap
			self.counter.next(),
			LogLevel::INFO,
			message.to_string(),
		);

		self.logs.push(log_entry);
	}

	pub fn log_warn(&mut self, message: &str) {
		let log_entry = LogEntry::new(
			// TODO: Handle unwrap
			self.counter.next(),
			LogLevel::WARN,
			message.to_string(),
		);

		self.logs.push(log_entry);
	}

	pub fn log_error(&mut self, message: &str, error: EdgeError) {
		let concat_message = format!("{:?}: {}\n{}", error, error, message);

		let log_entry = LogEntry::new(
			// TODO: Handle unwrap
			self.counter.next(),
			LogLevel::ERROR,
			concat_message,
		);

		self.logs.push(log_entry);
	}

	pub fn send_logs(&mut self) {
		let logs = std::mem::take(&mut self.logs);

		for log in logs {
			// TODO: Actually send the logs to the server
			self._print_log(&log);
		}
	}

	fn _print_log(&self, log: &LogEntry) {
		println!(
			"[{:05}] {}\n{:?}\n{}\n----------",
			log.count, log.timestamp, log.level, log.message
		);
	}
}

struct LogCounter {
	date: NaiveDate,
	counter: u32,
}

impl LogCounter {
	fn new() -> Self {
		LogCounter {
			date: Utc::now().date_naive(),
			counter: 0,
		}
	}

	fn next(&mut self) -> u32 {
		let today = Utc::now().date_naive();

		if today != self.date {
			self.date = today;
			self.counter = 1;
		} else {
			self.counter += 1;
		}

		self.counter
	}

	fn _reset_counter(&mut self) {
		let yesterday = Utc::now().date_naive() - Duration::days(1);
		self.date = yesterday;
	}
}

#[cfg(test)]
mod test {
	use crate::error::EdgeError::{
		AutoConnect, FrameDecodeCorruption, FrameDecodeOutOfMemory, FrameRead, General,
		NokhwaInitialisation, NokhwaNoBackend, ServerConnection, SetControl, StreamOpen,
		WebSocketConnection,
	};

	use super::logger;

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
		let mut error = AutoConnect;
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", error);

		error = WebSocketConnection;
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", error);

		error = ServerConnection;
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", error);

		error = NokhwaNoBackend;
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", error);

		error = NokhwaInitialisation {
			index: 69,
			reason: "*Generic reason*".to_string(),
		};
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", error);

		error = StreamOpen {
			reason: "*Generic reason*".to_string(),
		};
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", error);

		error = FrameRead;
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", error);

		error = FrameDecodeCorruption;
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", error);

		error = FrameDecodeOutOfMemory;
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", error);

		error = SetControl;
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", error);

		error = General("*Generic reason*".to_string());
		logger()
			.lock()
			.unwrap()
			.log_error("This is an ERROR log test.", error);

		logger().lock().unwrap().send_logs();
		logger().lock().unwrap().send_logs();
	}

	#[test]
	fn new_day() {
		logger().lock().unwrap().log_info("Logging today.");

		// {
		//     let mut counter = logger().lock().unwrap().counter.lock().unwrap();
		//     counter._reset_counter();
		// }

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
}
