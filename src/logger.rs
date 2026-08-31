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

use chrono::{NaiveDate, Utc, Duration};

use crate::error::EdgeError::{self};

use serde::{Deserialize, Serialize};

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn logger() -> &'static Logger {
	LOGGER.get_or_init(Logger::new)
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
	fn new(count: u32, level: LogLevel, message: &str, error: Option<EdgeError>) -> Self {
		let timestamp = Utc::now().to_rfc3339();
		let error_msg = match error {
			Some(error) => format!("{:?}: {}\n", error, error),
			None => String::new(),
		};

		let message = error_msg + message;

		LogEntry {
			count,
			timestamp,
			level,
			message
		}
	}
}

/// The logger struct that holds each unsent (buffered) log entry and the log count.
///
/// Each log entry is stored in a Vector.
///
/// All log entries are consumed upon calling the send_logs function.
pub struct Logger {
	logs: Mutex<Vec<LogEntry>>,
	counter: Mutex<LogCounter>,
}

impl Logger {
	fn new() -> Self {
		let logger = Logger {
			logs: Mutex::new(Vec::new()),
			counter: Mutex::new(LogCounter::new()),
		};

		logger.log_info("Logger initialised.");

		logger
	}

	pub fn log_info(&self, message: &str) {
		let mut logs = self.logs.lock().unwrap();

		let log_entry = LogEntry::new(
			// TODO: Handle unwrap
			self.counter.lock().unwrap().next(),
			LogLevel::INFO,
			message,
			None,
		);

		logs.push(log_entry);
	}

	pub fn log_warn(&self, message: &str) {
		let mut logs = self.logs.lock().unwrap();

		let log_entry = LogEntry::new(
			// TODO: Handle unwrap
			self.counter.lock().unwrap().next(),
			LogLevel::WARN,
			message,
			None,
		);

		logs.push(log_entry);
	}

	pub fn log_error(&self, message: &str, error: EdgeError) {
		let mut logs = self.logs.lock().unwrap();

		let log_entry = LogEntry::new(
			// TODO: Handle unwrap
			self.counter.lock().unwrap().next(),
			LogLevel::ERROR,
			message,
			Some(error),
		);

		logs.push(log_entry);
	}

	pub fn send_logs(&self) {
		let logs = {
			let mut logs = self.logs.lock().unwrap();
			std::mem::take(&mut *logs)
		};
		for log in logs {
            // TODO: Actually send the logs to the server
			logger()._print_log(&log);
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

    pub fn _set_date_to_yesterday(&mut self) {
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
	fn all_error_types() {
		let mut error = AutoConnect;
		logger().log_error("This is an ERROR log test.", error);

		error = WebSocketConnection;
		logger().log_error("This is an ERROR log test.", error);

		error = ServerConnection;
		logger().log_error("This is an ERROR log test.", error);

		error = NokhwaNoBackend;
		logger().log_error("This is an ERROR log test.", error);

		error = NokhwaInitialisation {
			index: 69,
			reason: "*Generic reason*".to_string(),
		};
		logger().log_error("This is an ERROR log test.", error);

		error = StreamOpen {
			reason: "*Generic reason*".to_string(),
		};
		logger().log_error("This is an ERROR log test.", error);

		error = FrameRead;
		logger().log_error("This is an ERROR log test.", error);

		error = FrameDecodeCorruption;
		logger().log_error("This is an ERROR log test.", error);

		error = FrameDecodeOutOfMemory;
		logger().log_error("This is an ERROR log test.", error);

		error = SetControl;
		logger().log_error("This is an ERROR log test.", error);

		error = General("*Generic reason*".to_string());
		logger().log_error("This is an ERROR log test.", error);

        logger().send_logs();
        logger().send_logs();
	}

	#[test]
	fn new_day() {
        logger().log_info("Logging today.");

        {
            let mut counter = logger().counter.lock().unwrap();
            counter._set_date_to_yesterday();
        }
        
        logger().log_info("Attempting to log yesterday.");

        logger().log_info("Logging again today.");

        logger().log_info("Logging again today.");

        {
            let mut counter = logger().counter.lock().unwrap();
            counter._set_date_to_yesterday();
        }

        logger().log_info("Attempting to log yesterday.");

        logger().log_info("Logging again today.");

        logger().log_info("Logging again today.");

        logger().send_logs();
    }
}
