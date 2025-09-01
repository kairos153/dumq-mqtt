#!/usr/bin/env rust-script
//! Setup file logging for DumQ MQTT debugging
//! 
//! This script helps you set up file logging for debugging your MQTT application.
//! Run this script to create the necessary logging configuration.

use std::fs;
use std::path::Path;

fn main() {
    println!("DumQ MQTT Logging Setup");
    println!("=======================");
    println!();
    
    // Create logs directory
    let logs_dir = "logs";
    if !Path::new(logs_dir).exists() {
        fs::create_dir(logs_dir).expect("Failed to create logs directory");
        println!("✓ Created logs directory: {}", logs_dir);
    } else {
        println!("✓ Logs directory already exists: {}", logs_dir);
    }
    
    // Create .gitignore entry for logs if it doesn't exist
    let gitignore_path = ".gitignore";
    let gitignore_content = fs::read_to_string(gitignore_path).unwrap_or_default();
    
    if !gitignore_content.contains("logs/") {
        let mut content = gitignore_content.clone();
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str("\n# Log files\nlogs/\n*.log\n");
        
        fs::write(gitignore_path, content).expect("Failed to write to .gitignore");
        println!("✓ Updated .gitignore to exclude log files");
    } else {
        println!("✓ .gitignore already configured for log files");
    }
    
    // Create example logging configuration
    let config_content = r#"# DumQ MQTT Logging Configuration
# 
# This file shows how to set up logging for debugging.
# Copy the relevant section to your application.

# Method 1: Simple file logging
# 
# use dumq_mqtt::logging::init_simple_file_logging;
# 
# let _handle = init_simple_file_logging(
#     "debug.log",
#     log::LevelFilter::Debug,
# ).unwrap();

# Method 2: Directory-based logging with rotation
# 
# use dumq_mqtt::logging::init_file_logging;
# 
# let _handle = init_file_logging(
#     "logs",
#     "dumq_mqtt.log",
#     log::LevelFilter::Info,
# ).unwrap();

# Method 3: Environment-based logging
# 
# use dumq_mqtt::logging::init_env_logging;
# 
# let _handle = init_env_logging(
#     Some("logs/app.log"),
#     log::LevelFilter::Debug,
# ).unwrap();

# Environment Variables:
# 
# Set RUST_LOG to control log levels:
# export RUST_LOG=debug                    # All debug logs
# export RUST_LOG=dumq_mqtt=debug          # Only dumq_mqtt debug logs
# export RUST_LOG=dumq_mqtt::server=debug  # Only server debug logs
# export RUST_LOG=info                     # Only info and above
# export RUST_LOG=warn                     # Only warnings and errors
# export RUST_LOG=error                    # Only errors

# Log Levels (from most to least verbose):
# - trace: Very detailed debugging information
# - debug: General debugging information
# - info: General information about program execution
# - warn: Warning messages
# - error: Error messages
"#;
    
    fs::write("logging_config.txt", config_content).expect("Failed to create logging config file");
    println!("✓ Created logging configuration guide: logging_config.txt");
    
    println!();
    println!("Setup complete! You can now use file logging in your applications.");
    println!();
    println!("Quick start:");
    println!("1. Add this to your main.rs or lib.rs:");
    println!("   use dumq_mqtt::logging::init_simple_file_logging;");
    println!("   let _handle = init_simple_file_logging(\"debug.log\", log::LevelFilter::Debug).unwrap();");
    println!();
    println!("2. Run your application with logging:");
    println!("   cargo run --example file_logging_example");
    println!();
    println!("3. Check the generated log files for debugging information.");
}
