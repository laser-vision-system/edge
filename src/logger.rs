// Each log needs to include:
// - The log count (key)
// - The timestamp of when the log was generated
// - The log level (e.g., INFO, WARN, ERROR)
// - The type of error (optional)
// - Message (what happened)

// There should be a log object created at the start of the program that is used throughout the program to log messages.

//

use std::{
    time::{SystemTime, UNIX_EPOCH},
    sync::OnceLock,
};

use chrono::{NaiveDate, Utc};

use crate::error::EdgeError::{self};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
	error_msg: String,
	message: String,
}

impl LogEntry {
	fn new(count: u32, level: LogLevel, message: &str, error: Option<EdgeError>) -> Self {
		let timestamp = Utc::now().to_rfc3339();
		let error_msg = match error {
			Some(error) => format!("{:?}", error),
			None => String::new(),
		};

		LogEntry {
			count: count,
			timestamp: timestamp,
			level: level,
			error_msg: error_msg,
			message: message.to_string(),
		}
	}
}

/// The logger struct that holds each unsetn log entry and the log count.
///
/// Each log entry is stored in a Vector.
///
/// All log entries are consumed upon calling the send_logs function.
pub struct Logger {
	logs: Vec<LogEntry>,
	count: LogCounter,
}

impl Logger {
	/// This function creates a new logger instance.
	///
	/// It should only be called once at the start of the program.
	fn new() -> Self {
		let mut logger = Logger {
			logs: Vec::new(),
			count: LogCounter::new(),
		};

        logger.log_info("Logger initialised.");

        logger
	}

	pub fn log_info(&mut self, message: &str) {
		let log_entry = LogEntry::new(
            self.count.next(),
            LogLevel::INFO, message,
            None
        );
		self.stash_log(log_entry);
	}

	pub fn log_warn(&mut self, message: &str) {
		let log_entry = LogEntry::new(
            self.count.next(),
            LogLevel::WARN, message,
            None
        );
		self.stash_log(log_entry);
	}

	pub fn log_error(&mut self, message: &str, error: Option<EdgeError>) {
		let log_entry = LogEntry::new(
            self.count.next(),
            LogLevel::ERROR,
            message,
            error
        );
		self.stash_log(log_entry);
	}

	fn stash_log(&mut self, log_entry: LogEntry) {
		self.logs.push(log_entry);
	}

	pub fn send_logs(self) {
		for log in self.logs {
			// send the logs to the server
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
			date: Utc::now().date_naive(),
			counter: 0,
		}
	}

	fn next(&mut self) -> u32 {
		let today = Utc::now().date_naive();

		if today != self.date {
			self.date = today;
			self.counter = 1
		} else {
			self.counter += 1;
		}

		self.counter
	}
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn logger() -> &'static Logger {
    LOGGER.get_or_init(Logger::new)
}
