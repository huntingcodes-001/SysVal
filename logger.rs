use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::Local;

fn main() {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/telemetry.log")
        .unwrap();

    let now = Local::now();
    let log_entry = format!("{} - Rust Logger: Test Completed.\n", now);
    file.write_all(log_entry.as_bytes()).unwrap();
}
