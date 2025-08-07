use dumq_mqtt::client::{Client, ClientConfig};
use dumq_mqtt::protocol::{ConnectOptions, QoS, PublishOptions};
use dumq_mqtt::error::Result;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("MQTT Client Example");
    println!("==================");

    // Create client configuration
    let config = ClientConfig::new("localhost:1883")
        .connect_timeout(Duration::from_secs(10))
        .read_timeout(Duration::from_secs(30))
        .write_timeout(Duration::from_secs(30))
        .keep_alive_interval(Duration::from_secs(60))
        .protocol_version(4); // MQTT 3.1.1

    // Create client
    let client = Client::new(config);

    // Create connect options
    let options = ConnectOptions::new("example_client")
        .clean_session(true)
        .keep_alive(Duration::from_secs(60));

    // Connect to broker
    println!("Connecting to MQTT broker...");
    let mut client = client.connect(options).await?;
    println!("Connected successfully!");

    // Subscribe to a topic
    println!("Subscribing to 'test/topic'...");
    client.subscribe("test/topic", QoS::AtLeastOnce).await?;
    println!("Subscribed successfully!");

    // Publish a message
    println!("Publishing message to 'test/topic'...");
    let publish_options = PublishOptions::new("test/topic", "Hello, MQTT!")
        .qos(QoS::AtLeastOnce)
        .retain(false);
    
    client.publish(publish_options).await?;
    println!("Message published successfully!");

    // Receive messages for a while
    println!("Receiving messages for 10 seconds...");
    let start = std::time::Instant::now();
    
    while start.elapsed() < Duration::from_secs(10) {
        match client.recv().await {
            Ok(Some(message)) => {
                println!("Received message:");
                println!("  Topic: {}", message.topic);
                println!("  Payload: {}", String::from_utf8_lossy(&message.payload));
                println!("  QoS: {}", message.qos);
                println!("  Retain: {}", message.retain);
                println!("  Dup: {}", message.dup);
                if let Some(packet_id) = message.packet_id {
                    println!("  Packet ID: {}", packet_id);
                }
                println!();
            }
            Ok(None) => {
                // No message available, continue
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }

    // Unsubscribe
    println!("Unsubscribing from 'test/topic'...");
    client.unsubscribe("test/topic").await?;
    println!("Unsubscribed successfully!");

    // Disconnect
    println!("Disconnecting...");
    client.disconnect().await?;
    println!("Disconnected successfully!");

    println!("Example completed successfully!");
    Ok(())
} 