use dumq_mqtt::types::{
    PublishPacket, PublishProperties, 
    PAYLOAD_FORMAT_INDICATOR_UTF8, PAYLOAD_FORMAT_INDICATOR_UNSPECIFIED
};
use bytes::Bytes;

fn main() {
    println!("MQTT v5 Publish Properties Example\n");

    // Example 1: Basic PublishProperties creation
    println!("=== Example 1: Basic PublishProperties ===");
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

    println!("Payload Format: {:?}", properties.payload_format_indicator_str());
    println!("Message Expiry: {} seconds", properties.message_expiry_interval.unwrap());
    println!("Topic Alias: {}", properties.topic_alias.unwrap());
    println!("Response Topic: {}", properties.response_topic.as_ref().unwrap());
    println!("Correlation Data: {:?}", properties.correlation_data.as_ref().unwrap());
    println!("Subscription ID: {}", properties.subscription_identifier.unwrap());
    println!("Content Type: {}", properties.content_type.as_ref().unwrap());
    println!("User Properties: {:?}", properties.user_properties);
    println!("Is UTF-8: {}", properties.is_utf8_payload());
    println!("Is Empty: {}\n", properties.is_empty());

    // Example 2: Creating PublishPacket with properties using builder pattern
    println!("=== Example 2: PublishPacket with Properties (Builder Pattern) ===");
    let payload = Bytes::from(r#"{"message": "Hello MQTT v5!", "timestamp": 1234567890}"#);
    let packet = PublishPacket::new("sensor/temperature".to_string(), payload.clone())
        .payload_format_indicator(PAYLOAD_FORMAT_INDICATOR_UTF8)
        .message_expiry_interval(7200) // 2 hours
        .topic_alias(789)
        .response_topic("sensor/response".to_string())
        .correlation_data(Bytes::from("sensor-request-456"))
        .subscription_identifier(101)
        .content_type("application/json".to_string())
        .user_property("sensor_id".to_string(), "temp_001".to_string())
        .user_property("location".to_string(), "room_101".to_string())
        .user_property("unit".to_string(), "celsius".to_string());

    println!("Topic: {}", packet.topic_name);
    println!("Payload: {:?}", packet.payload);
    println!("Has Properties: {}", packet.has_properties());
    println!("QoS: {}", packet.qos());

    if let Some(props) = &packet.properties {
        println!("Properties:");
        println!("  - Payload Format: {:?}", props.payload_format_indicator_str());
        println!("  - Message Expiry: {} seconds", props.message_expiry_interval.unwrap());
        println!("  - Topic Alias: {}", props.topic_alias.unwrap());
        println!("  - Response Topic: {}", props.response_topic.as_ref().unwrap());
        println!("  - Correlation Data: {:?}", props.correlation_data.as_ref().unwrap());
        println!("  - Subscription ID: {}", props.subscription_identifier.unwrap());
        println!("  - Content Type: {}", props.content_type.as_ref().unwrap());
        println!("  - User Properties: {:?}", props.user_properties);
    }
    println!();

    // Example 3: Different payload format indicators
    println!("=== Example 3: Payload Format Indicators ===");
    
    let utf8_props = PublishProperties::new()
        .payload_format_indicator(PAYLOAD_FORMAT_INDICATOR_UTF8);
    println!("UTF-8 Properties:");
    println!("  - Indicator: {:?}", utf8_props.payload_format_indicator_str());
    println!("  - Is UTF-8: {}", utf8_props.is_utf8_payload());
    println!("  - Is Unspecified: {}", utf8_props.is_unspecified_payload());

    let unspecified_props = PublishProperties::new()
        .payload_format_indicator(PAYLOAD_FORMAT_INDICATOR_UNSPECIFIED);
    println!("Unspecified Properties:");
    println!("  - Indicator: {:?}", unspecified_props.payload_format_indicator_str());
    println!("  - Is UTF-8: {}", unspecified_props.is_utf8_payload());
    println!("  - Is Unspecified: {}", unspecified_props.is_unspecified_payload());
    println!();

    // Example 4: Empty properties
    println!("=== Example 4: Empty Properties ===");
    let empty_props = PublishProperties::new();
    println!("Empty Properties:");
    println!("  - Is Empty: {}", empty_props.is_empty());
    println!("  - Payload Format: {:?}", empty_props.payload_format_indicator_str());
    println!("  - Is UTF-8: {}", empty_props.is_utf8_payload());
    println!("  - Is Unspecified: {}", empty_props.is_unspecified_payload());
    println!();

    // Example 5: PublishPacket with QoS and packet ID
    println!("=== Example 5: PublishPacket with QoS ===");
    let qos_packet = PublishPacket::with_qos("qos/topic".to_string(), payload, 1, 999);
    println!("QoS Packet:");
    println!("  - Topic: {}", qos_packet.topic_name);
    println!("  - Packet ID: {:?}", qos_packet.packet_id);
    println!("  - QoS: {}", qos_packet.qos());
    println!("  - Has Properties: {}", qos_packet.has_properties());

    // Example 6: Setting properties after creation
    println!("\n=== Example 6: Setting Properties After Creation ===");
    let mut dynamic_packet = PublishPacket::new("dynamic/topic".to_string(), Bytes::from("dynamic payload"));
    
    // Add properties one by one
    dynamic_packet = dynamic_packet
        .payload_format_indicator(PAYLOAD_FORMAT_INDICATOR_UTF8)
        .message_expiry_interval(1800); // 30 minutes
    
    println!("After adding basic properties:");
    println!("  - Has Properties: {}", dynamic_packet.has_properties());
    
    // Add more properties
    dynamic_packet = dynamic_packet
        .user_property("dynamic_key".to_string(), "dynamic_value".to_string())
        .content_type("text/plain".to_string());
    
    println!("After adding more properties:");
    println!("  - Has Properties: {}", dynamic_packet.has_properties());
    
    if let Some(props) = &dynamic_packet.properties {
        let mut count = 0;
        if props.payload_format_indicator.is_some() { count += 1; }
        if props.message_expiry_interval.is_some() { count += 1; }
        if props.topic_alias.is_some() { count += 1; }
        if props.response_topic.is_some() { count += 1; }
        if props.correlation_data.is_some() { count += 1; }
        if props.subscription_identifier.is_some() { count += 1; }
        if props.content_type.is_some() { count += 1; }
        count += props.user_properties.len();
        println!("  - Properties count: {}", count);
    }

    println!("\nMQTT v5 Publish Properties example completed successfully!");
}
