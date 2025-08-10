use crate::codec::MqttCodec;
use crate::error::{Error, Result};
use crate::protocol::QoS;
use crate::types::*;
use bytes::{BytesMut};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// MQTT server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub max_connections: usize,
    pub max_packet_size: usize,
    pub protocol_version: u8,
    pub allow_anonymous: bool,
    pub authentication: Option<Authentication>,
}

impl ServerConfig {
    pub fn new(bind_addr: impl Into<String>) -> Self {
        Self {
            bind_addr: bind_addr.into(),
            max_connections: 1000,
            max_packet_size: 1024 * 1024, // 1MB
            protocol_version: 4, // MQTT 3.1.1
            allow_anonymous: true,
            authentication: None,
        }
    }

    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    pub fn max_packet_size(mut self, size: usize) -> Self {
        self.max_packet_size = size;
        self
    }

    pub fn protocol_version(mut self, version: u8) -> Self {
        self.protocol_version = version;
        self
    }

    pub fn allow_anonymous(mut self, allow: bool) -> Self {
        self.allow_anonymous = allow;
        self
    }

    pub fn authentication(mut self, auth: Authentication) -> Self {
        self.authentication = Some(auth);
        self
    }
}

/// Authentication configuration
#[derive(Debug, Clone, Default)]
pub struct Authentication {
    pub users: HashMap<String, String>, // username -> password
}

impl Authentication {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_user(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.users.insert(username.into(), password.into());
        self
    }

    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        if let Some(expected_password) = self.users.get(username) {
            expected_password == password
        } else {
            false
        }
    }
}

/// MQTT server
pub struct Server {
    config: ServerConfig,
    listener: Option<TcpListener>,
    sessions: Arc<tokio::sync::RwLock<HashMap<String, Session>>>,
    subscriptions: Arc<tokio::sync::RwLock<HashMap<String, Vec<Subscription>>>>,
    retained_messages: Arc<tokio::sync::RwLock<HashMap<String, Message>>>,
}

impl Server {
    /// Create a new MQTT server
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            listener: None,
            sessions: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            subscriptions: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            retained_messages: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Start the server
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting MQTT server on {}", self.config.bind_addr);
        
        let listener = TcpListener::bind(&self.config.bind_addr).await
            .map_err(|e| Error::Server(format!("Failed to bind to {}: {}", self.config.bind_addr, e)))?;
        
        self.listener = Some(listener);
        info!("MQTT server started successfully");

        self.accept_connections().await
    }

    /// Accept incoming connections
    async fn accept_connections(&self) -> Result<()> {
        let listener = self.listener.as_ref()
            .ok_or_else(|| Error::Server("Server not started".to_string()))?;

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    info!("New connection from {}", addr);
                    
                    let config = self.config.clone();
                    let sessions = Arc::clone(&self.sessions);
                    let subscriptions = Arc::clone(&self.subscriptions);
                    let retained_messages = Arc::clone(&self.retained_messages);
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(
                            stream,
                            addr,
                            config,
                            sessions,
                            subscriptions,
                            retained_messages,
                        ).await {
                            error!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                }
            }
        }
    }

    /// Handle a client connection
    async fn handle_connection(
        stream: TcpStream,
        _addr: std::net::SocketAddr,
        config: ServerConfig,
        sessions: Arc<tokio::sync::RwLock<HashMap<String, Session>>>,
        subscriptions: Arc<tokio::sync::RwLock<HashMap<String, Vec<Subscription>>>>,
        retained_messages: Arc<tokio::sync::RwLock<HashMap<String, Message>>>,
    ) -> Result<()> {
        let mut connection = ServerConnection::new(
            stream,
            config,
            sessions,
            subscriptions,
            retained_messages,
        );

        connection.handle().await
    }
}

/// MQTT session
#[derive(Debug, Clone)]
pub struct Session {
    pub client_id: String,
    pub username: Option<String>,
    pub clean_session: bool,
    pub subscriptions: HashMap<String, QoS>,
    pub pending_messages: Vec<Message>,
}

impl Session {
    pub fn new(client_id: String, username: Option<String>, clean_session: bool) -> Self {
        Self {
            client_id,
            username,
            clean_session,
            subscriptions: HashMap::new(),
            pending_messages: Vec::new(),
        }
    }
}

/// MQTT subscription
#[derive(Debug, Clone)]
pub struct Subscription {
    pub client_id: String,
    pub topic_filter: String,
    pub qos: QoS,
}

impl Subscription {
    pub fn new(client_id: String, topic_filter: String, qos: QoS) -> Self {
        Self {
            client_id,
            topic_filter,
            qos,
        }
    }
}

/// MQTT server connection handler
struct ServerConnection {
    stream: TcpStream,
    config: ServerConfig,
    codec: MqttCodec,
    read_buffer: BytesMut,
    _write_buffer: BytesMut,
    client_id: Option<String>,
    username: Option<String>,
    sessions: Arc<tokio::sync::RwLock<HashMap<String, Session>>>,
    subscriptions: Arc<tokio::sync::RwLock<HashMap<String, Vec<Subscription>>>>,
    retained_messages: Arc<tokio::sync::RwLock<HashMap<String, Message>>>,
}

impl ServerConnection {
    fn new(
        stream: TcpStream,
        config: ServerConfig,
        sessions: Arc<tokio::sync::RwLock<HashMap<String, Session>>>,
        subscriptions: Arc<tokio::sync::RwLock<HashMap<String, Vec<Subscription>>>>,
        retained_messages: Arc<tokio::sync::RwLock<HashMap<String, Message>>>,
    ) -> Self {
        let codec = MqttCodec::new(config.protocol_version);
        Self {
            stream,
            config,
            codec,
            read_buffer: BytesMut::new(),
            _write_buffer: BytesMut::new(),
            client_id: None,
            username: None,
            sessions,
            subscriptions,
            retained_messages,
        }
    }

    async fn handle(&mut self) -> Result<()> {
        loop {
            let packet = self.read_packet().await?;
            self.handle_packet(packet).await?;
        }
    }

    async fn handle_packet(&mut self, packet: Packet) -> Result<()> {
        match packet.payload {
            PacketPayload::Connect(connect) => self.handle_connect(connect).await,
            PacketPayload::Publish(publish) => self.handle_publish(publish, &packet.header).await,
            PacketPayload::Subscribe(subscribe) => self.handle_subscribe(subscribe).await,
            PacketPayload::Unsubscribe(unsubscribe) => self.handle_unsubscribe(unsubscribe).await,
            PacketPayload::PingReq => self.handle_pingreq().await,
            PacketPayload::Disconnect(_) => self.handle_disconnect().await,
            _ => {
                warn!("Unhandled packet type: {:?}", packet.header.packet_type);
                Ok(())
            }
        }
    }

    async fn handle_connect(&mut self, connect: ConnectPacket) -> Result<()> {
        info!("Handling CONNECT from client: {}", connect.client_id);

        // Validate protocol version
        if connect.protocol_version != self.config.protocol_version {
            return self.send_connack(ConnectReturnCode::UnacceptableProtocolVersion, false).await;
        }

        // Validate client ID
        if connect.client_id.is_empty() && !connect.clean_session {
            return self.send_connack(ConnectReturnCode::IdentifierRejected, false).await;
        }

        // Handle authentication
        if let Some(ref auth) = self.config.authentication {
            if connect.username.is_none() && !self.config.allow_anonymous {
                return self.send_connack(ConnectReturnCode::NotAuthorized, false).await;
            }

            if let (Some(username), Some(password)) = (&connect.username, &connect.password) {
                if !auth.authenticate(username, password) {
                    return self.send_connack(ConnectReturnCode::BadUsernameOrPassword, false).await;
                }
            }
        }

        // Store client information
        self.client_id = Some(connect.client_id.clone());
        self.username = connect.username.clone();

        // Check for existing session
        let session_present = {
            let sessions = self.sessions.read().await;
            sessions.contains_key(&connect.client_id)
        };

        // Create or update session
        {
            let mut sessions = self.sessions.write().await;
            let session = Session::new(
                connect.client_id.clone(),
                connect.username.clone(),
                connect.clean_session,
            );
            sessions.insert(connect.client_id.clone(), session);
        }

        // Send CONNACK
        self.send_connack(ConnectReturnCode::Accepted, session_present).await
    }

    async fn handle_publish(&mut self, publish: PublishPacket, header: &PacketHeader) -> Result<()> {
        info!("Handling PUBLISH to topic: {}", publish.topic_name);

        // Get QoS level and retain flag from the packet header
        let qos_level = header.qos as u8;
        let retain_flag = header.retain;

        // Create message
        let message = Message {
            topic: publish.topic_name.clone(),
            payload: publish.payload.clone(),
            qos: qos_level,
            retain: retain_flag,
            dup: header.dup,
            packet_id: publish.packet_id,
        };

        // Handle retained message
        if retain_flag {
            let mut retained = self.retained_messages.write().await;
            if publish.payload.is_empty() {
                // Empty payload with retain flag means clear the retained message
                retained.remove(&publish.topic_name);
                info!("Cleared retained message for topic: {}", publish.topic_name);
            } else {
                // Store the retained message
                retained.insert(publish.topic_name.clone(), message.clone());
                info!("Stored retained message for topic: {}", publish.topic_name);
            }
        }

        // Publish to subscribers
        self.publish_to_subscribers(&message).await?;

        // Send acknowledgment for QoS 1 and 2
        if let Some(packet_id) = publish.packet_id {
            match qos_level {
                1 => {
                    // QoS 1: Send PUBACK
                    info!("Sending PUBACK for QoS 1 message with packet ID: {}", packet_id);
                    self.send_puback(packet_id).await?;
                }
                2 => {
                    // QoS 2: Send PUBREC first
                    info!("Sending PUBREC for QoS 2 message with packet ID: {}", packet_id);
                    self.send_pubrec(packet_id).await?;
                    
                    // Wait for PUBREL
                    info!("Waiting for PUBREL for packet ID: {}", packet_id);
                    let packet = self.read_packet().await?;
                    match packet.payload {
                        PacketPayload::PubRel(pubrel) => {
                            if pubrel.packet_id != packet_id {
                                return Err(Error::Protocol("Mismatched PUBREL packet ID".to_string()));
                            }
                            // Send PUBCOMP
                            info!("Sending PUBCOMP for packet ID: {}", packet_id);
                            self.send_pubcomp(packet_id).await?;
                        }
                        _ => return Err(Error::Protocol("Expected PUBREL packet".to_string())),
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    async fn handle_subscribe(&mut self, subscribe: SubscribePacket) -> Result<()> {
        info!("Handling SUBSCRIBE with packet ID: {}", subscribe.packet_id);

        let mut return_codes = Vec::new();

        for topic_filter in &subscribe.topic_filters {
            // Add subscription
            {
                let mut subscriptions = self.subscriptions.write().await;
                let subscription = Subscription::new(
                    self.client_id.clone().unwrap_or_default(),
                    topic_filter.topic.clone(),
                    QoS::from_u8(topic_filter.qos).unwrap_or(QoS::AtMostOnce),
                );
                
                subscriptions
                    .entry(topic_filter.topic.clone())
                    .or_insert_with(Vec::new)
                    .push(subscription);
            }

            // Update session
            if let Some(client_id) = &self.client_id {
                let mut sessions = self.sessions.write().await;
                if let Some(session) = sessions.get_mut(client_id) {
                    session.subscriptions.insert(
                        topic_filter.topic.clone(),
                        QoS::from_u8(topic_filter.qos).unwrap_or(QoS::AtMostOnce),
                    );
                }
            }

            return_codes.push(topic_filter.qos);
        }

        // Send SUBACK
        self.send_suback(subscribe.packet_id, return_codes).await?;

        // Send retained messages for matching topics
        self.send_retained_messages(&subscribe.topic_filters).await?;

        Ok(())
    }

    async fn handle_unsubscribe(&mut self, unsubscribe: UnsubscribePacket) -> Result<()> {
        info!("Handling UNSUBSCRIBE with packet ID: {}", unsubscribe.packet_id);

        for topic_filter in &unsubscribe.topic_filters {
            // Remove subscription
            {
                let mut subscriptions = self.subscriptions.write().await;
                if let Some(subs) = subscriptions.get_mut(topic_filter) {
                    subs.retain(|sub| sub.client_id != *self.client_id.as_ref().unwrap_or(&String::new()));
                    if subs.is_empty() {
                        subscriptions.remove(topic_filter);
                    }
                }
            }

            // Update session
            if let Some(client_id) = &self.client_id {
                let mut sessions = self.sessions.write().await;
                if let Some(session) = sessions.get_mut(client_id) {
                    session.subscriptions.remove(topic_filter);
                }
            }
        }

        // Send UNSUBACK
        self.send_unsuback(unsubscribe.packet_id).await
    }

    async fn handle_pingreq(&mut self) -> Result<()> {
        info!("Handling PINGREQ");
        self.send_pingresp().await
    }

    async fn handle_disconnect(&mut self) -> Result<()> {
        info!("Handling DISCONNECT");
        // Clean up session if clean_session is true
                    if let Some(client_id) = &self.client_id {
                let mut sessions = self.sessions.write().await;
                if let Some(session) = sessions.get(client_id) {
                    if session.clean_session {
                        sessions.remove(client_id);
                    }
                }
            }
        Err(Error::Disconnected)
    }

    async fn publish_to_subscribers(&self, message: &Message) -> Result<()> {
        let subscriptions = self.subscriptions.read().await;
        
        for (topic_filter, subs) in subscriptions.iter() {
            if Self::topic_matches(topic_filter, &message.topic) {
                for subscription in subs {
                    // TODO: Send message to subscriber
                    debug!("Would send message to client: {}", subscription.client_id);
                }
            }
        }

        Ok(())
    }

    fn topic_matches(filter: &str, topic: &str) -> bool {
        // MQTT topic matching with wildcards
        let filter_parts: Vec<&str> = filter.split('/').collect();
        let topic_parts: Vec<&str> = topic.split('/').collect();
        
        let mut filter_idx = 0;
        let mut topic_idx = 0;
        
        while filter_idx < filter_parts.len() && topic_idx < topic_parts.len() {
            let filter_part = filter_parts[filter_idx];
            let topic_part = topic_parts[topic_idx];
            
            if filter_part == "#" {
                // Multi-level wildcard - matches everything
                return true;
            } else if filter_part == "+" {
                // Single-level wildcard - matches any single level
                filter_idx += 1;
                topic_idx += 1;
            } else if filter_part == topic_part {
                // Exact match
                filter_idx += 1;
                topic_idx += 1;
            } else {
                // No match
                return false;
            }
        }
        
        // Check if we've processed all parts
        if filter_idx == filter_parts.len() && topic_idx == topic_parts.len() {
            return true;
        }
        
        // Handle trailing # wildcard
        if filter_idx == filter_parts.len() - 1 && filter_parts[filter_idx] == "#" {
            return true;
        }
        
        false
    }

    async fn send_connack(&mut self, return_code: ConnectReturnCode, session_present: bool) -> Result<()> {
        let connack = ConnAckPacket {
            session_present,
            return_code,
            properties: None,
        };

        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::ConnAck,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::ConnAck(connack),
        };

        let data = self.codec.encode(&packet)?;
        self.write_all(&data).await
    }

    async fn send_puback(&mut self, packet_id: u16) -> Result<()> {
        let puback = PubAckPacket {
            packet_id,
            reason_code: None,
            properties: None,
        };

        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::PubAck,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::PubAck(puback),
        };

        let data = self.codec.encode(&packet)?;
        self.write_all(&data).await
    }

    async fn send_suback(&mut self, packet_id: u16, return_codes: Vec<u8>) -> Result<()> {
        let suback = SubAckPacket {
            packet_id,
            return_codes,
            properties: None,
        };

        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::SubAck,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::SubAck(suback),
        };

        let data = self.codec.encode(&packet)?;
        self.write_all(&data).await
    }

    async fn send_unsuback(&mut self, packet_id: u16) -> Result<()> {
        let unsuback = UnsubAckPacket {
            packet_id,
            reason_codes: vec![0], // Success
            properties: None,
        };

        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::UnsubAck,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::UnsubAck(unsuback),
        };

        let data = self.codec.encode(&packet)?;
        self.write_all(&data).await
    }

    async fn send_pubrec(&mut self, packet_id: u16) -> Result<()> {
        let pubrec = PubRecPacket {
            packet_id,
            reason_code: None,
            properties: None,
        };

        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::PubRec,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::PubRec(pubrec),
        };

        let data = self.codec.encode(&packet)?;
        self.write_all(&data).await
    }

    async fn send_pubcomp(&mut self, packet_id: u16) -> Result<()> {
        let pubcomp = PubCompPacket {
            packet_id,
            reason_code: None,
            properties: None,
        };

        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::PubComp,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::PubComp(pubcomp),
        };

        let data = self.codec.encode(&packet)?;
        self.write_all(&data).await
    }

    async fn send_pingresp(&mut self) -> Result<()> {
        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::PingResp,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::PingResp,
        };

        let data = self.codec.encode(&packet)?;
        self.write_all(&data).await
    }

    /// Send retained messages for matching topic filters to the client
    async fn send_retained_messages(&mut self, topic_filters: &[TopicFilter]) -> Result<()> {
        // Collect retained messages to send to avoid borrowing issues
        let mut messages_to_send = Vec::new();
        
        {
            let retained = self.retained_messages.read().await;
            
            for topic_filter in topic_filters {
                for (topic, message) in retained.iter() {
                    if Self::topic_matches(&topic_filter.topic, topic) {
                        info!("Sending retained message for topic '{}' to client '{}'", 
                              topic, self.client_id.as_ref().unwrap_or(&"unknown".to_string()));
                        
                        messages_to_send.push((message.topic.clone(), message.payload.clone(), message.qos));
                    }
                }
            }
        }
        
        // Now send the messages without holding the read lock
        for (topic, payload, qos) in messages_to_send {
            // Create a publish packet for the retained message
            let publish = PublishPacket {
                topic_name: topic,
                packet_id: if qos > 0 { Some(self.next_packet_id()) } else { None },
                payload,
                properties: None, // No MQTT 5.0 properties for now
            };

            let packet = Packet {
                header: PacketHeader {
                    packet_type: PacketType::Publish,
                    dup: false,
                    qos,
                    retain: true, // Mark as retained
                    remaining_length: 0, // Will be calculated by encoder
                },
                payload: PacketPayload::Publish(publish),
            };

            let data = self.codec.encode(&packet)?;
            self.write_all(&data).await?;
        }

        Ok(())
    }

    /// Get next packet ID for QoS 1 and 2 messages
    fn next_packet_id(&mut self) -> u16 {
        static mut COUNTER: u16 = 0;
        unsafe {
            COUNTER = COUNTER.wrapping_add(1);
            if COUNTER == 0 {
                COUNTER = 1;
            }
            COUNTER
        }
    }

    async fn read_packet(&mut self) -> Result<Packet> {
        loop {
            // Try to decode a packet from the buffer
            if let Some(packet) = self.codec.decode(&mut self.read_buffer)? {
                return Ok(packet);
            }

            // Read more data from the stream
            let mut buf = vec![0u8; 1024];
            let n = self.stream.read(&mut buf).await
                .map_err(Error::Io)?;

            if n == 0 {
                return Err(Error::Disconnected);
            }

            self.read_buffer.extend_from_slice(&buf[..n]);
        }
    }

    async fn write_all(&mut self, data: &[u8]) -> Result<()> {
        self.stream.write_all(data).await.map_err(Error::Io)?;
        self.stream.flush().await.map_err(Error::Io)?;
        Ok(())
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::QoS;

    #[test]
    fn test_server_config_new() {
        let config = ServerConfig::new("127.0.0.1:1883");
        assert_eq!(config.bind_addr, "127.0.0.1:1883");
        assert_eq!(config.max_connections, 1000);
        assert_eq!(config.max_packet_size, 1024 * 1024);
        assert_eq!(config.protocol_version, 4);
        assert_eq!(config.allow_anonymous, true);
        assert!(config.authentication.is_none());
    }

    #[test]
    fn test_server_config_builder_pattern() {
        let config = ServerConfig::new("localhost:1883")
            .max_connections(500)
            .max_packet_size(512 * 1024)
            .protocol_version(5)
            .allow_anonymous(false);

        assert_eq!(config.max_connections, 500);
        assert_eq!(config.max_packet_size, 512 * 1024);
        assert_eq!(config.protocol_version, 5);
        assert_eq!(config.allow_anonymous, false);
    }

    #[test]
    fn test_server_config_clone() {
        let config1 = ServerConfig::new("127.0.0.1:1883");
        let config2 = config1.clone();
        assert_eq!(config1.bind_addr, config2.bind_addr);
        assert_eq!(config1.max_connections, config2.max_connections);
    }

    #[test]
    fn test_authentication_new() {
        let auth = Authentication::new();
        assert!(auth.users.is_empty());
    }

    #[test]
    fn test_authentication_add_user() {
        let auth = Authentication::new()
            .add_user("user1", "pass1")
            .add_user("user2", "pass2");

        assert_eq!(auth.users.len(), 2);
        assert_eq!(auth.users.get("user1"), Some(&"pass1".to_string()));
        assert_eq!(auth.users.get("user2"), Some(&"pass2".to_string()));
    }

    #[test]
    fn test_authentication_authenticate() {
        let auth = Authentication::new()
            .add_user("user1", "pass1");

        assert!(auth.authenticate("user1", "pass1"));
        assert!(!auth.authenticate("user1", "wrong_pass"));
        assert!(!auth.authenticate("unknown_user", "pass1"));
    }

    #[test]
    fn test_authentication_clone() {
        let auth1 = Authentication::new().add_user("user1", "pass1");
        let auth2 = auth1.clone();
        assert_eq!(auth1.users.len(), auth2.users.len());
        assert_eq!(auth1.authenticate("user1", "pass1"), auth2.authenticate("user1", "pass1"));
    }

    #[test]
    fn test_session_new() {
        let session = Session::new("client1".to_string(), Some("user1".to_string()), true);
        assert_eq!(session.client_id, "client1");
        assert_eq!(session.username, Some("user1".to_string()));
        assert_eq!(session.clean_session, true);
        assert!(session.subscriptions.is_empty());
        assert!(session.pending_messages.is_empty());
    }

    #[test]
    fn test_session_add_subscription() {
        let mut session = Session::new("client1".to_string(), None, false);
        session.subscriptions.insert("topic1".to_string(), QoS::AtLeastOnce);
        session.subscriptions.insert("topic2".to_string(), QoS::ExactlyOnce);

        assert_eq!(session.subscriptions.len(), 2);
        assert_eq!(session.subscriptions.get("topic1"), Some(&QoS::AtLeastOnce));
        assert_eq!(session.subscriptions.get("topic2"), Some(&QoS::ExactlyOnce));
    }

    #[test]
    fn test_session_clone() {
        let session1 = Session::new("client1".to_string(), Some("user1".to_string()), true);
        let session2 = session1.clone();
        assert_eq!(session1.client_id, session2.client_id);
        assert_eq!(session1.username, session2.username);
        assert_eq!(session1.clean_session, session2.clean_session);
    }

    #[test]
    fn test_subscription_new() {
        let subscription = Subscription::new("client1".to_string(), "topic1".to_string(), QoS::AtMostOnce);
        assert_eq!(subscription.client_id, "client1");
        assert_eq!(subscription.topic_filter, "topic1");
        assert_eq!(subscription.qos, QoS::AtMostOnce);
    }

    #[test]
    fn test_subscription_clone() {
        let sub1 = Subscription::new("client1".to_string(), "topic1".to_string(), QoS::AtLeastOnce);
        let sub2 = sub1.clone();
        assert_eq!(sub1.client_id, sub2.client_id);
        assert_eq!(sub1.topic_filter, sub2.topic_filter);
        assert_eq!(sub1.qos, sub2.qos);
    }

    #[test]
    fn test_server_new() {
        let config = ServerConfig::new("127.0.0.1:1883");
        let server = Server::new(config);
        assert!(server.listener.is_none());
        // Note: We can't test async operations in sync tests
        // The server is created successfully
    }

    #[test]
    fn test_topic_matches() {
        // Exact match
        assert!(ServerConnection::topic_matches("home/temp", "home/temp"));
        
        // Single level wildcard
        assert!(ServerConnection::topic_matches("home/+/temp", "home/living/temp"));
        assert!(ServerConnection::topic_matches("home/+/temp", "home/bedroom/temp"));
        assert!(!ServerConnection::topic_matches("home/+/temp", "home/living/bedroom/temp"));
        
        // Multi-level wildcard
        assert!(ServerConnection::topic_matches("home/#", "home/living/temp"));
        assert!(ServerConnection::topic_matches("home/#", "home/bedroom/humidity"));
        assert!(ServerConnection::topic_matches("home/#", "home"));
        assert!(!ServerConnection::topic_matches("home/#", "office/temp"));
        
        // Edge cases
        assert!(ServerConnection::topic_matches("#", "any/topic"));
        assert!(ServerConnection::topic_matches("+", "single"));
        assert!(!ServerConnection::topic_matches("home/+/temp", "home/temp"));
    }

    #[test]
    fn test_qos_handling_in_subscription() {
        let subscription1 = Subscription::new("client1".to_string(), "topic1".to_string(), QoS::AtMostOnce);
        let subscription2 = Subscription::new("client2".to_string(), "topic2".to_string(), QoS::AtLeastOnce);
        let subscription3 = Subscription::new("client3".to_string(), "topic3".to_string(), QoS::ExactlyOnce);

        assert_eq!(subscription1.qos, QoS::AtMostOnce);
        assert_eq!(subscription2.qos, QoS::AtLeastOnce);
        assert_eq!(subscription3.qos, QoS::ExactlyOnce);
    }

    #[test]
    fn test_qos_handling_in_session() {
        let mut session = Session::new("client1".to_string(), None, false);
        
        // Add subscriptions with different QoS levels
        session.subscriptions.insert("topic1".to_string(), QoS::AtMostOnce);
        session.subscriptions.insert("topic2".to_string(), QoS::AtLeastOnce);
        session.subscriptions.insert("topic3".to_string(), QoS::ExactlyOnce);

        assert_eq!(session.subscriptions.len(), 3);
        assert_eq!(session.subscriptions.get("topic1"), Some(&QoS::AtMostOnce));
        assert_eq!(session.subscriptions.get("topic2"), Some(&QoS::AtLeastOnce));
        assert_eq!(session.subscriptions.get("topic3"), Some(&QoS::ExactlyOnce));
    }

    #[test]
    fn test_qos_enum_values() {
        assert_eq!(QoS::AtMostOnce as u8, 0);
        assert_eq!(QoS::AtLeastOnce as u8, 1);
        assert_eq!(QoS::ExactlyOnce as u8, 2);
    }

    #[test]
    fn test_qos_from_u8() {
        assert_eq!(QoS::from_u8(0), Some(QoS::AtMostOnce));
        assert_eq!(QoS::from_u8(1), Some(QoS::AtLeastOnce));
        assert_eq!(QoS::from_u8(2), Some(QoS::ExactlyOnce));
        assert_eq!(QoS::from_u8(3), None);
    }

    #[test]
    fn test_retained_message_storage() {
        use tokio::sync::RwLock;
        use std::collections::HashMap;
        use std::sync::Arc;
        
        let retained_messages = Arc::new(RwLock::new(HashMap::new()));
        
        // Test that retained messages can be stored and retrieved
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let mut retained = retained_messages.write().await;
            let message = Message {
                topic: "test/topic".to_string(),
                payload: bytes::Bytes::from("test message"),
                qos: 0,
                retain: true,
                dup: false,
                packet_id: None,
            };
            
            retained.insert("test/topic".to_string(), message);
            assert!(retained.contains_key("test/topic"));
            
            let stored_message = retained.get("test/topic").unwrap();
            assert_eq!(stored_message.topic, "test/topic");
            assert_eq!(stored_message.payload, bytes::Bytes::from("test message"));
            assert!(stored_message.retain);
        });
    }

    #[test]
    fn test_topic_matching_for_retained_messages() {
        // Test that topic matching works correctly for retained messages
        assert!(ServerConnection::topic_matches("test/+", "test/hello"));
        assert!(ServerConnection::topic_matches("test/#", "test/hello/world"));
        assert!(ServerConnection::topic_matches("test/topic", "test/topic"));
        assert!(!ServerConnection::topic_matches("test/topic", "other/topic"));
    }
} 