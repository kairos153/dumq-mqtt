//! # DumQ MQTT Library
//! 
//! A comprehensive, high-performance MQTT protocol library written in Rust, supporting both MQTT 3.1.1 and MQTT 5.0 specifications.
//! 
//! ## Overview
//! 
//! DumQ MQTT is a robust and feature-rich MQTT implementation designed for building scalable IoT applications,
//! messaging systems, and real-time communication platforms. The library provides both client and server
//! implementations with full protocol compliance and excellent performance characteristics.
//! 
//! ## Key Features
//! 
//! ### Protocol Support
//! - **MQTT 3.1.1**: Full compliance with MQTT 3.1.1 specification
//! - **MQTT 5.0**: Complete support for MQTT 5.0 features including properties, user properties, and enhanced error handling
//! - **Protocol Negotiation**: Automatic protocol version detection and negotiation
//! 
//! ### Core Functionality
//! - **QoS Levels**: Support for all three QoS levels (0, 1, 2)
//! - **Message Persistence**: Configurable message retention and delivery guarantees
//! - **Session Management**: Clean and persistent session handling
//! - **Topic Management**: Wildcard subscription support and topic validation
//! 
//! ### Performance & Scalability
//! - **Async/Await**: Built on Tokio for high-performance asynchronous I/O
//! - **Connection Pooling**: Efficient connection management for high-throughput scenarios
//! - **Memory Efficient**: Zero-copy operations where possible, optimized buffer management
//! - **High Concurrency**: Designed for handling thousands of concurrent connections
//! 
//! ### Security & Reliability
//! - **Authentication**: Username/password and certificate-based authentication
//! - **TLS/SSL Support**: Encrypted communication (planned)
//! - **Error Handling**: Comprehensive error types with detailed diagnostics
//! - **Connection Resilience**: Automatic reconnection and keep-alive management
//! 
//! ## Architecture
//! 
//! The library is organized into several core modules:
//! 
//! - **`client`**: MQTT client implementation with connection management and message handling
//! - **`server`**: MQTT broker/server implementation supporting multiple client connections
//! - **`protocol`**: Protocol-level abstractions, QoS handling, and connection options
//! - **`codec`**: Binary packet encoding/decoding for MQTT wire protocol
//! - **`types`**: Core data structures representing MQTT packets and messages
//! - **`error`**: Comprehensive error handling and result types
//! 
//! ## Quick Start
//! 
//! ### Basic Client Usage
//! 
//! ```rust,no_run
//! use dumq_mqtt::client::{Client, ClientConfig};
//! use dumq_mqtt::protocol::{ConnectOptions, PublishOptions, QoS};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client configuration
//!     let config = ClientConfig::new("localhost:1883")
//!         .connect_timeout(std::time::Duration::from_secs(30))
//!         .keep_alive_interval(std::time::Duration::from_secs(60));
//!     
//!     let client = Client::new(config);
//!     
//!     // Configure connection options
//!     let options = ConnectOptions::new("test_client")
//!         .clean_session(true)
//!         .username("user")
//!         .password("pass");
//!     
//!     // Connect to broker
//!     let mut client = client.connect(options).await?;
//!     
//!     // Subscribe to topics
//!     client.subscribe("sensors/+/temperature", QoS::AtLeastOnce).await?;
//!     
//!     // Publish messages
//!     let publish_opts = dumq_mqtt::protocol::PublishOptions::new("sensors/room1/temperature", "22.5")
//!         .qos(QoS::AtLeastOnce);
//!     client.publish(publish_opts).await?;
//!     
//!     // Receive messages
//!     while let Some(message) = client.recv().await? {
//!         println!("Received: {:?}", message);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ### Basic Server Usage
//! 
//! ```rust,no_run
//! use dumq_mqtt::server::{Server, ServerConfig};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ServerConfig::new("0.0.0.0:1883")
//!         .max_connections(1000)
//!         .allow_anonymous(true);
//!     
//!     let mut server = Server::new(config);
//!     
//!     // Start the server
//!     server.start().await?;
//!     
//!     // Keep the server running
//!     tokio::signal::ctrl_c().await?;
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Advanced Features
//! 
//! ### MQTT 5.0 Properties
//! 
//! ```rust,no_run
//! use dumq_mqtt::types::ConnectProperties;
//! 
//! // Note: This is a conceptual example - actual implementation may vary
//! println!("Setting up MQTT 5.0 properties");
//! println!("Session expiry: 3600 seconds");
//! println!("Receive maximum: 100");
//! println!("Max packet size: 1MB");
//! ```
//! 
//! ### Custom Message Handlers
//! 
//! ```rust,no_run
//! use dumq_mqtt::client::MessageHandler;
//! use dumq_mqtt::types::Message;
//! 
//! // Note: This is a conceptual example - actual implementation may vary
//! println!("Setting up message handler");
//! println!("Message handler configured for topic processing");
//! ```
//! 
//! ## Performance Considerations
//! 
//! - **Connection Pooling**: Reuse connections when possible to reduce overhead
//! - **Batch Operations**: Use batch publish/subscribe operations for high-throughput scenarios
//! - **Memory Management**: Configure appropriate buffer sizes based on your message sizes
//! - **Async Operations**: Leverage async/await for non-blocking I/O operations
//! 
//! ## Error Handling
//! 
//! The library provides comprehensive error handling with specific error types:
//! 
//! ```rust
//! use dumq_mqtt::error::{Error, Result};
//! 
//! // Note: This is a conceptual example - actual error handling depends on implementation
//! println!("Handling connection errors");
//! println!("Connection failed: Connection refused");
//! println!("Authentication failed: Invalid credentials");
//! println!("Connection timed out: Network timeout");
//! ```
//! 
//! ## Contributing
//! 
//! Contributions are welcome! Please see the project repository for contribution guidelines.
//! 
//! ## License
//! 
//! This project is licensed under the MIT License - see the LICENSE file for details.

pub mod client;
pub mod server;
pub mod protocol;
pub mod codec;
pub mod error;
pub mod types;
pub mod logging;

// Re-export main client components
pub use client::{Client, ClientConfig, ClientConnection, ConnectionState, MessageHandler};
pub use server::Server;
pub use error::{Error, Result};
pub use protocol::{ConnectOptions, QoS, RetainFlag, DupFlag};
pub use types::{Packet, PacketType, Message, TopicFilter};

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Supported MQTT protocol versions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolVersion {
    /// MQTT 3.1
    V3_1 = 3,
    /// MQTT 3.1.1
    V3_1_1 = 4,
    /// MQTT 5.0
    V5_0 = 5,
}

impl ProtocolVersion {
    /// Get the protocol name string
    pub fn protocol_name(&self) -> &'static str {
        match self {
            ProtocolVersion::V3_1 => "MQIsdp",
            ProtocolVersion::V3_1_1 => "MQTT",
            ProtocolVersion::V5_0 => "MQTT",
        }
    }
    
    /// Check if this version supports MQTT 5.0 features
    pub fn is_v5(&self) -> bool {
        matches!(self, ProtocolVersion::V5_0)
    }
} 