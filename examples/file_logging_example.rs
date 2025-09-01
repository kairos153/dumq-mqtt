use dumq_mqtt::logging::init_simple_file_logging;
use dumq_mqtt::client::{Client, ClientConfig};
use dumq_mqtt::server::{Server, ServerConfig};
use dumq_mqtt::protocol::{ConnectOptions, QoS, PublishOptions};
use dumq_mqtt::error::Result;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    println!("MQTT File Logging Example");
    println!("=========================");
    println!();
    
    // Set up file logging for debugging
    println!("Setting up file logging for debugging...");
    let _handle = init_simple_file_logging(
        "debug.log",
        log::LevelFilter::Debug,
    ).unwrap();
    
    log::info!("File logging initialized for debugging");
    log::debug!("This debug message will be written to debug.log");
    log::info!("This info message will also be written to debug.log");
    log::warn!("This warning will be written to debug.log");
    log::error!("This error will be written to debug.log");
    
    // Now let's demonstrate MQTT operations with file logging
    println!();
    println!("Starting MQTT server and client with file logging...");
    
    // Start server in background
    let server_config = ServerConfig::new("127.0.0.1:1884")
        .max_connections(10)
        .allow_anonymous(true);
    
    let mut server = Server::new(server_config);
    
    // Spawn server in background
    let server_handle = tokio::spawn(async move {
        log::info!("Starting MQTT server on 127.0.0.1:1884");
        if let Err(e) = server.start().await {
            log::error!("Server error: {}", e);
        }
    });
    
    // Wait a bit for server to start
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Create and connect client
    let client_config = ClientConfig::new("localhost:1884")
        .connect_timeout(Duration::from_secs(5))
        .keep_alive_interval(Duration::from_secs(30));
    
    let client = Client::new(client_config);
    
    log::info!("Connecting to MQTT broker...");
    let connect_options = ConnectOptions::new("logging_test_client")
        .clean_session(true);
    
    let mut client = match client.connect(connect_options).await {
        Ok(client) => {
            log::info!("Successfully connected to MQTT broker");
            client
        }
        Err(e) => {
            log::error!("Failed to connect to MQTT broker: {}", e);
            return Err(e);
        }
    };
    
    // Subscribe to a topic
    log::info!("Subscribing to 'test/logging' topic...");
    if let Err(e) = client.subscribe("test/logging", QoS::AtLeastOnce).await {
        log::error!("Failed to subscribe: {}", e);
        return Err(e);
    }
    log::info!("Successfully subscribed to 'test/logging'");
    
    // Publish a message
    log::info!("Publishing test message...");
    let publish_options = PublishOptions::new("test/logging", "Hello from file logging example!")
        .qos(QoS::AtLeastOnce);
    
    if let Err(e) = client.publish(publish_options).await {
        log::error!("Failed to publish message: {}", e);
        return Err(e);
    }
    log::info!("Successfully published message");
    
    // Receive messages for a short time
    log::info!("Receiving messages for 3 seconds...");
    let start = std::time::Instant::now();
    
    while start.elapsed() < Duration::from_secs(3) {
        match client.recv().await {
            Ok(Some(message)) => {
                log::info!("Received message on topic '{}': {}", 
                    message.topic, 
                    String::from_utf8_lossy(&message.payload)
                );
            }
            Ok(None) => {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            Err(e) => {
                log::error!("Error receiving message: {}", e);
                break;
            }
        }
    }
    
    // Disconnect
    log::info!("Disconnecting from MQTT broker...");
    if let Err(e) = client.disconnect().await {
        log::error!("Error during disconnect: {}", e);
    } else {
        log::info!("Successfully disconnected from MQTT broker");
    }
    
    // Stop server
    server_handle.abort();
    
    println!();
    println!("File logging example completed!");
    println!("Check the following log files:");
    println!("  - debug.log (file logging for debugging)");
    println!();
    println!("You can also set the RUST_LOG environment variable to control log levels:");
    println!("  export RUST_LOG=debug");
    println!("  export RUST_LOG=dumq_mqtt=debug");
    println!("  export RUST_LOG=dumq_mqtt::server=debug,dumq_mqtt::client=info");
    
    Ok(())
}
