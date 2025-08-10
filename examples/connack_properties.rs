use dumq_mqtt::types::{ConnAckPacket, ConnAckProperties, ConnectReturnCode};

fn main() {
    println!("MQTT ConnAck Properties Example");
    println!("===============================");

    // Create a basic ConnAck packet
    let basic_connack = ConnAckPacket::new(ConnectReturnCode::Accepted)
        .session_present(true);

    println!("Basic ConnAck Packet:");
    println!("  Session Present: {}", basic_connack.session_present);
    println!("  Return Code: {:?}", basic_connack.return_code);
    println!("  Is Success: {}", basic_connack.is_success());
    println!("  Properties: {:?}", basic_connack.properties);
    println!();

    // Create ConnAck Properties with various settings
    let properties = ConnAckProperties::new()
        .session_expiry_interval(3600)        // 1 hour
        .receive_maximum(100)
        .max_qos(2)
        .retain_available(true)
        .max_packet_size(1_000_000)          // 1MB
        .assigned_client_identifier("server_assigned_id_123".to_string())
        .topic_alias_maximum(20)
        .reason_string("Connection successful".to_string())
        .user_property("server_version".to_string(), "2.0.0".to_string())
        .user_property("server_location".to_string(), "us-west-1".to_string())
        .wildcard_subscription_available(true)
        .subscription_identifiers_available(true)
        .shared_subscription_available(true)
        .server_keep_alive(60)
        .response_information("mqtt.example.com".to_string())
        .server_reference("backup.example.com".to_string())
        .authentication_method("JWT".to_string())
        .authentication_data(b"auth_token_here".to_vec().into());

    println!("ConnAck Properties:");
    println!("  Session Expiry Interval: {:?}", properties.session_expiry_interval);
    println!("  Receive Maximum: {:?}", properties.receive_maximum);
    println!("  Max QoS: {:?}", properties.max_qos);
    println!("  Retain Available: {:?}", properties.retain_available);
    println!("  Max Packet Size: {:?}", properties.max_packet_size);
    println!("  Assigned Client ID: {:?}", properties.assigned_client_identifier);
    println!("  Topic Alias Maximum: {:?}", properties.topic_alias_maximum);
    println!("  Reason String: {:?}", properties.reason_string);
    println!("  User Properties: {:?}", properties.user_properties);
    println!("  Wildcard Subscription Available: {:?}", properties.wildcard_subscription_available);
    println!("  Subscription Identifiers Available: {:?}", properties.subscription_identifiers_available);
    println!("  Shared Subscription Available: {:?}", properties.shared_subscription_available);
    println!("  Server Keep Alive: {:?}", properties.server_keep_alive);
    println!("  Response Information: {:?}", properties.response_information);
    println!("  Server Reference: {:?}", properties.server_reference);
    println!("  Authentication Method: {:?}", properties.authentication_method);
    println!("  Authentication Data: {:?}", properties.authentication_data);
    println!("  Is Empty: {}", properties.is_empty());
    println!();

    // Create a ConnAck packet with properties
    let connack_with_properties = ConnAckPacket::new(ConnectReturnCode::Accepted)
        .session_present(false)
        .properties(properties);

    println!("ConnAck Packet with Properties:");
    println!("  Session Present: {}", connack_with_properties.session_present);
    println!("  Return Code: {:?}", connack_with_properties.return_code);
    println!("  Is Success: {}", connack_with_properties.is_success());
    println!("  Has Properties: {}", connack_with_properties.properties.is_some());
    println!();

    // Example with error return codes
    let error_connack = ConnAckPacket::new(ConnectReturnCode::BadUsernameOrPassword)
        .session_present(false)
        .properties(
            ConnAckProperties::new()
                .reason_string("Invalid credentials provided".to_string())
                .user_property("error_code".to_string(), "AUTH_001".to_string())
        );

    println!("Error ConnAck Packet:");
    println!("  Session Present: {}", error_connack.session_present);
    println!("  Return Code: {:?}", error_connack.return_code);
    println!("  Is Success: {}", error_connack.is_success());
    println!("  Is Error: {}", error_connack.is_error());
    println!("  Error Message: {:?}", error_connack.error_message());
    println!();

    // Example with MQTT 5.0 specific return codes
    let mqtt5_error = ConnAckPacket::new(ConnectReturnCode::PacketTooLarge)
        .session_present(false)
        .properties(
            ConnAckProperties::new()
                .reason_string("Packet size exceeds server limit".to_string())
                .max_packet_size(512_000)  // 512KB limit
                .user_property("max_allowed".to_string(), "512KB".to_string())
        );

    println!("MQTT 5.0 Error ConnAck Packet:");
    println!("  Return Code: {:?}", mqtt5_error.return_code);
    println!("  Error Message: {:?}", mqtt5_error.error_message());
    if let Some(ref props) = mqtt5_error.properties {
        println!("  Max Packet Size: {:?}", props.max_packet_size);
    }
    println!();

    // Example with server capabilities
    let server_capabilities = ConnAckProperties::new()
        .session_expiry_interval(7200)        // 2 hours
        .receive_maximum(50)
        .max_qos(1)                           // Only QoS 0 and 1 supported
        .retain_available(false)              // Retain not supported
        .max_packet_size(256_000)            // 256KB limit
        .topic_alias_maximum(10)
        .wildcard_subscription_available(false)  // No wildcard support
        .shared_subscription_available(false)    // No shared subscription support
        .subscription_identifiers_available(false) // No subscription IDs
        .server_keep_alive(30);

    let capability_connack = ConnAckPacket::new(ConnectReturnCode::Accepted)
        .session_present(true)
        .properties(server_capabilities);

    println!("Server Capabilities ConnAck:");
    println!("  Session Present: {}", capability_connack.session_present);
    if let Some(ref props) = capability_connack.properties {
        println!("  Max QoS: {:?}", props.max_qos);
        println!("  Retain Available: {:?}", props.retain_available);
        println!("  Wildcard Subscription Available: {:?}", props.wildcard_subscription_available);
        println!("  Shared Subscription Available: {:?}", props.shared_subscription_available);
        println!("  Subscription Identifiers Available: {:?}", props.subscription_identifiers_available);
    }
    println!();

    // Example with authentication
    let auth_connack = ConnAckPacket::new(ConnectReturnCode::Accepted)
        .session_present(false)
        .properties(
            ConnAckProperties::new()
                .authentication_method("CHALLENGE".to_string())
                .authentication_data(b"challenge_token_123".to_vec().into())
                .reason_string("Authentication required".to_string())
        );

    println!("Authentication ConnAck:");
    println!("  Return Code: {:?}", auth_connack.return_code);
    if let Some(ref props) = auth_connack.properties {
        println!("  Authentication Method: {:?}", props.authentication_method);
        println!("  Authentication Data: {:?}", props.authentication_data);
        println!("  Reason String: {:?}", props.reason_string);
    }
    println!();

    // Example with server redirection
    let redirect_connack = ConnAckPacket::new(ConnectReturnCode::UseAnotherServer)
        .session_present(false)
        .properties(
            ConnAckProperties::new()
                .server_reference("loadbalancer.example.com:1883".to_string())
                .reason_string("Please connect to the recommended server".to_string())
                .user_property("redirect_reason".to_string(), "load_balancing".to_string())
        );

    println!("Server Redirection ConnAck:");
    println!("  Return Code: {:?}", redirect_connack.return_code);
    println!("  Error Message: {:?}", redirect_connack.error_message());
    if let Some(ref props) = redirect_connack.properties {
        println!("  Server Reference: {:?}", props.server_reference);
        println!("  Reason String: {:?}", props.reason_string);
    }
}
