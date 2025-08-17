//! Client connection handling module

use crate::codec::MqttCodec;
use crate::error::{Error, Result};
use crate::protocol::QoS;
use crate::types::*;
use bytes::BytesMut;
use log::{debug, info, warn};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use super::config::ServerConfig;
use super::session::SessionManager;
use super::router::MessageRouter;

/// MQTT server connection handler
pub struct ServerConnection {
    stream: TcpStream,
    config: ServerConfig,
    codec: MqttCodec,
    read_buffer: BytesMut,
    _write_buffer: BytesMut,
    client_id: Option<String>,
    username: Option<String>,
    session_manager: Arc<SessionManager>,
    message_router: Arc<MessageRouter>,
}

impl ServerConnection {
    /// Handle a new client connection
    pub async fn handle_connection(
        stream: TcpStream,
        _addr: SocketAddr,
        config: ServerConfig,
        session_manager: Arc<SessionManager>,
        message_router: Arc<MessageRouter>,
    ) -> Result<()> {
        let mut connection = Self::new(
            stream,
            config,
            session_manager,
            message_router,
        );

        connection.handle().await
    }

    fn new(
        stream: TcpStream,
        config: ServerConfig,
        session_manager: Arc<SessionManager>,
        message_router: Arc<MessageRouter>,
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
            session_manager,
            message_router,
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
            let session = self.session_manager.get_session(&connect.client_id).await;
            session.is_some()
        };

        // Create or update session
        self.session_manager.create_session(
            connect.client_id.clone(),
            connect.username.clone(),
            connect.clean_session,
        ).await;

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
            if publish.payload.is_empty() {
                // Empty payload with retain flag means clear the retained message
                self.message_router.clear_retained_message(&publish.topic_name).await;
                info!("Cleared retained message for topic: {}", publish.topic_name);
            } else {
                // Store the retained message
                self.message_router.store_retained_message(publish.topic_name.clone(), message.clone()).await;
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
            let qos = QoS::from_u8(topic_filter.qos).unwrap_or(QoS::AtMostOnce);
            self.session_manager.add_subscription(
                self.client_id.clone().unwrap_or_default(),
                topic_filter.topic.clone(),
                qos,
            ).await;

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
            if let Some(client_id) = &self.client_id {
                self.session_manager.remove_subscription(client_id, topic_filter).await;
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
            let session = self.session_manager.get_session(client_id).await;
            if let Some(session) = session {
                if session.clean_session {
                    self.session_manager.remove_session(client_id).await;
                }
            }
        }
        Err(Error::Disconnected)
    }

    async fn publish_to_subscribers(&self, message: &Message) -> Result<()> {
        let subscriptions = self.session_manager.get_all_subscriptions().await;
        
        for (topic_filter, subs) in subscriptions.iter() {
            if MessageRouter::topic_matches(topic_filter, &message.topic) {
                for subscription in subs {
                    // TODO: Send message to subscriber
                    debug!("Would send message to client: {}", subscription.client_id);
                }
            }
        }

        Ok(())
    }

    /// Send retained messages for matching topic filters to the client
    async fn send_retained_messages(&mut self, topic_filters: &[TopicFilter]) -> Result<()> {
        // Collect topic filter strings
        let topic_filter_strings: Vec<String> = topic_filters
            .iter()
            .map(|tf| tf.topic.clone())
            .collect();
        
        // Get retained messages for matching topics
        let messages = self.message_router.get_retained_messages_for_filters(&topic_filter_strings).await;
        
        // Send each retained message
        for message in messages {
            info!("Sending retained message for topic '{}' to client '{}'", 
                  message.topic, self.client_id.as_ref().unwrap_or(&"unknown".to_string()));
            
            // Create a publish packet for the retained message
            let publish = PublishPacket {
                topic_name: message.topic,
                packet_id: if message.qos > 0 { Some(self.next_packet_id()) } else { None },
                payload: message.payload,
                properties: None, // No MQTT 5.0 properties for now
            };

            let packet = Packet {
                header: PacketHeader {
                    packet_type: PacketType::Publish,
                    dup: false,
                    qos: message.qos,
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

    // Response packet sending methods
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::QoS;
    use crate::server::{Session, Subscription};

    #[test]
    fn test_topic_matches() {
        // Exact match
        assert!(MessageRouter::topic_matches("home/temp", "home/temp"));
        
        // Single level wildcard
        assert!(MessageRouter::topic_matches("home/+/temp", "home/living/temp"));
        assert!(MessageRouter::topic_matches("home/+/temp", "home/bedroom/temp"));
        assert!(!MessageRouter::topic_matches("home/+/temp", "home/living/bedroom/temp"));
        
        // Multi-level wildcard
        assert!(MessageRouter::topic_matches("home/#", "home/living/temp"));
        assert!(MessageRouter::topic_matches("home/#", "home/bedroom/humidity"));
        assert!(MessageRouter::topic_matches("home/#", "home"));
        assert!(!MessageRouter::topic_matches("home/#", "office/temp"));
        
        // Edge cases
        assert!(MessageRouter::topic_matches("#", "any/topic"));
        assert!(MessageRouter::topic_matches("+", "single"));
        assert!(!MessageRouter::topic_matches("home/+/temp", "home/temp"));
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
}
