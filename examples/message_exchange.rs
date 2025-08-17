use dumq_mqtt::{
    server::{Server, ServerConfig, Authentication},
    client::{Client, ClientConfig},
    protocol::{QoS, ConnectOptions, PublishOptions},
    types::Message,
};
use log::{info, error};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    info!("Starting MQTT message exchange example");

    // Create server configuration
    let server_config = ServerConfig::new("127.0.0.1:1883")
        .allow_anonymous(true)
        .max_connections(10);

    // Create authentication (optional)
    let mut auth = Authentication::new();
    auth = auth.add_user("user1".to_string(), "password1".to_string());
    let server_config = server_config.authentication(auth);

    // Start server
    let mut server = Server::new(server_config);
    
    // Spawn server in background
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.start().await {
            error!("Server error: {}", e);
        }
    });

    // Wait a bit for server to start
    sleep(Duration::from_millis(100)).await;

    // Create publisher client
    let publisher_config = ClientConfig::new("127.0.0.1:1883");
    let mut publisher = Client::new(publisher_config);

    // Connect publisher
    let connect_options = ConnectOptions::new("publisher_client")
        .username("user1")
        .password("password1")
        .clean_session(true);

    publisher = publisher.connect(connect_options).await?;
    info!("Publisher connected");

    // Create subscriber client
    let subscriber_config = ClientConfig::new("127.0.0.1:1883");
    let mut subscriber = Client::new(subscriber_config);

    // Connect subscriber
    let connect_options = ConnectOptions::new("subscriber_client")
        .username("user1")
        .password("password1")
        .clean_session(true);

    subscriber = subscriber.connect(connect_options).await?;
    info!("Subscriber connected");

    // Subscribe to topic
    subscriber.subscribe("test/topic", QoS::AtLeastOnce).await?;
    info!("Subscriber subscribed to test/topic");

    // Set message handler for subscriber
    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let message_count_clone = message_count.clone();
    
    subscriber = subscriber.set_message_handler(move |message: Message| {
        info!("Received message: topic={}, payload={:?}", message.topic, message.payload);
        message_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    });

    // Spawn subscriber listener in background
    let subscriber_handle = tokio::spawn(async move {
        if let Err(e) = subscriber.listen().await {
            error!("Subscriber error: {}", e);
        }
    });

    // Wait a bit for subscription to be processed
    sleep(Duration::from_millis(100)).await;

    // Publish messages
    for i in 1..=5 {
        let payload = format!("Hello MQTT World! Message {}", i);
        let publish_options = PublishOptions {
            topic: "test/topic".to_string(),
            payload: payload.into_bytes(),
            qos: QoS::AtLeastOnce,
            retain: false,
            dup: false,
            packet_id: None,
        };

        publisher.publish(publish_options).await?;
        info!("Published message {}", i);
        
        sleep(Duration::from_millis(100)).await;
    }

    // Wait for messages to be processed
    sleep(Duration::from_secs(2)).await;

    // Check message count
    let received_count = message_count.load(std::sync::atomic::Ordering::SeqCst);
    info!("Received {} messages", received_count);

    // Disconnect clients
    publisher.disconnect().await?;
    info!("Publisher disconnected");

    // Cancel server and subscriber
    server_handle.abort();
    subscriber_handle.abort();

    info!("Message exchange example completed");
    Ok(())
}
