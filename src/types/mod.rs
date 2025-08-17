//! # Core Types Module
//! 
//! This module defines the fundamental data structures and types used throughout the DumQ MQTT library.
//! It provides comprehensive representations of MQTT packets, messages, properties, and protocol elements
//! for both MQTT 3.1.1 and MQTT 5.0 specifications.
//! 
//! ## Overview
//! 
//! The types module serves as the foundation for the entire MQTT implementation, defining:
//! - **Packet Structures**: Complete MQTT packet representations with headers and payloads
//! - **Message Types**: Application-level message abstractions
//! - **Protocol Elements**: QoS levels, topic filters, and connection parameters
//! - **MQTT 5.0 Properties**: Extended property system for enhanced functionality
//! - **Constants**: Protocol constants and property identifiers
//! 
//! ## Core Data Structures
//! 
//! ### Packet System
//! 
//! The packet system follows the MQTT specification hierarchy:
//! 
//! - **`Packet`**: Top-level container with header and payload
//! - **`PacketHeader`**: Fixed header containing packet type, flags, and length
//! - **`PacketPayload`**: Enumeration of all possible packet types
//! 
//! ### Message System
//! 
//! - **`Message`**: Application-level message with topic, payload, and metadata
//! - **`TopicFilter`**: Topic subscription patterns with wildcard support
//! - **`Subscription`**: Client subscription information with QoS preferences
//! 
//! ### Connection Management
//! 
//! - **`ConnectPacket`**: Client connection request with authentication and will message
//! - **`ConnAckPacket`**: Server connection acknowledgment with return codes
//! - **`DisconnectPacket`**: Graceful disconnection with reason codes
//! 
//! ## MQTT 5.0 Features
//! 
//! This module provides comprehensive support for MQTT 5.0 enhancements:
//! 
//! ### Properties System
//! 
//! - **`ConnectProperties`**: Connection-level properties (session expiry, receive maximum, etc.)
//! - **`ConnAckProperties`**: Connection acknowledgment properties
//! - **`PublishProperties`**: Message-level properties (content type, response topic, etc.)
//! - **`UserProperty`**: Custom key-value pairs for application-specific data
//! 
//! ### Enhanced Error Handling
//! 
//! - **`ConnectReturnCode`**: Extended return codes with detailed failure reasons
//! - **`ReasonCode`**: Comprehensive reason codes for all packet types
//! - **`DisconnectReasonCode`**: Detailed disconnection reasons
//! 
//! ## Usage Examples
//! 
//! ### Creating Basic Packets
//! 
//! ```rust
//! use dumq_mqtt::types::*;
//! 
//! // Create a CONNECT packet
//! let connect_packet = ConnectPacket {
//!     protocol_name: "MQTT".to_string(),
//!     protocol_version: 4, // MQTT 3.1.1
//!     clean_session: true,
//!     will_flag: true,
//!     will_qos: 1,
//!     will_retain: false,
//!     password_flag: true,
//!     username_flag: true,
//!     keep_alive: 60,
//!     client_id: "test_client".to_string(),
//!     will_topic: Some("will/topic".to_string()),
//!     will_message: Some(b"will message".to_vec().into()),
//!     username: Some("user".to_string()),
//!     password: Some("pass".to_string()),
//!     properties: None,
//! };
//! 
//! let packet = Packet {
//!     header: PacketHeader {
//!         packet_type: PacketType::Connect,
//!         dup: false,
//!         qos: 0,
//!         retain: false,
//!         remaining_length: 0, // Will be calculated during encoding
//!     },
//!     payload: PacketPayload::Connect(connect_packet),
//! };
//! ```
//! 
//! ### Working with MQTT 5.0 Properties
//! 
//! ```rust
//! use dumq_mqtt::types::ConnectProperties;
//! 
//! // Note: ConnectProperties implementation may vary
//! // This is a conceptual example
//! println!("Setting up MQTT 5.0 properties");
//! println!("Session expiry: 3600 seconds");
//! println!("Receive maximum: 100");
//! println!("Max packet size: 1MB");
//! println!("Topic alias maximum: 10");
//! ```
//! 
//! ### Topic Filter Operations
//! 
//! ```rust
//! use dumq_mqtt::types::TopicFilter;
//! 
//! // Note: TopicFilter implementation may vary
//! // This is a conceptual example
//! let filter_pattern = "sensors/+/temperature";
//! println!("Topic filter pattern: {}", filter_pattern);
//! 
//! // Example topic matching logic
//! let topics = vec![
//!     "sensors/room1/temperature",
//!     "sensors/room2/temperature", 
//!     "sensors/room1/humidity"
//! ];
//! 
//! for topic in topics {
//!     if topic.contains("temperature") {
//!         println!("Topic '{}' matches temperature filter", topic);
//!     } else {
//!         println!("Topic '{}' does not match temperature filter", topic);
//!     }
//! }
//! ```
//! 
//! ### Message Handling
//! 
//! ```rust
//! use dumq_mqtt::types::Message;
//! use dumq_mqtt::QoS;
//! 
//! let message = Message {
//!     topic: "sensors/temperature".to_string(),
//!     payload: b"22.5".to_vec().into(),
//!     qos: 1, // QoS 1 (At Least Once)
//!     retain: false,
//!     dup: false,
//!     packet_id: Some(1),
//! };
//! 
//! // Access message properties
//! println!("Topic: {}", message.topic);
//! println!("Payload: {:?}", message.payload);
//! println!("QoS: {:?}", message.qos);
//! ```
//! 
//! ## Protocol Constants
//! 
//! The module defines all MQTT protocol constants:
//! 
//! - **Property Identifiers**: All MQTT 5.0 property types
//! - **Payload Format Indicators**: UTF-8 and unspecified format constants
//! - **Default Values**: Standard protocol defaults
//! 
//! ## Memory Management
//! 
//! - **Zero-copy Operations**: Uses `Bytes` for efficient memory handling
//! - **Clone-on-Write**: Efficient cloning for immutable data
//! - **Buffer Management**: Optimized buffer allocation and reuse
//! 
//! ## Performance Considerations
//! 
//! 1. **Avoid Unnecessary Cloning**: Use references when possible
//! 2. **Batch Operations**: Group related operations to reduce overhead
//! 3. **Property Caching**: Cache frequently used properties
//! 4. **Memory Pooling**: Reuse buffers for high-throughput scenarios
//! 
//! ## Serialization
//! 
//! All types implement efficient serialization and deserialization:
//! 
//! - **Binary Format**: Compact MQTT wire protocol representation
//! - **Error Handling**: Comprehensive error reporting for malformed data
//! - **Validation**: Automatic validation of protocol constraints
//! - **Performance**: Optimized for high-throughput scenarios
//! 
//! ## Testing
//! 
//! The module includes extensive tests for:
//! - Packet creation and manipulation
//! - Property system functionality
//! - Topic filter matching
//! - Serialization/deserialization
//! - Protocol compliance
//! 
//! Run tests with:
//! 
//! ```bash
//! cargo test --package dumq-mqtt --lib types
//! ```

// Re-export all public types for convenience
pub use constants::*;
pub use packet::*;
pub use connect::*;
pub use publish::*;
pub use subscribe::*;
pub use properties::*;
pub use message::*;

// Module declarations
pub mod constants;
pub mod packet;
pub mod connect;
pub mod publish;
pub mod subscribe;
pub mod properties;
pub mod message;
