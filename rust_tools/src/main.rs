mod logger;
mod gpu_metrics;

use logger::{Logger, LogLevel};
use gpu_metrics::log_gpu_metrics;
use clap::Parser;
use std::{thread, time::Duration};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value = "logs/system.log")]
    log_file: String,
    #[arg(short, long, default_value_t = 5)]
    entries: u32,
}

fn main() {
    let args = Args::parse();
    let logger = Logger::new(&args.log_file);

    logger.log(LogLevel::Info, "System Logger initialized.");

    for i in 1..=args.entries {
        logger.log(LogLevel::Info, &format!("Log entry {}", i));

        // GPU metrics log
        log_gpu_metrics();

        thread::sleep(Duration::from_secs(2));
    }

    logger.log(LogLevel::Info, "Logging complete.");
}
