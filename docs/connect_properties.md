# MQTT Connect Properties

This document explains how to use the MQTT 5.0 Connect Properties functionality in the dumq-mqtt library.

## Overview

MQTT 5.0 introduces a properties system that allows clients and servers to exchange additional information during the connection process. The Connect Properties provide enhanced control over connection behavior, session management, and authentication.

## Available Properties

### Session Management
- **Session Expiry Interval**: Controls how long the server should keep the session after disconnection
- **Receive Maximum**: Limits the number of unacknowledged QoS 1 and 2 messages the client can receive

### Packet and Topic Limits
- **Maximum Packet Size**: Sets the maximum size of packets the client can receive
- **Topic Alias Maximum**: Sets the maximum number of topic aliases the client can use

### Information Requests
- **Request Response Information**: Asks the server to include response information in the ConnAck
- **Request Problem Information**: Asks the server to include reason strings and user properties in error responses

### Authentication
- **Authentication Method**: Specifies the authentication method to use
- **Authentication Data**: Provides authentication data (e.g., tokens, certificates)

### Custom Properties
- **User Properties**: Key-value pairs for application-specific information

## Usage Examples

### Basic Connect Options (MQTT 3.1.1)

```rust
use dumq_mqtt::protocol::{ConnectOptions, QoS};
use std::time::Duration;

let options = ConnectOptions::new("client_id")
    .username("user")
    .password("pass")
    .keep_alive(Duration::from_secs(60));
```

### MQTT 5.0 with Properties

```rust
let mqtt5_options = ConnectOptions::new("mqtt5_client")
    .protocol_version(5)
    .session_expiry_interval(3600)        // 1 hour
    .receive_maximum(100)
    .max_packet_size(1_000_000)          // 1MB
    .topic_alias_maximum(20)
    .request_response_information(true)
    .request_problem_information(false)
    .user_property("client_type", "mobile")
    .user_property("version", "2.0.0")
    .authentication_method("JWT")
    .authentication_data(b"jwt_token_here");
```

### Properties Builder Pattern

The Connect Properties use a fluent builder pattern, allowing you to chain property settings:

```rust
let options = ConnectOptions::new("client_id")
    .protocol_version(5)
    .session_expiry_interval(7200)        // 2 hours
    .receive_maximum(50)
    .max_packet_size(512_000)            // 512KB
    .topic_alias_maximum(10)
    .request_response_information(true)
    .user_property("platform", "iOS")
    .user_property("app_version", "1.2.3");
```

## Property Details

### Session Expiry Interval
- **Type**: `u32` (seconds)
- **Default**: `None` (session expires when connection closes)
- **Range**: 0 to 4,294,967,295 seconds (about 136 years)
- **Use Case**: Persistent sessions that survive network interruptions

```rust
// Session expires after 1 hour of disconnection
.session_expiry_interval(3600)

// Session expires immediately when connection closes
.session_expiry_interval(0)

// Session never expires (use with caution)
.session_expiry_interval(u32::MAX)
```

### Receive Maximum
- **Type**: `u16`
- **Default**: `None` (no limit)
- **Range**: 1 to 65,535
- **Use Case**: Flow control for QoS 1 and 2 messages

```rust
// Allow up to 100 unacknowledged messages
.receive_maximum(100)

// Conservative setting for resource-constrained devices
.receive_maximum(10)
```

### Maximum Packet Size
- **Type**: `u32` (bytes)
- **Default**: `None` (no limit)
- **Range**: 1 to 268,435,455 bytes (256MB)
- **Use Case**: Memory management and bandwidth control

```rust
// Limit packets to 1MB
.max_packet_size(1_000_000)

// Conservative setting for embedded devices
.max_packet_size(16_384)  // 16KB
```

### Topic Alias Maximum
- **Type**: `u16`
- **Default**: `None` (no aliases)
- **Range**: 0 to 65,535
- **Use Case**: Reduce bandwidth for frequently used topics

```rust
// Allow up to 20 topic aliases
.topic_alias_maximum(20)

// Disable topic aliases
.topic_alias_maximum(0)
```

### User Properties
- **Type**: `HashMap<String, String>`
- **Default**: Empty
- **Use Case**: Application-specific metadata

```rust
.user_property("device_id", "sensor_001")
.user_property("location", "building_a")
.user_property("firmware_version", "v2.1.0")
```

## Protocol Version Considerations

- **MQTT 3.1.1 (v4)**: Properties are ignored, connection uses standard MQTT 3.1.1 behavior
- **MQTT 5.0 (v5)**: All properties are processed and sent to the server

```rust
// MQTT 3.1.1 - properties ignored
let options = ConnectOptions::new("client")
    .protocol_version(4)
    .session_expiry_interval(3600);  // This will be ignored

// MQTT 5.0 - properties processed
let options = ConnectOptions::new("client")
    .protocol_version(5)
    .session_expiry_interval(3600);  // This will be sent to server
```

## Best Practices

1. **Set Protocol Version First**: Always set the protocol version before adding properties
2. **Use Reasonable Limits**: Set packet size and receive maximum based on your device capabilities
3. **Session Management**: Use session expiry interval for applications that need to survive network interruptions
4. **User Properties**: Use user properties for application-specific metadata, not for core MQTT functionality
5. **Authentication**: Use authentication method and data for secure connections

## Error Handling

Properties are validated when creating the connection. Invalid values will cause connection failures:

```rust
// This will fail if the server doesn't support the specified limits
let options = ConnectOptions::new("client")
    .protocol_version(5)
    .max_packet_size(1_000_000_000);  // 1GB - likely too large
```

## Integration with Client

The Connect Properties are automatically used when creating a connection:

```rust
use dumq_mqtt::client::Client;

let options = ConnectOptions::new("client_id")
    .protocol_version(5)
    .session_expiry_interval(3600)
    .receive_maximum(100);

let client = Client::new(options);
// Properties are automatically included in the CONNECT packet
```

## Testing

You can test the Connect Properties functionality using the provided example:

```bash
cargo run --example connect_properties
```

This will demonstrate various property configurations and their effects.
