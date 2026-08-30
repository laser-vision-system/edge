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
};

use crate::error::EdgeError::{self};

enum LogLevel {
	INFO,
	WARN,
	ERROR,
}

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
        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();
        let error_msg = match error {
            Some(error) => format!("{:?}", error),
            None => String::new(),
        };

        LogEntry {
            count,
            timestamp,
            level,
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
    count: u32,
}

impl Logger {
	/// This function creates a new logger instance.
	///
	/// It should only be called once at the start of the program.
	pub fn new() -> Self {
		Logger {
			logs: vec![],
			count: 1,
		}
	}

	pub fn log_info(&mut self, message: &str) {
		let log_entry = LogEntry::new(self.count, LogLevel::INFO, message, None);
		self.stash_log(log_entry);
	}

    pub fn log_warn(&mut self, message: &str) {
        let log_entry = LogEntry::new(self.count, LogLevel::WARN, message, None);
        self.stash_log(log_entry);
    }

    pub fn log_error(&mut self, message: &str, error: Option<EdgeError>) {
        let log_entry = LogEntry::new(self.count, LogLevel::ERROR, message, error);
        self.stash_log(log_entry);
    }

    fn stash_log(&mut self, log_entry: LogEntry) {
        self.logs.push(log_entry);
        self.count += 1;
    }

	pub fn send_logs(&self) {
		for log in &self.logs {
			// send the logs to the server
		}
	}
}
