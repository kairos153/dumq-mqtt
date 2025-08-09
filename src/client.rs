use crate::codec::MqttCodec;
use crate::error::{Error, Result};
use crate::protocol::{ConnectOptions, QoS, PublishOptions};
use crate::types::*;
use bytes::{Bytes, BytesMut};
use log::{info};
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

/// MQTT client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub server_addr: String,
    pub connect_timeout: Duration,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub keep_alive_interval: Duration,
    pub max_packet_size: usize,
    pub protocol_version: u8,
}

impl ClientConfig {
    pub fn new(server_addr: impl Into<String>) -> Self {
        Self {
            server_addr: server_addr.into(),
            connect_timeout: Duration::from_secs(30),
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            keep_alive_interval: Duration::from_secs(60),
            max_packet_size: 1024 * 1024, // 1MB
            protocol_version: 4, // MQTT 3.1.1
        }
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    pub fn read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = timeout;
        self
    }

    pub fn write_timeout(mut self, timeout: Duration) -> Self {
        self.write_timeout = timeout;
        self
    }

    pub fn keep_alive_interval(mut self, interval: Duration) -> Self {
        self.keep_alive_interval = interval;
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
}

/// MQTT client connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
}

/// MQTT client
pub struct Client {
    config: ClientConfig,
    connection: Option<ClientConnection>,
    state: ConnectionState,
    packet_id_counter: u16,
    subscriptions: HashMap<String, QoS>,
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
        }
    }

    /// Connect to MQTT broker
    pub async fn connect(mut self, options: ConnectOptions) -> Result<Self> {
        if self.state != ConnectionState::Disconnected {
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
        let mut connection = ClientConnection::new(
            stream,
            self.config.clone(),
        );

        // Send CONNECT packet
        let connack = connection.connect(options).await?;
        
        if connack.return_code != ConnectReturnCode::Accepted {
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
        if self.state != ConnectionState::Connected {
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
        if self.state != ConnectionState::Connected {
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
        if self.state != ConnectionState::Connected {
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
        if self.state != ConnectionState::Connected {
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
        if self.state != ConnectionState::Connected {
            return Err(Error::Client("Client is not connected".to_string()));
        }

        if let Some(ref mut connection) = self.connection {
            connection.recv().await
        } else {
            Ok(None)
        }
    }

    /// Get connection state
    pub fn state(&self) -> ConnectionState {
        self.state.clone()
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

/// MQTT client connection handler
struct ClientConnection {
    stream: TcpStream,
    config: ClientConfig,
    codec: MqttCodec,
    read_buffer: BytesMut,
    _write_buffer: BytesMut,
    packet_id_counter: u16,
}

impl ClientConnection {
    fn new(stream: TcpStream, config: ClientConfig) -> Self {
        let codec = MqttCodec::new(config.protocol_version);
        Self {
            stream,
            config,
            codec,
            read_buffer: BytesMut::new(),
            _write_buffer: BytesMut::new(),
            packet_id_counter: 1,
        }
    }

    async fn connect(&mut self, options: ConnectOptions) -> Result<ConnAckPacket> {
        // Create CONNECT packet
        let connect = ConnectPacket {
            protocol_name: match self.config.protocol_version {
                3 => "MQIsdp".to_string(),
                4 | 5 => "MQTT".to_string(),
                _ => return Err(Error::UnsupportedVersion(self.config.protocol_version)),
            },
            protocol_version: self.config.protocol_version,
            clean_session: options.clean_session,
            will_flag: options.will_topic.is_some(),
            will_qos: options.will_qos as u8,
            will_retain: options.will_retain,
            password_flag: options.password.is_some(),
            username_flag: options.username.is_some(),
            keep_alive: self.config.keep_alive_interval.as_secs() as u16,
            client_id: options.client_id,
            will_topic: options.will_topic,
            will_message: options.will_message.map(Bytes::from),
            username: options.username,
            password: options.password,
            properties: None, // TODO: Support MQTT 5.0 properties
        };

        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::Connect,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0, // Will be calculated by codec
            },
            payload: PacketPayload::Connect(connect),
        };

        // Send CONNECT packet
        let data = self.codec.encode(&packet)?;
        self.write_all(&data).await?;

        // Receive CONNACK packet
        let connack_packet = self.read_packet().await?;
        match connack_packet.payload {
            PacketPayload::ConnAck(connack) => Ok(connack),
            _ => Err(Error::Protocol("Expected CONNACK packet".to_string())),
        }
    }

    async fn disconnect(&mut self) -> Result<()> {
        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::Disconnect,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::Disconnect(DisconnectPacket {
                reason_code: None,
                properties: None,
            }),
        };

        let data = self.codec.encode(&packet)?;
        self.write_all(&data).await?;
        Ok(())
    }

    async fn subscribe(&mut self, topic: &str, qos: QoS, packet_id: u16) -> Result<()> {
        let topic_filter = TopicFilter {
            topic: topic.to_string(),
            qos: qos as u8,
            no_local: false,
            retain_as_published: false,
            retain_handling: 0,
        };

        let subscribe = SubscribePacket {
            packet_id,
            topic_filters: vec![topic_filter],
            properties: None,
        };

        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::Subscribe,
                dup: false,
                qos: 1, // SUBSCRIBE must use QoS 1
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::Subscribe(subscribe),
        };

        let data = self.codec.encode(&packet)?;
        self.write_all(&data).await?;

        // Wait for SUBACK
        let suback_packet = self.read_packet().await?;
        match suback_packet.payload {
            PacketPayload::SubAck(_suback) => Ok(()),
            _ => Err(Error::Protocol("Expected SUBACK packet".to_string())),
        }
    }

    async fn unsubscribe(&mut self, topic: &str, packet_id: u16) -> Result<()> {
        let unsubscribe = UnsubscribePacket {
            packet_id,
            topic_filters: vec![topic.to_string()],
            properties: None,
        };

        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::Unsubscribe,
                dup: false,
                qos: 1, // UNSUBSCRIBE must use QoS 1
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::Unsubscribe(unsubscribe),
        };

        let data = self.codec.encode(&packet)?;
        self.write_all(&data).await?;

        // Wait for UNSUBACK
        let unsuback_packet = self.read_packet().await?;
        match unsuback_packet.payload {
            PacketPayload::UnsubAck(_unsuback) => Ok(()),
            _ => Err(Error::Protocol("Expected UNSUBACK packet".to_string())),
        }
    }

    async fn publish(&mut self, options: PublishOptions) -> Result<()> {
        let publish = PublishPacket {
            topic_name: options.topic,
            packet_id: options.packet_id,
            payload: Bytes::from(options.payload),
            properties: None,
        };

        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::Publish,
                dup: options.dup,
                qos: options.qos as u8,
                retain: options.retain,
                remaining_length: 0,
            },
            payload: PacketPayload::Publish(publish),
        };

        let data = self.codec.encode(&packet)?;
        self.write_all(&data).await?;

        // Handle QoS 1 and 2 acknowledgments
        if options.qos != QoS::AtMostOnce {
            let packet_id = options.packet_id.unwrap_or_else(|| self.next_packet_id());
            
            match options.qos {
                QoS::AtLeastOnce => {
                    // Wait for PUBACK
                    let ack_packet = self.read_packet().await?;
                    match ack_packet.payload {
                        PacketPayload::PubAck(puback) => {
                            if puback.packet_id != packet_id {
                                return Err(Error::Protocol("Mismatched PUBACK packet ID".to_string()));
                            }
                        }
                        _ => return Err(Error::Protocol("Expected PUBACK packet".to_string())),
                    }
                }
                QoS::ExactlyOnce => {
                    // Wait for PUBREC
                    let rec_packet = self.read_packet().await?;
                    match rec_packet.payload {
                        PacketPayload::PubRec(pubrec) => {
                            if pubrec.packet_id != packet_id {
                                return Err(Error::Protocol("Mismatched PUBREC packet ID".to_string()));
                            }
                            // Send PUBREL
                            self.send_pubrel(packet_id).await?;
                            
                            // Wait for PUBCOMP
                            let comp_packet = self.read_packet().await?;
                            match comp_packet.payload {
                                PacketPayload::PubComp(pubcomp) => {
                                    if pubcomp.packet_id != packet_id {
                                        return Err(Error::Protocol("Mismatched PUBCOMP packet ID".to_string()));
                                    }
                                }
                                _ => return Err(Error::Protocol("Expected PUBCOMP packet".to_string())),
                            }
                        }
                        _ => return Err(Error::Protocol("Expected PUBREC packet".to_string())),
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    async fn recv(&mut self) -> Result<Option<Message>> {
        let packet = self.read_packet().await?;
        
        match packet.payload {
            PacketPayload::Publish(publish) => {
                let message = Message {
                    topic: publish.topic_name,
                    payload: publish.payload,
                    qos: packet.header.qos,
                    retain: packet.header.retain,
                    dup: packet.header.dup,
                    packet_id: publish.packet_id,
                };

                // Send acknowledgment for QoS 1 and 2
                if packet.header.qos > 0 {
                    match packet.header.qos {
                        1 => {
                            // QoS 1: Send PUBACK
                            self.send_puback(publish.packet_id.unwrap()).await?;
                        }
                        2 => {
                            // QoS 2: Send PUBREC
                            self.send_pubrec(publish.packet_id.unwrap()).await?;
                        }
                        _ => {}
                    }
                }

                Ok(Some(message))
            }
            PacketPayload::PubRel(pubrel) => {
                // Handle PUBREL for QoS 2
                self.send_pubcomp(pubrel.packet_id).await?;
                Ok(None)
            }
            PacketPayload::PingReq => {
                // Send PINGRESP
                self.send_pingresp().await?;
                Ok(None)
            }
            _ => Ok(None),
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
            let n = timeout(self.config.read_timeout, self.stream.read(&mut buf)).await
                .map_err(|_| Error::Timeout)?
                .map_err(Error::Io)?;

            if n == 0 {
                return Err(Error::Disconnected);
            }

            self.read_buffer.extend_from_slice(&buf[..n]);
        }
    }

    async fn write_all(&mut self, data: &[u8]) -> Result<()> {
        timeout(self.config.write_timeout, self.stream.write_all(data)).await
            .map_err(|_| Error::Timeout)?
            .map_err(Error::Io)?;
        
        self.stream.flush().await.map_err(Error::Io)?;
        Ok(())
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

    async fn send_pubrel(&mut self, packet_id: u16) -> Result<()> {
        let pubrel = PubRelPacket {
            packet_id,
            reason_code: None,
            properties: None,
        };

        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::PubRel,
                dup: false,
                qos: 1, // PUBREL must use QoS 1
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::PubRel(pubrel),
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

    fn next_packet_id(&mut self) -> u16 {
        // Use instance counter for thread safety
        let id = self.packet_id_counter;
        self.packet_id_counter = self.packet_id_counter.wrapping_add(1);
        id
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::QoS;
    use std::time::Duration;

    #[test]
    fn test_client_config_new() {
        let config = ClientConfig::new("localhost:1883");
        assert_eq!(config.server_addr, "localhost:1883");
        assert_eq!(config.connect_timeout, Duration::from_secs(30));
        assert_eq!(config.read_timeout, Duration::from_secs(30));
        assert_eq!(config.write_timeout, Duration::from_secs(30));
        assert_eq!(config.keep_alive_interval, Duration::from_secs(60));
        assert_eq!(config.max_packet_size, 1024 * 1024);
        assert_eq!(config.protocol_version, 4);
    }

    #[test]
    fn test_client_config_builder_pattern() {
        let config = ClientConfig::new("mqtt.example.com:8883")
            .connect_timeout(Duration::from_secs(60))
            .read_timeout(Duration::from_secs(45))
            .write_timeout(Duration::from_secs(45))
            .keep_alive_interval(Duration::from_secs(120))
            .max_packet_size(2 * 1024 * 1024)
            .protocol_version(5);

        assert_eq!(config.connect_timeout, Duration::from_secs(60));
        assert_eq!(config.read_timeout, Duration::from_secs(45));
        assert_eq!(config.write_timeout, Duration::from_secs(45));
        assert_eq!(config.keep_alive_interval, Duration::from_secs(120));
        assert_eq!(config.max_packet_size, 2 * 1024 * 1024);
        assert_eq!(config.protocol_version, 5);
    }

    #[test]
    fn test_client_config_clone() {
        let config1 = ClientConfig::new("localhost:1883");
        let config2 = config1.clone();
        assert_eq!(config1.server_addr, config2.server_addr);
        assert_eq!(config1.connect_timeout, config2.connect_timeout);
        assert_eq!(config1.max_packet_size, config2.max_packet_size);
    }

    #[test]
    fn test_connection_state_values() {
        assert_eq!(ConnectionState::Disconnected as u8, 0);
        assert_eq!(ConnectionState::Connecting as u8, 1);
        assert_eq!(ConnectionState::Connected as u8, 2);
        assert_eq!(ConnectionState::Disconnecting as u8, 3);
    }

    #[test]
    fn test_connection_state_clone() {
        let state1 = ConnectionState::Connected;
        let state2 = state1.clone();
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_connection_state_partial_eq() {
        assert_ne!(ConnectionState::Disconnected, ConnectionState::Connected);
        assert_eq!(ConnectionState::Connecting, ConnectionState::Connecting);
        assert_ne!(ConnectionState::Connected, ConnectionState::Disconnecting);
    }

    #[test]
    fn test_client_new() {
        let config = ClientConfig::new("localhost:1883");
        let client = Client::new(config);
        assert!(client.connection.is_none());
        assert_eq!(client.state, ConnectionState::Disconnected);
        assert_eq!(client.packet_id_counter, 1);
        assert!(client.subscriptions.is_empty());
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
    fn test_client_creation() {
        let config = ClientConfig::new("localhost:1883");
        let client = Client::new(config);
        assert_eq!(client.state, ConnectionState::Disconnected);
        assert_eq!(client.packet_id_counter, 1);
        assert!(client.subscriptions.is_empty());
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