//! # DumQ MQTT Library
//! 
//! A comprehensive MQTT protocol library supporting both MQTT 3.1.1 and MQTT 5.0.
//! 
//! ## Features
//! 
//! - MQTT 3.1.1 protocol support
//! - MQTT 5.0 protocol support
//! - Async/await support with Tokio
//! - Client and server implementations
//! - QoS levels 0, 1, and 2
//! - TLS/SSL support (planned)
//! - Message persistence (planned)
//! 
//! ## Example
//! 
//! ```rust,no_run
//! use dumq_mqtt::client::{Client, ClientConfig};
//! use dumq_mqtt::protocol::{ConnectOptions, QoS};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ClientConfig::new("localhost:1883");
//!     let client = Client::new(config);
//!     
//!     let options = ConnectOptions::new("test_client")
//!         .clean_session(true);
//!     
//!     let mut client = client.connect(options).await?;
//!     
//!     client.subscribe("test/topic", QoS::AtLeastOnce).await?;
//!     
//!     while let Some(message) = client.recv().await? {
//!         println!("Received: {:?}", message);
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod server;
pub mod protocol;
pub mod codec;
pub mod error;
pub mod types;

pub use client::Client;
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