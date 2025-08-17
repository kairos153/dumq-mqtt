//! # MQTT Codec Module
//! 
//! This module provides the binary encoding and decoding functionality for the MQTT wire protocol.
//! It handles the conversion between high-level MQTT packet structures and their binary
//! representation according to the MQTT specification.
//! 
//! ## Architecture
//! 
//! The codec follows a layered approach:
//! 
//! - **Application Layer**: Packet structs for high-level representation
//! - **Codec Layer**: MqttCodec for encoding/decoding logic
//! - **Binary Layer**: Bytes/BytesMut for efficient buffer management
//! 
//! ## Module Structure
//! 
//! - **`core`**: Main MqttCodec implementation and core functionality
//! - **`connect`**: Connect and ConnAck packet encoding/decoding
//! - **`publish`**: Publish, PubAck, PubRec, PubRel, PubComp packet handling
//! - **`subscribe`**: Subscribe, SubAck, Unsubscribe, UnsubAck packet handling
//! - **`utils`**: Utility functions for string, bytes, and length encoding/decoding
//! - **`properties`**: MQTT 5.0 properties encoding/decoding
//! 
//! ## Usage Examples
//! 
//! ```rust
//! use dumq_mqtt::codec::MqttCodec;
//! use dumq_mqtt::types::*;
//! use bytes::BytesMut;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create codec for MQTT 3.1.1
//!     let codec = MqttCodec::new(4);
//! 
//!     // Create a packet
//!     let packet = Packet {
//!         header: PacketHeader {
//!             packet_type: PacketType::Connect,
//!             dup: false,
//!             qos: 0,
//!             retain: false,
//!             remaining_length: 0,
//!         },
//!         payload: PacketPayload::Connect(ConnectPacket {
//!             protocol_name: "MQTT".to_string(),
//!             protocol_version: 4,
//!             clean_session: true,
//!             will_flag: false,
//!             will_qos: 0,
//!             will_retain: false,
//!             password_flag: false,
//!             username_flag: false,
//!             keep_alive: 60,
//!             client_id: "test_client".to_string(),
//!             will_topic: None,
//!             will_message: None,
//!             username: None,
//!             password: None,
//!             properties: None,
//!         }),
//!     };
//! 
//!     // Encode packet to binary
//!     let binary = codec.encode(&packet)?;
//!     println!("Encoded packet: {:?}", binary);
//! 
//!     // Decode binary back to packet
//!     let mut buffer = bytes::BytesMut::from(&binary[..]);
//!     let decoded = codec.decode(&mut buffer)?;
//! 
//!     match decoded {
//!         Some(packet) => println!("Decoded packet: {:?}", packet),
//!         None => println!("Incomplete packet data"),
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod core;
pub mod connect;
pub mod publish;
pub mod subscribe;
pub mod utils;
pub mod properties;

// Re-export main types
pub use core::MqttCodec;
