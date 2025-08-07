use dumq_mqtt::server::{Server, ServerConfig, Authentication};
use dumq_mqtt::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("MQTT Server Example");
    println!("==================");

    // Create authentication (not used in this example)
    let _auth = Authentication::new()
        .add_user("user", "password")
        .add_user("admin", "admin123");

    // Create server configuration
    let config = ServerConfig::new("127.0.0.1:1883")
        .max_connections(100)
        .max_packet_size(1024 * 1024) // 1MB
        .protocol_version(4) // MQTT 3.1.1
        .allow_anonymous(true);

    // Create server
    let mut server = Server::new(config);

    println!("Starting MQTT server on 127.0.0.1:1883");
    println!("Authentication required:");
    println!("  - user/password");
    println!("  - admin/admin123");
    println!("Press Ctrl+C to stop the server");

    // Start the server
    server.start().await?;

    Ok(())
} 