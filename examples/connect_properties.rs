use dumq_mqtt::protocol::{ConnectOptions, QoS};
use std::time::Duration;

fn main() {
    println!("MQTT Connect Properties Example");
    println!("===============================");

    // Create a basic connect options
    let basic_options = ConnectOptions::new("basic_client")
        .username("user")
        .password("pass")
        .keep_alive(Duration::from_secs(30));

    println!("Basic Connect Options:");
    println!("  Client ID: {}", basic_options.client_id);
    println!("  Username: {:?}", basic_options.username);
    println!("  Keep Alive: {:?}", basic_options.keep_alive);
    println!("  Protocol Version: {}", basic_options.protocol_version);
    println!("  Properties: {:?}", basic_options.properties);
    println!();

    // Create MQTT 5.0 connect options with properties
    let mqtt5_options = ConnectOptions::new("mqtt5_client")
        .protocol_version(5)
        .username("mqtt5_user")
        .password("mqtt5_pass")
        .keep_alive(Duration::from_secs(60))
        .session_expiry_interval(3600) // 1 hour
        .receive_maximum(100)
        .max_packet_size(1_000_000) // 1MB
        .topic_alias_maximum(20)
        .request_response_information(true)
        .request_problem_information(false)
        .user_property("client_type", "example")
        .user_property("version", "1.0.0")
        .authentication_method("PLAIN")
        .authentication_data(b"auth_data");

    println!("MQTT 5.0 Connect Options with Properties:");
    println!("  Client ID: {}", mqtt5_options.client_id);
    println!("  Username: {:?}", mqtt5_options.username);
    println!("  Protocol Version: {}", mqtt5_options.protocol_version);
    
    if let Some(props) = &mqtt5_options.properties {
        println!("  Properties:");
        println!("    Session Expiry Interval: {:?}", props.session_expiry_interval);
        println!("    Receive Maximum: {:?}", props.receive_maximum);
        println!("    Max Packet Size: {:?}", props.max_packet_size);
        println!("    Topic Alias Maximum: {:?}", props.topic_alias_maximum);
        println!("    Request Response Information: {:?}", props.request_response_information);
        println!("    Request Problem Information: {:?}", props.request_problem_information);
        println!("    Authentication Method: {:?}", props.authentication_method);
        println!("    Authentication Data: {:?}", props.authentication_data);
        println!("    User Properties:");
        for (key, value) in &props.user_properties {
            println!("      {}: {}", key, value);
        }
    }

    // Example with will message and properties
    let will_options = ConnectOptions::new("will_client")
        .protocol_version(5)
        .will("test/will", b"Goodbye!", QoS::AtLeastOnce, true)
        .session_expiry_interval(7200) // 2 hours
        .receive_maximum(50)
        .user_property("will_enabled", "true");

    println!();
    println!("Connect Options with Will Message and Properties:");
    println!("  Client ID: {}", will_options.client_id);
    println!("  Will Topic: {:?}", will_options.will_topic);
    println!("  Will Message: {:?}", will_options.will_message);
    println!("  Will QoS: {:?}", will_options.will_qos);
    println!("  Will Retain: {}", will_options.will_retain);
    
    if let Some(props) = &will_options.properties {
        println!("  Properties:");
        println!("    Session Expiry Interval: {:?}", props.session_expiry_interval);
        println!("    Receive Maximum: {:?}", props.receive_maximum);
        println!("    User Properties:");
        for (key, value) in &props.user_properties {
            println!("      {}: {}", key, value);
        }
    }

    println!();
    println!("Example completed successfully!");
}
