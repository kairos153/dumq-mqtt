use std::time::Duration;
use tokio::time::sleep;
use dumq_mqtt::{
    client::{Client, ClientConfig},
    server::{Server, ServerConfig},
    protocol::{ConnectOptions, PublishOptions, QoS},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with debug level
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    
    println!("=== MQTT Message Transmission Test ===");
    
    // Start server
    let server_config = ServerConfig::new("127.0.0.1:1884")
        .max_connections(10)
        .allow_anonymous(true);
    
    let mut server = Server::new(server_config);
    
    // Start server in background
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.start().await {
            eprintln!("Server error: {}", e);
        }
    });
    
    // Wait for server to start
    sleep(Duration::from_millis(500)).await;
    
    // Create publisher client
    let publisher_config = ClientConfig::new("127.0.0.1:1884");
    let mut publisher = Client::new(publisher_config);
    
    // Connect publisher
    println!("Connecting publisher...");
    let connect_options = ConnectOptions::new("publisher")
        .clean_session(true);
    
    publisher = publisher.connect(connect_options).await?;
    println!("Publisher connected successfully");
    
    // Create subscriber client
    let subscriber_config = ClientConfig::new("127.0.0.1:1884");
    let mut subscriber = Client::new(subscriber_config);
    
    // Connect subscriber
    println!("Connecting subscriber...");
    let connect_options = ConnectOptions::new("subscriber")
        .clean_session(true);
    
    subscriber = subscriber.connect(connect_options).await?;
    println!("Subscriber connected successfully");
    
    // Subscribe to test topic
    println!("Subscribing to test topic...");
    subscriber.subscribe("test/message", QoS::AtLeastOnce).await?;
    println!("Subscribed to test/message");
    
    // Wait a bit for subscription to be processed
    sleep(Duration::from_millis(500)).await;
    
    // Publish test message
    println!("Publishing test message...");
    let publish_options = PublishOptions::new("test/message", "Hello, MQTT!")
        .qos(QoS::AtLeastOnce)
        .retain(false);
    
    publisher.publish(publish_options).await?;
    println!("Test message published");
    
    // Wait for message to be received and check
    println!("Waiting for message to be received...");
    let mut message_received = false;
    
    // Try to receive message for a few seconds
    for i in 0..20 {
        println!("Attempt {}: Checking for message...", i + 1);
        match subscriber.recv().await {
            Ok(Some(message)) => {
                println!("✅ Message received!");
                println!("  Topic: {}", message.topic);
                println!("  Payload: {:?}", String::from_utf8_lossy(&message.payload));
                println!("  QoS: {}", message.qos);
                println!("  Retain: {}", message.retain);
                message_received = true;
                break;
            }
            Ok(None) => {
                println!("No message available yet...");
            }
            Err(e) => {
                println!("Error receiving message: {}", e);
            }
        }
        sleep(Duration::from_millis(200)).await;
    }
    
    if !message_received {
        println!("❌ No message received within timeout period");
    }
    
    // Disconnect clients
    println!("Disconnecting clients...");
    publisher.disconnect().await?;
    subscriber.disconnect().await?;
    
    // Stop server
    server_handle.abort();
    
    println!("Test completed!");
    Ok(())
}
