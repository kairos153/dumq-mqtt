use dumq_mqtt::{
    server::{Server, ServerConfig},
    client::{Client, ClientConfig},
    protocol::{QoS, ConnectOptions, PublishOptions},
    types::Message,
};
use log::{info, error};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    info!("Starting simple MQTT pub/sub example");

    // Create and start server
    let server_config = ServerConfig::new("127.0.0.1:1884")
        .allow_anonymous(true);
    
    let mut server = Server::new(server_config);
    
    // Spawn server in background
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.start().await {
            error!("Server error: {}", e);
        }
    });

    // Wait for server to start
    sleep(Duration::from_millis(100)).await;

    // Create subscriber client
    let subscriber_config = ClientConfig::new("127.0.0.1:1884");
    let mut subscriber = Client::new(subscriber_config);

    // Connect subscriber
    let connect_options = ConnectOptions::new("subscriber")
        .clean_session(true);

    subscriber = subscriber.connect(connect_options).await?;
    info!("Subscriber connected");

    // Subscribe to topic
    subscriber.subscribe("hello/world", QoS::AtLeastOnce).await?;
    info!("Subscribed to hello/world");

    // Set message handler
    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let message_count_clone = message_count.clone();
    
    subscriber = subscriber.set_message_handler(move |message: Message| {
        info!("Received: {}", String::from_utf8_lossy(&message.payload));
        message_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    });

    // Start listening in background
    let subscriber_handle = tokio::spawn(async move {
        if let Err(e) = subscriber.listen().await {
            error!("Subscriber error: {}", e);
        }
    });

    // Wait for subscription
    sleep(Duration::from_millis(100)).await;

    // Create publisher client
    let publisher_config = ClientConfig::new("127.0.0.1:1884");
    let mut publisher = Client::new(publisher_config);

    // Connect publisher
    let connect_options = ConnectOptions::new("publisher")
        .clean_session(true);

    publisher = publisher.connect(connect_options).await?;
    info!("Publisher connected");

    // Publish message
    let publish_options = PublishOptions {
        topic: "hello/world".to_string(),
        payload: b"Hello, MQTT World!".to_vec(),
        qos: QoS::AtLeastOnce,
        retain: false,
        dup: false,
        packet_id: None,
    };

    publisher.publish(publish_options).await?;
    info!("Message published");

    // Wait for message to be received
    sleep(Duration::from_secs(1)).await;

    // Check result
    let received_count = message_count.load(std::sync::atomic::Ordering::SeqCst);
    info!("Received {} messages", received_count);

    // Cleanup
    publisher.disconnect().await?;
    server_handle.abort();
    subscriber_handle.abort();

    info!("Example completed successfully");
    Ok(())
}



