# MQTT ConnAck Properties

This document explains how to use the MQTT 5.0 ConnAck Properties functionality in the dumq-mqtt library.

## Overview

MQTT 5.0 ConnAck Properties allow servers to communicate additional information to clients during the connection acknowledgment process. These properties provide enhanced control over connection behavior, server capabilities, and error reporting.

## Available Properties

### Session Management
- **Session Expiry Interval**: How long the server will keep the session after disconnection
- **Receive Maximum**: Maximum number of unacknowledged QoS 1 and 2 messages the client can receive

### Server Capabilities
- **Max QoS**: Maximum QoS level supported by the server
- **Retain Available**: Whether the server supports retained messages
- **Wildcard Subscription Available**: Whether the server supports wildcard subscriptions
- **Shared Subscription Available**: Whether the server supports shared subscriptions
- **Subscription Identifiers Available**: Whether the server supports subscription identifiers

### Packet and Topic Limits
- **Maximum Packet Size**: Maximum size of packets the server can receive
- **Topic Alias Maximum**: Maximum number of topic aliases the server can use

### Server Information
- **Server Keep Alive**: Recommended keep alive interval for the client
- **Response Information**: Additional information about the server
- **Server Reference**: Alternative server to connect to

### Authentication
- **Authentication Method**: Authentication method to use
- **Authentication Data**: Authentication data (e.g., tokens, certificates)

### Error Reporting
- **Reason String**: Human-readable reason for connection status
- **Assigned Client Identifier**: Server-assigned client identifier

### Custom Properties
- **User Properties**: Key-value pairs for application-specific information

## Usage Examples

### Basic ConnAck Packet

```rust
use dumq_mqtt::types::{ConnAckPacket, ConnectReturnCode};

let connack = ConnAckPacket::new(ConnectReturnCode::Accepted)
    .session_present(true);
```

### ConnAck with Properties

```rust
use dumq_mqtt::types::{ConnAckPacket, ConnAckProperties, ConnectReturnCode};

let properties = ConnAckProperties::new()
    .session_expiry_interval(3600)        // 1 hour
    .receive_maximum(100)
    .max_qos(2)
    .retain_available(true)
    .max_packet_size(1_000_000)          // 1MB
    .topic_alias_maximum(20);

let connack = ConnAckPacket::new(ConnectReturnCode::Accepted)
    .session_present(false)
    .properties(properties);
```

### Server Capabilities

```rust
let server_capabilities = ConnAckProperties::new()
    .max_qos(1)                           // Only QoS 0 and 1 supported
    .retain_available(false)              // Retain not supported
    .wildcard_subscription_available(false)  // No wildcard support
    .shared_subscription_available(false)    // No shared subscription support
    .subscription_identifiers_available(false) // No subscription IDs
    .max_packet_size(256_000);            // 256KB limit

let connack = ConnAckPacket::new(ConnectReturnCode::Accepted)
    .session_present(true)
    .properties(server_capabilities);
```

### Error ConnAck with Properties

```rust
let error_properties = ConnAckProperties::new()
    .reason_string("Invalid credentials provided".to_string())
    .user_property("error_code".to_string(), "AUTH_001".to_string());

let error_connack = ConnAckPacket::new(ConnectReturnCode::BadUsernameOrPassword)
    .session_present(false)
    .properties(error_properties);
```

### Authentication Challenge

```rust
let auth_properties = ConnAckProperties::new()
    .authentication_method("CHALLENGE".to_string())
    .authentication_data(b"challenge_token_123".to_vec().into())
    .reason_string("Authentication required".to_string());

let auth_connack = ConnAckPacket::new(ConnectReturnCode::Accepted)
    .session_present(false)
    .properties(auth_properties);
```

### Server Redirection

```rust
let redirect_properties = ConnAckProperties::new()
    .server_reference("loadbalancer.example.com:1883".to_string())
    .reason_string("Please connect to the recommended server".to_string())
    .user_property("redirect_reason".to_string(), "load_balancing".to_string());

let redirect_connack = ConnAckPacket::new(ConnectReturnCode::UseAnotherServer)
    .session_present(false)
    .properties(redirect_properties);
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

### Max QoS
- **Type**: `u8`
- **Default**: `None` (all QoS levels supported)
- **Range**: 0 to 2
- **Use Case**: Server QoS capability advertisement

```rust
// Only QoS 0 and 1 supported
.max_qos(1)

// All QoS levels supported
.max_qos(2)
```

### Retain Available
- **Type**: `bool`
- **Default**: `None` (retain supported)
- **Use Case**: Server retain capability advertisement

```rust
// Retain not supported
.retain_available(false)

// Retain supported
.retain_available(true)
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

### Server Keep Alive
- **Type**: `u16` (seconds)
- **Default**: `None` (use client's keep alive)
- **Range**: 1 to 65,535 seconds
- **Use Case**: Server-recommended keep alive interval

```rust
// Server recommends 30 second keep alive
.server_keep_alive(30)

// Server recommends 2 minute keep alive
.server_keep_alive(120)
```

### User Properties
- **Type**: `HashMap<String, String>`
- **Default**: Empty
- **Use Case**: Application-specific metadata

```rust
.user_property("server_version".to_string(), "2.0.0".to_string())
.user_property("server_location".to_string(), "us-west-1".to_string())
.user_property("error_code".to_string(), "AUTH_001".to_string())
```

## MQTT 5.0 Return Codes

MQTT 5.0 introduces additional return codes beyond the basic MQTT 3.1.1 codes:

### Basic Return Codes (0-5)
- `Accepted` (0): Connection accepted
- `UnacceptableProtocolVersion` (1): Protocol version not supported
- `IdentifierRejected` (2): Client identifier rejected
- `ServerUnavailable` (3): Server unavailable
- `BadUsernameOrPassword` (4): Bad username or password
- `NotAuthorized` (5): Not authorized

### MQTT 5.0 Return Codes (128+)
- `MalformedPacket` (128): Malformed packet
- `ProtocolError` (130): Protocol error
- `ImplementationSpecificError` (131): Implementation specific error
- `UnsupportedProtocolVersion` (132): Unsupported protocol version
- `ClientIdentifierNotValid` (133): Client identifier not valid
- `BadUsernameOrPasswordV5` (134): Bad username or password (MQTT 5.0)
- `NotAuthorizedV5` (135): Not authorized (MQTT 5.0)
- `ServerUnavailableV5` (136): Server unavailable (MQTT 5.0)
- `ServerBusy` (137): Server busy
- `Banned` (138): Client banned
- `BadAuthenticationMethod` (140): Bad authentication method
- `TopicNameInvalid` (144): Topic name invalid
- `PacketTooLarge` (149): Packet too large
- `QuotaExceeded` (151): Quota exceeded
- `PayloadFormatInvalid` (153): Payload format invalid
- `RetainNotSupported` (154): Retain not supported
- `QoSNotSupported` (155): QoS not supported
- `UseAnotherServer` (156): Use another server
- `ServerMoved` (157): Server moved
- `ConnectionRateExceeded` (159): Connection rate exceeded

## Best Practices

1. **Set Reasonable Limits**: Set packet size and receive maximum based on your server capabilities
2. **Advertise Capabilities**: Use properties to clearly communicate what your server supports
3. **Error Reporting**: Provide meaningful reason strings and error codes for debugging
4. **User Properties**: Use user properties for application-specific metadata, not for core MQTT functionality
5. **Authentication**: Use authentication method and data for secure connections
6. **Server Redirection**: Use server reference for load balancing and failover scenarios

## Error Handling

Properties are validated when creating the connection acknowledgment. Invalid values will cause encoding/decoding failures:

```rust
// This will fail if the QoS value is invalid
.max_qos(3)  // Invalid QoS level

// This will fail if the packet size is too large
.max_packet_size(300_000_000)  // Exceeds MQTT 5.0 limit
```

## Protocol Version Considerations

- **MQTT 3.1.1 (v4)**: Properties are ignored, connection uses standard MQTT 3.1.1 behavior
- **MQTT 5.0 (v5)**: All properties are processed and sent to the client

```rust
// MQTT 3.1.1 - properties ignored
let connack = ConnAckPacket::new(ConnectReturnCode::Accepted)
    .properties(properties);  // This will be ignored

// MQTT 5.0 - properties processed
let connack = ConnAckPacket::new(ConnectReturnCode::Accepted)
    .properties(properties);  // This will be sent to client
```

## Testing

Run the ConnAck Properties example to see all features in action:

```bash
cargo run --example connack_properties
```

This will demonstrate:
- Basic ConnAck packet creation
- Properties configuration
- Error handling
- Server capabilities
- Authentication scenarios
- Server redirection

