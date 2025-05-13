mod logger;
use logger::{Logger, LogLevel};

fn main() {
    let logger = Logger::new("logs/system.log");

    logger.log(LogLevel::Info, "System initialized.");
    logger.log(LogLevel::Warning, "High GPU memory usage detected.");
    logger.log(LogLevel::Error, "GPU process crashed unexpectedly.");
}
