# DumQ MQTT Library

A comprehensive MQTT protocol library for Rust supporting both MQTT 3.1.1 and MQTT 5.0.

## Features

- **MQTT 3.1.1 Protocol Support**: Full implementation of MQTT 3.1.1 specification
- **MQTT 5.0 Protocol Support**: Full implementation of MQTT 5.0 specification (in progress)
- **Async/Await Support**: Built on top of Tokio for high-performance async operations
- **Client and Server**: Both client and server implementations included
- **QoS Levels**: Support for QoS 0, 1, and 2
- **Authentication**: Username/password authentication support
- **Session Management**: Persistent and clean session support
- **Topic Filtering**: Wildcard topic support (# and +)
- **Retained Messages**: Full support for retained messages with automatic delivery to new subscribers
- **Will Messages**: Last Will and Testament support
- **Keep Alive**: Automatic keep-alive mechanism

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dumq-mqtt = "0.1.0"
```

## Quick Start

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
    let publish_options = PublishOptions::new("test/topic", "Hello, MQTT!")
        .qos(QoS::AtLeastOnce);
    
    client.publish(publish_options).await?;

    // Receive messages
    while let Some(message) = client.recv().await? {
        println!("Received: {:?}", message);
    }

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