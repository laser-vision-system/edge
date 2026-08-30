// Each log needs to include:
// - The log count (key)
// - The timestamp of when the log was generated
// - The log level (e.g., INFO, WARN, ERROR)
// - The type of error (optional)
// - Message (what happened)

// There should be a log object created at the start of the program that is used throughout the program to log messages.

//

use std::{
	collections::HashMap,
};

use crate::error::EdgeError;

pub enum LogLevel {
	INFO,
	WARN,
	ERROR,
}

/// A single log entry.
struct LogEntry {
	timestamp: String,
	level: LogLevel,
	error_type: Option<EdgeError>,
	message: String,
}

/// The logger struct that holds each unsetn log entry and the log count.
///
/// Each log entry is stored in a HashMap with the log count as the key.
///
/// All log entries are consumed upon calling the send_logs function.
pub struct Logger {
	logs: HashMap<u64, LogEntry>,
	count: u64,
}

impl Logger {
	/// This function creates a new logger instance.
	///
	/// It should only be called once at the start of the program.
	pub fn new() -> Self {
		Logger {
			logs: HashMap::new(),
			count: 0,
		}
	}

	pub fn log(&self, level: LogLevel, message: &str, error: Option<EdgeError>) {
		// do something
	}

	pub fn send_logs(&self) {
		for log in &self.logs {
			// send the logs to the server
		}
	}
}
