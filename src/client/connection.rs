use crate::codec::MqttCodec;
use crate::error::{Error, Result};
use crate::protocol::{ConnectOptions, QoS, PublishOptions};
use crate::types::*;
use bytes::{Bytes, BytesMut};
use log::{info, debug, warn};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

/// MQTT client connection handler
pub struct ClientConnection {
    stream: TcpStream,
    config: crate::client::config::ClientConfig,
    codec: MqttCodec,
    read_buffer: BytesMut,
    _write_buffer: BytesMut,
    packet_id_counter: u16,
}

impl ClientConnection {
    /// Create a new client connection
    pub fn new(stream: TcpStream, config: crate::client::config::ClientConfig) -> Self {
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

    /// Establish MQTT connection
    pub async fn connect(&mut self, options: ConnectOptions) -> Result<ConnAckPacket> {
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

    /// Disconnect from MQTT broker
    pub async fn disconnect(&mut self) -> Result<()> {
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

    /// Subscribe to a topic
    pub async fn subscribe(&mut self, topic: &str, qos: QoS, packet_id: u16) -> Result<()> {
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

    /// Unsubscribe from a topic
    pub async fn unsubscribe(&mut self, topic: &str, packet_id: u16) -> Result<()> {
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

    /// Publish a message
    pub async fn publish(&mut self, options: PublishOptions) -> Result<()> {
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

    /// Receive a message
    pub async fn recv(&mut self) -> Result<Option<Message>> {
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

    /// Read a packet from the stream
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

    /// Write data to the stream
    async fn write_all(&mut self, data: &[u8]) -> Result<()> {
        timeout(self.config.write_timeout, self.stream.write_all(data)).await
            .map_err(|_| Error::Timeout)?
            .map_err(Error::Io)?;
        
        self.stream.flush().await.map_err(Error::Io)?;
        Ok(())
    }

    /// Send PUBACK packet
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

    /// Send PUBREL packet
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

    /// Send PUBREC packet
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

    /// Send PUBCOMP packet
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

    /// Send PINGRESP packet
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

    /// Get next packet ID
    fn next_packet_id(&mut self) -> u16 {
        // Use instance counter for thread safety
        let id = self.packet_id_counter;
        self.packet_id_counter = self.packet_id_counter.wrapping_add(1);
        id
    }
}
