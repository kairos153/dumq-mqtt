use dumq_mqtt::client::{Client, ClientConfig};
use dumq_mqtt::server::{Server, ServerConfig};
use dumq_mqtt::protocol::{ConnectOptions, QoS, PublishOptions};
use std::time::Duration;
use std::thread;
use std::time::Duration as StdDuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("Starting MQTT Retain Message Example");
    println!("=====================================");

    // Start server in a separate thread
    let _server_handle = thread::spawn(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let config = ServerConfig::new("127.0.0.1:1884")
                .protocol_version(4); // MQTT 3.1.1

            let mut server = Server::new(config);
            println!("Server starting on 127.0.0.1:1884...");
            
            if let Err(e) = server.start().await {
                eprintln!("Server error: {}", e);
            }
        });
    });

    // Wait a bit for server to start
    thread::sleep(StdDuration::from_millis(100));

    // Create client configuration
    let config = ClientConfig::new("127.0.0.1:1884")
        .protocol_version(4); // MQTT 3.1.1

    // Create and connect client 1 (publisher)
    let client1 = Client::new(config.clone());
    let options1 = ConnectOptions::new("retain_publisher")
        .clean_session(true);
    
    let mut client1 = client1.connect(options1).await?;
    println!("Client 1 (publisher) connected");

    // Create and connect client 2 (subscriber)
    let client2 = Client::new(config);
    let options2 = ConnectOptions::new("retain_subscriber")
        .clean_session(true);
    
    let mut client2 = client2.connect(options2).await?;
    println!("Client 2 (subscriber) connected");

    // Subscribe to topic before publishing
    println!("Client 2 subscribing to 'test/retain'...");
    client2.subscribe("test/retain", QoS::AtLeastOnce).await?;

    // Wait a bit for subscription to be processed
    thread::sleep(StdDuration::from_millis(100));

    // Publish a retained message
    println!("Client 1 publishing retained message to 'test/retain'...");
    let publish_options = PublishOptions::new("test/retain", "This is a retained message!")
        .qos(QoS::AtLeastOnce)
        .retain(true);
    
    client1.publish(publish_options).await?;
    println!("Retained message published");

    // Wait a bit for message to be processed
    thread::sleep(StdDuration::from_millis(100));

    // Disconnect client 1
    println!("Client 1 disconnecting...");
    drop(client1);

    // Wait a bit
    thread::sleep(StdDuration::from_millis(100));

    // Create and connect client 3 (new subscriber)
    let client3 = Client::new(ClientConfig::new("127.0.0.1:1884").protocol_version(4));
    let options3 = ConnectOptions::new("new_subscriber")
        .clean_session(true);
    
    let mut client3 = client3.connect(options3).await?;
    println!("Client 3 (new subscriber) connected");

    // Subscribe to the same topic - should receive retained message
    println!("Client 3 subscribing to 'test/retain'...");
    client3.subscribe("test/retain", QoS::AtLeastOnce).await?;

    // Wait for retained message
    println!("Waiting for retained message...");
    let mut received_count = 0;
    let start_time = std::time::Instant::now();
    
    while received_count < 1 && start_time.elapsed() < Duration::from_secs(5) {
        if let Ok(Some(message)) = client3.recv().await {
            println!("Client 3 received: {:?}", message);
            if message.retain {
                println!("✓ Received retained message: {}", String::from_utf8_lossy(&message.payload));
                received_count += 1;
            }
        }
        thread::sleep(StdDuration::from_millis(10));
    }

    if received_count > 0 {
        println!("✓ Retain message functionality working correctly!");
    } else {
        println!("✗ No retained message received");
    }

    // Test clearing retained message
    println!("\nTesting retained message clearing...");
    
    // Publish empty payload with retain flag to clear
    let clear_options = PublishOptions::new("test/retain", "")
        .qos(QoS::AtLeastOnce)
        .retain(true);
    
    client1 = Client::new(ClientConfig::new("127.0.0.1:1884").protocol_version(4))
        .connect(ConnectOptions::new("clear_publisher").clean_session(true)).await?;
    
    client1.publish(clear_options).await?;
    println!("Cleared retained message");

    // Wait a bit
    thread::sleep(StdDuration::from_millis(100));

    // Create client 4 to verify message is cleared
    let client4 = Client::new(ClientConfig::new("127.0.0.1:1884").protocol_version(4));
    let options4 = ConnectOptions::new("verify_clear")
        .clean_session(true);
    
    let mut client4 = client4.connect(options4).await?;
    println!("Client 4 (verify clear) connected");

    // Subscribe to topic
    client4.subscribe("test/retain", QoS::AtLeastOnce).await?;

    // Wait a bit - should not receive any message
    println!("Waiting to verify no retained message...");
    let mut received_after_clear = false;
    let start_time = std::time::Instant::now();
    
    while !received_after_clear && start_time.elapsed() < Duration::from_secs(3) {
        if let Ok(Some(message)) = client4.recv().await {
            println!("Client 4 received: {:?}", message);
            if message.retain {
                received_after_clear = true;
            }
        }
        thread::sleep(StdDuration::from_millis(10));
    }

    if !received_after_clear {
        println!("✓ Retained message successfully cleared!");
    } else {
        println!("✗ Retained message still present after clearing");
    }

    println!("\nRetain message example completed!");
    
    // Clean shutdown
    Ok(())
}
