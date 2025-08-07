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
#[derive(Debug, Clone)]
pub struct Authentication {
    pub users: HashMap<String, String>, // username -> password
}

impl Authentication {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
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
            PacketPayload::Publish(publish) => self.handle_publish(publish).await,
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

    async fn handle_publish(&mut self, publish: PublishPacket) -> Result<()> {
        info!("Handling PUBLISH to topic: {}", publish.topic_name);

        // Create message
        let message = Message {
            topic: publish.topic_name.clone(),
            payload: publish.payload.clone(),
            qos: publish.packet_id.map(|_| 1).unwrap_or(0),
            retain: false, // TODO: Handle retain flag
            dup: false,
            packet_id: publish.packet_id,
        };

        // Handle retained message
        if false { // TODO: Check retain flag
            let mut retained = self.retained_messages.write().await;
            retained.insert(publish.topic_name.clone(), message.clone());
        }

        // Publish to subscribers
        self.publish_to_subscribers(&message).await?;

        // Send acknowledgment for QoS 1 and 2
        if let Some(packet_id) = publish.packet_id {
            self.send_puback(packet_id).await?;
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
        self.send_suback(subscribe.packet_id, return_codes).await
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
        // Simple wildcard matching for now
        // TODO: Implement proper MQTT topic matching
        filter == topic || filter.ends_with("/#") && topic.starts_with(&filter[..filter.len()-2])
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

    async fn read_packet(&mut self) -> Result<Packet> {
        loop {
            // Try to decode a packet from the buffer
            if let Some(packet) = self.codec.decode(&mut self.read_buffer)? {
                return Ok(packet);
            }

            // Read more data from the stream
            let mut buf = vec![0u8; 1024];
            let n = self.stream.read(&mut buf).await
                .map_err(|e| Error::Io(e))?;

            if n == 0 {
                return Err(Error::Disconnected);
            }

            self.read_buffer.extend_from_slice(&buf[..n]);
        }
    }

    async fn write_all(&mut self, data: &[u8]) -> Result<()> {
        self.stream.write_all(data).await.map_err(|e| Error::Io(e))?;
        self.stream.flush().await.map_err(|e| Error::Io(e))?;
        Ok(())
    }
} 