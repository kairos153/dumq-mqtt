use log::{LevelFilter, Log, Metadata, Record};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    Handle,
};
use std::path::Path;

/// Initialize file logging for debugging
/// 
/// This function sets up logging to write to both console and file.
/// The log file will be created in the specified directory with the given filename.
/// 
/// # Arguments
/// * `log_dir` - Directory where log files will be stored
/// * `log_filename` - Name of the log file (e.g., "dumq_mqtt.log")
/// * `level` - Minimum log level to capture (default: Debug)
/// 
/// # Example
/// ```rust
/// use dumq_mqtt::logging::init_file_logging;
/// 
/// // Initialize logging to write to "logs/dumq_mqtt.log"
/// init_file_logging("logs", "dumq_mqtt.log", log::LevelFilter::Debug).unwrap();
/// ```
pub fn init_file_logging(
    log_dir: &str,
    log_filename: &str,
    level: LevelFilter,
) -> Result<Handle, Box<dyn std::error::Error>> {
    // Create log directory if it doesn't exist
    std::fs::create_dir_all(log_dir)?;
    
    let log_path = Path::new(log_dir).join(log_filename);
    
    // Create file appender
    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S%.3f)} [{l}] {t} - {m}{n}"
        )))
        .build(log_path)?;
    
    // Create console appender
    let console_appender = log4rs::append::console::ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S%.3f)} [{l}] {t} - {m}{n}"
        )))
        .build();
    
    // Configure appenders
    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .build(Root::builder()
            .appender("file")
            .appender("console")
            .build(level))?;
    
    // Initialize the logger
    let handle = log4rs::init_config(config)?;
    
    log::info!("File logging initialized. Log file: {:?}", Path::new(log_dir).join(log_filename));
    
    Ok(handle)
}

/// Initialize simple file logging (console + file)
/// 
/// This is a simpler version that just logs to both console and a single file
/// without rotation or complex filtering.
pub fn init_simple_file_logging(
    log_file: &str,
    level: LevelFilter,
) -> Result<Handle, Box<dyn std::error::Error>> {
    // Create file appender
    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S%.3f)} [{l}] {t} - {m}{n}"
        )))
        .build(log_file)?;
    
    // Create console appender
    let console_appender = log4rs::append::console::ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S%.3f)} [{l}] {t} - {m}{n}"
        )))
        .build();
    
    // Configure appenders
    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .build(Root::builder()
            .appender("file")
            .appender("console")
            .build(level))?;
    
    // Initialize the logger
    let handle = log4rs::init_config(config)?;
    
    log::info!("Simple file logging initialized. Log file: {}", log_file);
    
    Ok(handle)
}

/// Initialize logging with environment variable support
/// 
/// This function initializes logging based on the RUST_LOG environment variable
/// and optionally writes to a file if specified.
/// 
/// # Arguments
/// * `log_file` - Optional log file path. If None, only console logging is used.
/// * `default_level` - Default log level if RUST_LOG is not set
pub fn init_env_logging(
    log_file: Option<&str>,
    default_level: LevelFilter,
) -> Result<Option<Handle>, Box<dyn std::error::Error>> {
    // Check if RUST_LOG is set
    let _rust_log = std::env::var("RUST_LOG").ok();
    
    if let Some(file_path) = log_file {
        // Initialize file logging
        let handle = init_simple_file_logging(file_path, default_level)?;
        log::info!("Environment-based file logging initialized. Log file: {}", file_path);
        Ok(Some(handle))
    } else {
        // Use env_logger for console-only logging
        env_logger::Builder::new()
            .filter_level(default_level)
            .init();
        log::info!("Environment-based console logging initialized");
        Ok(None)
    }
}

/// Custom logger that can be used for testing or special cases
pub struct CustomLogger {
    level: LevelFilter,
}

impl CustomLogger {
    pub fn new(level: LevelFilter) -> Self {
        Self { level }
    }
}

impl Log for CustomLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
            println!(
                "{} [{}] {} - {}",
                timestamp,
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}


