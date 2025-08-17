//! # MQTT Client Module
//! 
//! This module provides a comprehensive MQTT client implementation for connecting to MQTT brokers,
//! publishing messages, subscribing to topics, and receiving messages. It supports both MQTT 3.1.1
//! and MQTT 5.0 protocols with full feature compatibility.

pub mod config;
pub mod connection;
pub mod state;
pub mod handler;

// Re-export main components for easy access
pub use config::ClientConfig;
pub use connection::ClientConnection;
pub use state::ConnectionState;
pub use handler::MessageHandler;

// Re-export types that are commonly used with the client
pub use crate::protocol::{ConnectOptions, QoS, PublishOptions};
pub use crate::types::Message;

use crate::error::{Error, Result};
use log::{info, debug, warn};
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// MQTT client
pub struct Client {
    config: ClientConfig,
    connection: Option<ClientConnection>,
    state: ConnectionState,
    packet_id_counter: u16,
    subscriptions: HashMap<String, QoS>,
    message_handler: Option<MessageHandler>,
}

impl Client {
    /// Create a new MQTT client
    pub fn new(config: ClientConfig) -> Self {
        Self {
            config,
            connection: None,
            state: ConnectionState::Disconnected,
            packet_id_counter: 1,
            subscriptions: HashMap::new(),
            message_handler: None,
        }
    }

    /// Set message handler
    pub fn set_message_handler<F>(mut self, handler: F) -> Self 
    where
        F: Fn(Message) + Send + Sync + 'static,
    {
        self.message_handler = Some(Box::new(handler));
        self
    }

    /// Connect to MQTT broker
    pub async fn connect(mut self, options: ConnectOptions) -> Result<Self> {
        if !self.state.is_disconnected() {
            return Err(Error::Client("Client is not in disconnected state".to_string()));
        }

        self.state = ConnectionState::Connecting;
        info!("Connecting to MQTT broker at {}", self.config.server_addr);

        // Establish TCP connection
        let stream = timeout(
            self.config.connect_timeout,
            TcpStream::connect(&self.config.server_addr)
        ).await
            .map_err(|_| Error::Connection("Connection timeout".to_string()))?
            .map_err(|e| Error::Connection(format!("Failed to connect: {}", e)))?;

        info!("TCP connection established");

        // Create connection handler
        let mut connection = ClientConnection::new(stream, self.config.clone());

        // Send CONNECT packet
        let connack = connection.connect(options).await?;
        
        if connack.return_code != crate::types::ConnectReturnCode::Accepted {
            return Err(Error::Connection(format!(
                "Connection rejected: {:?}",
                connack.return_code
            )));
        }

        info!("MQTT connection established successfully");
        self.connection = Some(connection);
        self.state = ConnectionState::Connected;

        Ok(self)
    }

    /// Disconnect from MQTT broker
    pub async fn disconnect(&mut self) -> Result<()> {
        if !self.state.is_connected() {
            return Ok(());
        }

        self.state = ConnectionState::Disconnecting;
        info!("Disconnecting from MQTT broker");

        if let Some(ref mut connection) = self.connection {
            connection.disconnect().await?;
        }

        self.connection = None;
        self.state = ConnectionState::Disconnected;
        info!("Disconnected from MQTT broker");

        Ok(())
    }

    /// Subscribe to a topic
    pub async fn subscribe(&mut self, topic: impl Into<String>, qos: QoS) -> Result<()> {
        if !self.state.is_connected() {
            return Err(Error::Client("Client is not connected".to_string()));
        }

        let topic = topic.into();
        let packet_id = self.next_packet_id();
        
        info!("Subscribing to topic '{}' with QoS {:?}", topic, qos);

        if let Some(ref mut connection) = self.connection {
            connection.subscribe(&topic, qos, packet_id).await?;
            self.subscriptions.insert(topic, qos);
        }

        Ok(())
    }

    /// Unsubscribe from a topic
    pub async fn unsubscribe(&mut self, topic: impl Into<String>) -> Result<()> {
        if !self.state.is_connected() {
            return Err(Error::Client("Client is not connected".to_string()));
        }

        let topic = topic.into();
        let packet_id = self.next_packet_id();
        
        info!("Unsubscribing from topic '{}'", topic);

        if let Some(ref mut connection) = self.connection {
            connection.unsubscribe(&topic, packet_id).await?;
            self.subscriptions.remove(&topic);
        }

        Ok(())
    }

    /// Publish a message
    pub async fn publish(&mut self, options: PublishOptions) -> Result<()> {
        if !self.state.is_connected() {
            return Err(Error::Client("Client is not connected".to_string()));
        }

        let packet_id = if options.qos != QoS::AtMostOnce {
            Some(self.next_packet_id())
        } else {
            None
        };

        let publish_options = PublishOptions {
            packet_id,
            ..options
        };

        info!("Publishing message to topic '{}'", publish_options.topic);

        if let Some(ref mut connection) = self.connection {
            connection.publish(publish_options).await?;
        }

        Ok(())
    }

    /// Receive a message
    pub async fn recv(&mut self) -> Result<Option<Message>> {
        if !self.state.is_connected() {
            return Err(Error::Client("Client is not connected".to_string()));
        }

        if let Some(ref mut connection) = self.connection {
            connection.recv().await
        } else {
            Ok(None)
        }
    }

    /// Start listening for messages
    pub async fn listen(&mut self) -> Result<()> {
        if !self.state.is_connected() {
            return Err(Error::Client("Client is not connected".to_string()));
        }

        info!("Starting message listener");
        
        loop {
            match self.recv().await {
                Ok(Some(message)) => {
                    debug!("Received message on topic: {}", message.topic);
                    
                    // Call message handler if set
                    if let Some(ref handler) = self.message_handler {
                        handler(message);
                    }
                }
                Ok(None) => {
                    // No message available, continue
                    continue;
                }
                Err(e) => {
                    warn!("Error receiving message: {}", e);
                    if let Error::Disconnected = e {
                        break;
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Get connection state
    pub fn state(&self) -> ConnectionState {
        self.state.clone()
    }

    /// Check if client is connected
    pub fn is_connected(&self) -> bool {
        self.state.is_connected()
    }

    /// Check if client is disconnected
    pub fn is_disconnected(&self) -> bool {
        self.state.is_disconnected()
    }

    /// Get next packet ID
    fn next_packet_id(&mut self) -> u16 {
        let id = self.packet_id_counter;
        self.packet_id_counter = self.packet_id_counter.wrapping_add(1);
        if self.packet_id_counter == 0 {
            self.packet_id_counter = 1;
        }
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let config = ClientConfig::new("localhost:1883");
        let client = Client::new(config);
        assert!(client.connection.is_none());
        assert_eq!(client.state, ConnectionState::Disconnected);
        assert_eq!(client.packet_id_counter, 1);
        assert!(client.subscriptions.is_empty());
        assert!(client.message_handler.is_none());
    }

    #[test]
    fn test_client_next_packet_id() {
        let config = ClientConfig::new("localhost:1883");
        let mut client = Client::new(config);
        
        // Test packet ID incrementing
        assert_eq!(client.next_packet_id(), 1);
        assert_eq!(client.next_packet_id(), 2);
        assert_eq!(client.next_packet_id(), 3);
        
        // Test wrapping around u16::MAX
        client.packet_id_counter = u16::MAX;
        assert_eq!(client.next_packet_id(), u16::MAX);
        assert_eq!(client.next_packet_id(), 1);
        assert_eq!(client.next_packet_id(), 2);
    }

    #[test]
    fn test_client_state() {
        let config = ClientConfig::new("localhost:1883");
        let client = Client::new(config);
        assert_eq!(client.state(), ConnectionState::Disconnected);
    }

    #[test]
    fn test_client_connection_state_methods() {
        let config = ClientConfig::new("localhost:1883");
        let client = Client::new(config);
        
        assert!(!client.is_connected());
        assert!(client.is_disconnected());
    }

    #[test]
    fn test_client_add_subscription() {
        let config = ClientConfig::new("localhost:1883");
        let mut client = Client::new(config);
        
        client.subscriptions.insert("home/temp".to_string(), QoS::AtLeastOnce);
        client.subscriptions.insert("home/humidity".to_string(), QoS::ExactlyOnce);
        
        assert_eq!(client.subscriptions.len(), 2);
        assert_eq!(client.subscriptions.get("home/temp"), Some(&QoS::AtLeastOnce));
        assert_eq!(client.subscriptions.get("home/humidity"), Some(&QoS::ExactlyOnce));
    }

    #[test]
    fn test_packet_id_counter_thread_safety() {
        let config = ClientConfig::new("localhost:1883");
        let mut client1 = Client::new(config.clone());
        let mut client2 = Client::new(config);
        
        // Each client should have its own packet ID counter
        let id1 = client1.next_packet_id();
        let id2 = client2.next_packet_id();
        
        assert_eq!(id1, 1);
        assert_eq!(id2, 1);
        
        // Incrementing should not affect other clients
        let id1_next = client1.next_packet_id();
        let id2_next = client2.next_packet_id();
        
        assert_eq!(id1_next, 2);
        assert_eq!(id2_next, 2);
    }
}
