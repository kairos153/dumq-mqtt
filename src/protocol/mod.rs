//! # Protocol Module
//! 
//! This module provides high-level protocol abstractions and utilities for the DumQ MQTT library.
//! It defines the core protocol concepts, connection options, and quality of service handling
//! that are used across both client and server implementations.
//! 
//! ## Overview
//! 
//! The protocol module serves as the bridge between the low-level packet handling and the
//! high-level application interface. It provides:
//! - **QoS Management**: Quality of Service level definitions and validation
//! - **Connection Options**: Comprehensive connection configuration for clients
//! - **Protocol Flags**: Retain and duplicate flags for message handling
//! - **Builder Patterns**: Fluent interfaces for configuration objects
//! - **Protocol Validation**: Automatic validation of protocol constraints
//! 
//! ## Core Concepts
//! 
//! ### Quality of Service (QoS)
//! 
//! MQTT defines three QoS levels that determine the reliability of message delivery:
//! 
//! - **`QoS::AtMostOnce` (0)**: Fire and forget - no acknowledgment
//! - **`QoS::AtLeastOnce` (1)**: At least once delivery with acknowledgment
//! - **`QoS::ExactlyOnce` (2)**: Exactly once delivery with two-phase acknowledgment
//! 
//! ### Connection Options
//! 
//! The `ConnectOptions` struct provides a comprehensive configuration interface:
//! 
//! - **Client Identification**: Unique client ID and session management
//! - **Authentication**: Username/password credentials
//! - **Will Message**: Last will and testament configuration
//! - **Keep Alive**: Connection heartbeat intervals
//! - **Protocol Version**: MQTT version selection
//! - **Properties**: MQTT 5.0 extended properties
//! 
//! ### Protocol Flags
//! 
//! - **`RetainFlag`**: Controls message retention on the broker
//! - **`DupFlag`**: Indicates duplicate message delivery attempts
//! 
//! ## Usage Examples
//! 
//! ### Basic QoS Usage
//! 
//! ```rust
//! use dumq_mqtt::protocol::QoS;
//! 
//! // Create QoS levels
//! let qos0 = QoS::AtMostOnce;
//! let qos1 = QoS::AtLeastOnce;
//! let qos2 = QoS::ExactlyOnce;
//! 
//! // Convert from numeric values
//! let qos = QoS::from_u8(1).unwrap();
//! assert_eq!(qos, QoS::AtLeastOnce);
//! 
//! // Use in message operations
//! match qos {
//!     QoS::AtMostOnce => println!("No acknowledgment required"),
//!     QoS::AtLeastOnce => println!("Single acknowledgment"),
//!     QoS::ExactlyOnce => println!("Two-phase acknowledgment"),
//! }
//! ```
//! 
//! ### Connection Configuration
//! 
//! ```rust
//! use dumq_mqtt::protocol::{ConnectOptions, QoS};
//! use std::time::Duration;
//! 
//! // Basic connection
//! let options = ConnectOptions::new("my_client")
//!     .clean_session(true)
//!     .keep_alive(Duration::from_secs(60));
//! 
//! // With authentication
//! let options = ConnectOptions::new("my_client")
//!     .username("user")
//!     .password("pass")
//!     .clean_session(false);
//! 
//! // With will message
//! let options = ConnectOptions::new("my_client")
//!     .will("status/offline", "Client disconnected", QoS::AtLeastOnce, true)
//!     .keep_alive(Duration::from_secs(30));
//! 
//! // MQTT 5.0 with properties
//! let options = ConnectOptions::new("my_client")
//!     .protocol_version(5)
//!     .clean_session(true);
//! ```
//! 
//! ### Protocol Flag Usage
//! 
//! ```rust
//! use dumq_mqtt::protocol::{RetainFlag, DupFlag};
//! 
//! // Retain flag
//! let retain = RetainFlag::Retained;
//! let not_retain = RetainFlag::NotRetained;
//! 
//! // Duplicate flag
//! let duplicate = DupFlag::Duplicate;
//! let not_duplicate = DupFlag::NotDuplicate;
//! 
//! // Use in message operations
//! if retain == RetainFlag::Retained {
//!     println!("Message will be retained on the broker");
//! }
//! 
//! if duplicate == DupFlag::Duplicate {
//!     println!("This is a duplicate message delivery");
//! }
//! ```
//! 
//! ### Advanced Configuration
//! 
//! ```rust
//! use dumq_mqtt::protocol::{ConnectOptions, QoS};
//! use std::time::Duration;
//! 
//! // Complex configuration with all options
//! let options = ConnectOptions::new("iot_device_001")
//!     .clean_session(false)
//!     .keep_alive(Duration::from_secs(120))
//!     .username("device_user")
//!     .password("secure_password")
//!     .will("devices/001/status", "offline", QoS::AtLeastOnce, true)
//!     .protocol_version(5);
//! 
//! // Validate configuration
//! if options.keep_alive.as_secs() < 30 {
//!     eprintln!("Warning: Keep alive interval is very short");
//! }
//! 
//! if options.client_id.is_empty() {
//!     eprintln!("Error: Client ID cannot be empty");
//! }
//! ```
//! 
//! ## Builder Pattern
//! 
//! All configuration objects use the builder pattern for clean, readable configuration:
//! 
//! ```rust
//! use dumq_mqtt::protocol::{ConnectOptions, QoS};
//! 
//! let options = ConnectOptions::new("client_id")
//!     .clean_session(true)
//!     .keep_alive(std::time::Duration::from_secs(60))
//!     .username("user")
//!     .password("pass")
//!     .will("will/topic", "will message", QoS::AtLeastOnce, false)
//!     .protocol_version(4);
//! ```
//! 
//! ## Protocol Validation
//! 
//! The module automatically validates protocol constraints:
//! 
//! - **QoS Levels**: Ensures QoS values are within valid range (0-2)
//! - **Client ID**: Validates client identifier format and length
//! - **Keep Alive**: Ensures reasonable keep-alive intervals
//! - **Protocol Version**: Validates MQTT version compatibility
//! 
//! ## Performance Considerations
//! 
//! 1. **Reuse Options**: Create `ConnectOptions` once and reuse for multiple connections
//! 2. **Minimal Cloning**: Use references when possible to avoid unnecessary copying
//! 3. **Validation**: Validate options early to avoid runtime errors
//! 4. **Default Values**: Use sensible defaults to reduce configuration overhead
//! 
//! ## Error Handling
//! 
//! The module provides clear error messages for invalid configurations:
//! 
//! ```rust
//! // QoS validation example
//! let qos_level = 5;
//! if qos_level <= 2 {
//!     println!("Valid QoS: {}", qos_level);
//! } else {
//!     eprintln!("Invalid QoS level: {} (must be 0-2)", qos_level);
//! }
//! ```
//! 
//! ## Testing
//! 
//! The module includes comprehensive tests for:
//! - QoS level validation and conversion
//! - Connection options configuration
//! - Protocol flag handling
//! - Builder pattern functionality
//! - Protocol constraint validation
//! 
//! Run tests with:
//! 
//! ```bash
//! cargo test --package dumq-mqtt --lib protocol
//! ```

// Re-export all public items from submodules
pub use qos::QoS;
pub use flags::{RetainFlag, DupFlag};
pub use connect::ConnectOptions;
pub use subscribe::SubscribeOptions;
pub use publish::PublishOptions;
pub use reason_codes::ReasonCode;
pub use constants::*;

// Submodules
mod qos;
mod flags;
mod connect;
mod subscribe;
mod publish;
mod reason_codes;
mod constants;
