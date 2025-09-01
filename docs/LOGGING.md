# DumQ MQTT Logging Guide

This guide explains how to use the file logging functionality in DumQ MQTT for debugging purposes.

## Overview

DumQ MQTT provides comprehensive logging capabilities that can be configured to write logs to files, making it easier to debug MQTT applications. The logging system supports multiple log levels, file rotation, and both console and file output.

## Quick Start

### Basic File Logging

The simplest way to enable file logging is to use the `init_simple_file_logging` function:

```rust
use dumq_mqtt::logging::init_simple_file_logging;

fn main() {
    // Initialize logging to write to both console and file
    let _handle = init_simple_file_logging(
        "debug.log",
        log::LevelFilter::Debug,
    ).unwrap();
    
    // Your MQTT code here...
    log::info!("Application started");
    log::debug!("Debug information");
}
```

### Directory-Based Logging

For more organized logging, use the `init_file_logging` function which creates a logs directory:

```rust
use dumq_mqtt::logging::init_file_logging;

fn main() {
    // This will create a 'logs' directory and write to 'logs/dumq_mqtt.log'
    let _handle = init_file_logging(
        "logs",
        "dumq_mqtt.log",
        log::LevelFilter::Info,
    ).unwrap();
    
    // Your MQTT code here...
}
```

### Environment-Based Logging

For flexible logging configuration, use `init_env_logging`:

```rust
use dumq_mqtt::logging::init_env_logging;

fn main() {
    // This respects the RUST_LOG environment variable
    let _handle = init_env_logging(
        Some("app.log"),
        log::LevelFilter::Debug,
    ).unwrap();
    
    // Your MQTT code here...
}
```

## Log Levels

The logging system supports the following log levels (from most to least verbose):

- **trace**: Very detailed debugging information
- **debug**: General debugging information  
- **info**: General information about program execution
- **warn**: Warning messages
- **error**: Error messages

## Environment Variables

You can control logging behavior using the `RUST_LOG` environment variable:

```bash
# Enable all debug logs
export RUST_LOG=debug

# Enable only dumq_mqtt debug logs
export RUST_LOG=dumq_mqtt=debug

# Enable specific module debug logs
export RUST_LOG=dumq_mqtt::server=debug,dumq_mqtt::client=info

# Enable only info and above
export RUST_LOG=info

# Enable only warnings and errors
export RUST_LOG=warn

# Enable only errors
export RUST_LOG=error
```

## Examples

### Server with File Logging

```rust
use dumq_mqtt::server::{Server, ServerConfig};
use dumq_mqtt::logging::init_simple_file_logging;
use dumq_mqtt::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize file logging
    let _handle = init_simple_file_logging(
        "server.log",
        log::LevelFilter::Debug,
    ).unwrap();
    
    log::info!("Starting MQTT server...");
    
    let config = ServerConfig::new("127.0.0.1:1883")
        .max_connections(100)
        .allow_anonymous(true);
    
    let mut server = Server::new(config);
    
    log::info!("MQTT server configured and ready to start");
    server.start().await?;
    
    Ok(())
}
```

### Client with File Logging

```rust
use dumq_mqtt::client::{Client, ClientConfig};
use dumq_mqtt::protocol::ConnectOptions;
use dumq_mqtt::logging::init_file_logging;
use dumq_mqtt::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize directory-based logging
    let _handle = init_file_logging(
        "logs",
        "client.log",
        log::LevelFilter::Info,
    ).unwrap();
    
    log::info!("Starting MQTT client...");
    
    let config = ClientConfig::new("localhost:1883");
    let client = Client::new(config);
    
    let options = ConnectOptions::new("test_client")
        .clean_session(true);
    
    log::info!("Connecting to MQTT broker...");
    let mut client = client.connect(options).await?;
    
    log::info!("Successfully connected to MQTT broker");
    
    // Your client operations here...
    
    Ok(())
}
```

## Log File Formats

Log entries are formatted with the following pattern:

```
{timestamp} [{level}] {target} - {message}
```

Example:
```
2024-01-15 14:30:45.123 [INFO] dumq_mqtt::server - Starting MQTT server on 127.0.0.1:1883
2024-01-15 14:30:45.124 [DEBUG] dumq_mqtt::codec - Decoding packet, buffer size: 48
2024-01-15 14:30:45.125 [ERROR] dumq_mqtt::client - Connection failed: Connection refused
```

## File Rotation

The advanced logging configuration supports automatic file rotation:

- Files are rotated when they reach 10MB
- Old log files are automatically deleted
- This helps prevent log files from growing too large

## Best Practices

### 1. Choose Appropriate Log Levels

- Use `debug` for detailed debugging information
- Use `info` for general application flow
- Use `warn` for potentially problematic situations
- Use `error` for actual errors that need attention

### 2. Organize Log Files

```rust
// For development
let _handle = init_simple_file_logging("debug.log", log::LevelFilter::Debug).unwrap();

// For production
let _handle = init_file_logging("logs", "production.log", log::LevelFilter::Info).unwrap();
```

### 3. Use Environment Variables for Flexibility

```bash
# Development
export RUST_LOG=debug

# Production
export RUST_LOG=info

# Troubleshooting
export RUST_LOG=dumq_mqtt::server=debug,dumq_mqtt::client=warn
```

### 4. Monitor Log File Sizes

Regularly check log file sizes and implement rotation if needed:

```rust
// Check log file size periodically
use std::fs;

if let Ok(metadata) = fs::metadata("debug.log") {
    if metadata.len() > 100 * 1024 * 1024 { // 100MB
        log::warn!("Log file is getting large, consider rotation");
    }
}
```

## Troubleshooting

### Log Files Not Created

1. Check if the directory exists and is writable
2. Ensure the logging initialization is called before any log statements
3. Verify that the log level is appropriate

### Too Much Log Output

1. Increase the log level (e.g., from `debug` to `info`)
2. Use environment variables to filter specific modules
3. Use the `init_env_logging` function for more control

### Performance Issues

1. Use `info` or higher log levels in production
2. Consider using file rotation to manage log file sizes
3. Monitor disk space usage

## Integration with Existing Code

If you already have an MQTT application, you can easily add file logging:

```rust
// Before (console only)
env_logger::init();

// After (console + file)
use dumq_mqtt::logging::init_simple_file_logging;
let _handle = init_simple_file_logging("app.log", log::LevelFilter::Debug).unwrap();
```

## Running the Examples

To see file logging in action, run the provided examples:

```bash
# Run the file logging example
cargo run --example file_logging_example

# Check the generated log files
ls -la *.log logs/
```

This will create several log files demonstrating different logging configurations.
