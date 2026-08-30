// Each log needs to include:
// - The log count (key)
// - The timestamp of when the log was generated
// - The log level (e.g., INFO, WARN, ERROR)
// - The type of error (optional)
// - Message (what happened)

// There should be a log object created at the start of the program that is used throughout the program to log messages.

//

use std::sync::{Mutex, OnceLock};

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
	logs: Mutex<Vec<LogEntry>>,
	count: Mutex<LogCounter>,
}

impl Logger {
	/// This function creates a new logger instance.
	///
	/// It should only be called once at the start of the program.
	fn new() -> Self {
		let logger = Logger {
			logs: Mutex::new(Vec::new()),
			count: Mutex::new(LogCounter::new()),
		};

        logger.log_info("Logger initialised.");

        logger
	}

	pub fn log_info(&self, message: &str) {
        let mut logs = self.logs.lock().unwrap();

		let log_entry = LogEntry::new(
            // TODO: Handle unwrap better (or not use Mutex)
            self.count.lock().unwrap().next(),
            LogLevel::INFO, message,
            None
        );

        logs.push(log_entry);
	}

	pub fn log_warn(&self, message: &str) {
        let mut logs = self.logs.lock().unwrap();

		let log_entry = LogEntry::new(
            // TODO: Handle unwrap better (or not use Mutex)
            self.count.lock().unwrap().next(),
            LogLevel::WARN, message,
            None
        );
		
        logs.push(log_entry);
	}

	pub fn log_error(&self, message: &str, error: EdgeError) {
        let mut logs = self.logs.lock().unwrap();

		let log_entry = LogEntry::new(
            // TODO: Handle unwrap better (or not use Mutex)
            self.count.lock().unwrap().next(),
            LogLevel::ERROR,
            message,
            Some(error)
        );
		
        logs.push(log_entry);
	}

	pub fn send_logs(&self) {
        let logs = self.logs.lock().unwrap();
		for log in logs.iter() {
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
            logger().log_info("Last log before counter resets.");
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
