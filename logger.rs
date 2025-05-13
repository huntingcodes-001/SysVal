use std::fs::{OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};
use chrono::Local;

// ANSI color codes
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

#[derive(Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub struct Logger {
    log_file: Arc<Mutex<std::fs::File>>,
}

impl Logger {
    pub fn new(file_path: &str) -> Logger {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Unable to open log file");

        Logger {
            log_file: Arc::new(Mutex::new(file)),
        }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let level_str = match level {
            LogLevel::Info => format!("{}[INFO]{}", GREEN, RESET),
            LogLevel::Warning => format!("{}[WARN]{}", YELLOW, RESET),
            LogLevel::Error => format!("{}[ERROR]{}", RED, RESET),
        };

        let log_entry = format!("{} | {} {}\n", timestamp, level_str, message);

        // Print to terminal
        print!("{}", log_entry);

        // Append to file safely
        let mut file_lock = self.log_file.lock().unwrap();
        file_lock.write_all(log_entry.as_bytes()).expect("Failed to write to log file");
    }
}
