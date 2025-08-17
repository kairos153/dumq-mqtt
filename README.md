# DumQ MQTT Library

A comprehensive MQTT protocol library for Rust supporting both MQTT 3.1.1 and MQTT 5.0.

## Features

- **Modular Architecture**: Well-organized code structure with separate modules for different packet types
- **Codec Organization**: Structured MQTT codec with dedicated modules for:
  - Core codec functionality (`core.rs`)
  - Connect/ConnAck packets (`connect.rs`)
  - Publish-related packets (`publish.rs`)
  - Subscribe-related packets (`subscribe.rs`)
  - MQTT 5.0 properties (`properties.rs`)
  - Utility functions (`utils.rs`)

- **MQTT 3.1.1 Protocol Support**: Full implementation of MQTT 3.1.1 specification
- **MQTT 5.0 Protocol Support**: Full implementation of MQTT 5.0 specification (in progress)
- **MQTT v5 Publish Properties**: Complete implementation of MQTT v5 publish properties including:
  - Payload Format Indicator (UTF-8/Unspecified)
  - Message Expiry Interval
  - Topic Alias
  - Response Topic
  - Correlation Data
  - Subscription Identifier
  - Content Type
  - User Properties
- **Async/Await Support**: Built on top of Tokio for high-performance async operations
- **Client and Server**: Both client and server implementations included
- **QoS Levels**: Support for QoS 0, 1, and 2
- **Authentication**: Username/password authentication support
- **Session Management**: Persistent and clean session support
- **Topic Filtering**: Wildcard topic support (# and +)
- **Retained Messages**: Full support for retained messages with automatic delivery to new subscribers
- **Will Messages**: Last Will and Testament support
- **Keep Alive**: Automatic keep-alive mechanism
- **Message Routing**: Efficient message routing between publishers and subscribers
- **Real-time Communication**: Low-latency message delivery with async processing

## Project Structure

The library is organized into several core modules for better maintainability and code organization:

```
src/
├── codec/           # MQTT packet encoding/decoding
│   ├── core.rs      # Main MqttCodec implementation
│   ├── connect.rs   # Connect/ConnAck packet handling
│   ├── publish.rs   # Publish-related packet handling
│   ├── subscribe.rs # Subscribe-related packet handling
│   ├── properties.rs # MQTT 5.0 properties handling
│   ├── utils.rs     # Utility functions (strings, bytes, length)
│   └── mod.rs       # Module declarations and re-exports
├── types/           # Core data structures
├── client/          # MQTT client implementation
├── server/          # MQTT server implementation
├── protocol/        # Protocol-level abstractions
└── error/           # Error handling and result types
```

### Codec Module Benefits

- **Maintainability**: Each packet type has its own dedicated module
- **Readability**: Clear separation of concerns makes code easier to understand
- **Testability**: Individual modules can be tested in isolation
- **Extensibility**: Easy to add new packet types or modify existing ones
- **Collaboration**: Multiple developers can work on different packet types simultaneously

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dumq-mqtt = "0.1.0"
```

## Quick Start

### Message Exchange Example

Here's a complete example showing how to set up a server and exchange messages between clients:

```rust
use dumq_mqtt::{
    server::{Server, ServerConfig},
    client::{Client, ClientConfig},
    protocol::{QoS, ConnectOptions, PublishOptions},
    types::Message,
};
use log::{info, error};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Start MQTT server
    let server_config = ServerConfig::new("127.0.0.1:1883")
        .allow_anonymous(true);
    let mut server = Server::new(server_config);
    
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.start().await {
            error!("Server error: {}", e);
        }
    });

    sleep(Duration::from_millis(100)).await;

    // Create subscriber
    let mut subscriber = Client::new(ClientConfig::new("127.0.0.1:1883"));
    subscriber = subscriber.connect(ConnectOptions::new("subscriber").clean_session(true)).await?;
    subscriber.subscribe("hello/world", QoS::AtLeastOnce).await?;

    // Set message handler
    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let message_count_clone = message_count.clone();
    
    subscriber = subscriber.set_message_handler(move |message: Message| {
        info!("Received: {}", String::from_utf8_lossy(&message.payload));
        message_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    });

    // Start listening
    let subscriber_handle = tokio::spawn(async move {
        if let Err(e) = subscriber.listen().await {
            error!("Subscriber error: {}", e);
        }
    });

    sleep(Duration::from_millis(100)).await;

    // Create publisher and send message
    let mut publisher = Client::new(ClientConfig::new("127.0.0.1:1883"));
    publisher = publisher.connect(ConnectOptions::new("publisher").clean_session(true)).await?;

    let publish_options = PublishOptions {
        topic: "hello/world".to_string(),
        payload: b"Hello, MQTT World!".to_vec(),
        qos: QoS::AtLeastOnce,
        retain: false,
        dup: false,
        packet_id: None,
    };

    publisher.publish(publish_options).await?;
    info!("Message published");

    // Wait and check result
    sleep(Duration::from_secs(1)).await;
    let received_count = message_count.load(std::sync::atomic::Ordering::SeqCst);
    info!("Received {} messages", received_count);

    // Cleanup
    publisher.disconnect().await?;
    server_handle.abort();
    subscriber_handle.abort();

    Ok(())
}
```

### Client Example

```rust
use dumq_mqtt::client::{Client, ClientConfig};
use dumq_mqtt::protocol::{ConnectOptions, QoS, PublishOptions};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client configuration
    let config = ClientConfig::new("localhost:1883")
        .protocol_version(4); // MQTT 3.1.1

    // Create client
    let client = Client::new(config);

    // Connect to broker
    let options = ConnectOptions::new("my_client")
        .clean_session(true);
    
    let mut client = client.connect(options).await?;

    // Subscribe to a topic
    client.subscribe("test/topic", QoS::AtLeastOnce).await?;

    // Publish a message
    let publish_options = PublishOptions {
        topic: "test/topic".to_string(),
        payload: b"Hello, MQTT!".to_vec(),
        qos: QoS::AtLeastOnce,
        retain: false,
        dup: false,
        packet_id: None,
    };
    
    client.publish(publish_options).await?;

    // Receive messages
    while let Some(message) = client.recv().await? {
        println!("Received: {:?}", message);
    }

    Ok(())
}

### MQTT 5.0 Publish Properties Example

```rust
use dumq_mqtt::types::{
    PublishPacket, PublishProperties, 
    PAYLOAD_FORMAT_INDICATOR_UTF8
};
use bytes::Bytes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create PublishProperties with MQTT v5 features
    let properties = PublishProperties::new()
        .payload_format_indicator(PAYLOAD_FORMAT_INDICATOR_UTF8)
        .message_expiry_interval(3600) // 1 hour
        .topic_alias(123)
        .response_topic("response/topic".to_string())
        .correlation_data(Bytes::from("request-123"))
        .subscription_identifier(456)
        .content_type("application/json".to_string())
        .user_property("version".to_string(), "5.0".to_string())
        .user_property("priority".to_string(), "high".to_string());

    // Create PublishPacket with properties using builder pattern
    let payload = Bytes::from(r#"{"message": "Hello MQTT v5!", "timestamp": 1234567890}"#);
    let packet = PublishPacket::new("sensor/temperature".to_string(), payload)
        .payload_format_indicator(PAYLOAD_FORMAT_INDICATOR_UTF8)
        .message_expiry_interval(7200) // 2 hours
        .topic_alias(789)
        .response_topic("sensor/response".to_string())
        .correlation_data(Bytes::from("sensor-request-456"))
        .subscription_identifier(101)
        .content_type("application/json".to_string())
        .user_property("sensor_id".to_string(), "temp_001".to_string())
        .user_property("location".to_string(), "room_101".to_string());

    println!("Topic: {}", packet.topic_name);
    println!("Has Properties: {}", packet.has_properties());
    println!("Is UTF-8: {}", properties.is_utf8_payload());

    Ok(())
}
```

### Retain Message Example

```rust
use dumq_mqtt::client::{Client, ClientConfig};
use dumq_mqtt::protocol::{ConnectOptions, QoS, PublishOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("localhost:1883")
        .protocol_version(4);

    let client = Client::new(config);
    let mut client = client.connect(ConnectOptions::new("my_client").clean_session(true)).await?;

    // Publish a retained message
    let publish_options = PublishOptions::new("status/device1", "Online")
        .qos(QoS::AtLeastOnce)
        .retain(true);
    
    client.publish(publish_options).await?;

    // Subscribe to receive retained messages
    client.subscribe("status/+", QoS::AtLeastOnce).await?;

    while let Some(message) = client.recv().await? {
        if message.retain {
            println!("Received retained message: {}", String::from_utf8_lossy(&message.payload));
        }
    }

    Ok(())
}
```

### Server Example

```rust
use dumq_mqtt::server::{Server, ServerConfig, Authentication};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create authentication
    let auth = Authentication::new()
        .add_user("user", "password");

    // Create server configuration
    let config = ServerConfig::new("127.0.0.1:1883")
        .allow_anonymous(false)
        .authentication(auth);

    // Create and start server
    let mut server = Server::new(config);
    server.start().await?;

    Ok(())
}
```

## Examples

Run the examples:

```bash
# Client example
cargo run --example client_example

# Server example
cargo run --example server_example

# Retain message example
cargo run --example retain_message_example
```

## Protocol Support

### MQTT 3.1.1

- ✅ CONNECT/CONNACK
- ✅ PUBLISH/PUBACK
- ✅ SUBSCRIBE/SUBACK
- ✅ UNSUBSCRIBE/UNSUBACK
- ✅ PINGREQ/PINGRESP
- ✅ DISCONNECT
- ✅ QoS 0, 1, 2
- ✅ Username/Password Authentication
- ✅ Clean Session
- ✅ Keep Alive
- ✅ Will Messages
- ✅ Retained Messages

### MQTT 5.0 (In Progress)

- ✅ Basic packet structure
- ✅ CONNECT/CONNACK with properties
- ✅ PUBLISH with properties
- ✅ **Publish Properties** - Full implementation of MQTT v5 publish properties
- 🔄 Properties encoding/decoding
- 🔄 Reason codes
- 🔄 Topic aliases
- 🔄 Message expiry
- 🔄 Shared subscriptions
- 🔄 Subscription identifiers

## Configuration

### Client Configuration

```rust
let config = ClientConfig::new("localhost:1883")
    .connect_timeout(Duration::from_secs(30))
    .read_timeout(Duration::from_secs(30))
    .write_timeout(Duration::from_secs(30))
    .keep_alive_interval(Duration::from_secs(60))
    .max_packet_size(1024 * 1024)
    .protocol_version(4); // 4 for MQTT 3.1.1, 5 for MQTT 5.0
```

### Server Configuration

```rust
let config = ServerConfig::new("127.0.0.1:1883")
    .max_connections(1000)
    .max_packet_size(1024 * 1024)
    .protocol_version(4)
    .allow_anonymous(true)
    .authentication(auth);
```

## Error Handling

The library provides comprehensive error handling:

```rust
use dumq_mqtt::error::{Error, Result};

match client.connect(options).await {
    Ok(client) => {
        println!("Connected successfully");
    }
    Err(Error::Connection(msg)) => {
        eprintln!("Connection failed: {}", msg);
    }
    Err(Error::Authentication(msg)) => {
        eprintln!("Authentication failed: {}", msg);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Logging

The library uses the `log` crate for logging. Enable logging by setting the `RUST_LOG` environment variable:

```bash
export RUST_LOG=debug
cargo run --example client_example
```

## Codec Refactoring Summary

The MQTT codec has been refactored from a single large file (`codec.rs` - 2000+ lines) into a well-organized modular structure:

### Before (Single File)
- `src/codec.rs` - 2038 lines containing all codec functionality
- Difficult to navigate and maintain
- All packet types mixed together
- Hard to test individual components

### After (Modular Structure)
- `src/codec/core.rs` - Main codec implementation (380 lines)
- `src/codec/connect.rs` - Connect/ConnAck packets (205 lines)
- `src/codec/publish.rs` - Publish-related packets (243 lines)
- `src/codec/subscribe.rs` - Subscribe-related packets (187 lines)
- `src/codec/properties.rs` - MQTT 5.0 properties (392 lines)
- `src/codec/utils.rs` - Utility functions (190 lines)
- `src/codec/mod.rs` - Module organization (89 lines)

### Benefits Achieved
- **Reduced complexity**: Each module focuses on specific functionality
- **Better organization**: Logical grouping of related packet types
- **Improved maintainability**: Easier to locate and modify specific features
- **Enhanced testability**: Individual modules can be tested independently
- **Better collaboration**: Multiple developers can work on different areas
- **Cleaner imports**: Clear dependencies between modules

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Roadmap

- [ ] Complete MQTT 5.0 implementation
- [ ] TLS/SSL support
- [ ] Message persistence
- [ ] Cluster support
- [ ] Performance optimizations
- [ ] More comprehensive tests
- [ ] Documentation improvements
- [ ] WebSocket support
- [ ] Bridge functionality

## Acknowledgments

This library is inspired by the MQTT specification and various existing MQTT implementations. 