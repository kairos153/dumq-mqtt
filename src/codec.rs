use crate::error::{Error, Result};
use crate::types::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::collections::HashMap;

/// MQTT packet encoder/decoder
pub struct MqttCodec {
    protocol_version: u8,
}

impl MqttCodec {
    pub fn new(protocol_version: u8) -> Self {
        Self { protocol_version }
    }

    /// Encode a packet into bytes
    pub fn encode(&self, packet: &Packet) -> Result<Bytes> {
        let mut buf = BytesMut::new();
        self.encode_packet(packet, &mut buf)?;
        Ok(buf.freeze())
    }

    /// Decode bytes into a packet
    pub fn decode(&self, buf: &mut BytesMut) -> Result<Option<Packet>> {
        if buf.len() < 2 {
            return Ok(None);
        }

        log::debug!("Decoding packet, buffer size: {}", buf.len());

        // Read fixed header
        let first_byte = buf[0];
        let packet_type = PacketType::from_u8(first_byte >> 4)
            .ok_or_else(|| Error::InvalidPacket("Invalid packet type".to_string()))?;
        
        let dup = (first_byte & 0x08) != 0;
        let qos = (first_byte & 0x06) >> 1;
        let retain = (first_byte & 0x01) != 0;

        log::debug!("Packet type: {:?}, dup: {}, qos: {}, retain: {}", packet_type, dup, qos, retain);

        // Remove the first byte (packet type and flags)
        buf.advance(1);

        // Read remaining length
        let remaining_length = self.decode_remaining_length(buf)?;
        log::debug!("Remaining length: {}", remaining_length);
        
        if buf.len() < remaining_length {
            log::debug!("Insufficient data: need {}, have {}", remaining_length, buf.len());
            return Ok(None);
        }
        
        log::debug!("Buffer after remaining length: {:?}", buf);

        let header = PacketHeader {
            packet_type,
            dup,
            qos,
            retain,
            remaining_length,
        };

        // Read variable header and payload
        let mut payload_buf = buf.split_to(remaining_length);
        let payload = self.decode_payload(&header, &mut payload_buf)?;

        let packet = Packet { header, payload };
        Ok(Some(packet))
    }

    fn encode_packet(&self, packet: &Packet, buf: &mut BytesMut) -> Result<()> {
        // Encode fixed header
        let packet_type = packet.header.packet_type as u8;
        let mut first_byte = packet_type << 4;
        
        if packet.header.dup {
            first_byte |= 0x08;
        }
        first_byte |= (packet.header.qos & 0x03) << 1;
        if packet.header.retain {
            first_byte |= 0x01;
        }

        buf.put_u8(first_byte);

        // Calculate payload length first
        let mut payload_buf = BytesMut::new();
        self.encode_payload(&packet.payload, &mut payload_buf)?;
        let payload_len = payload_buf.len();
        
        // Encode remaining length
        self.encode_remaining_length(payload_len, buf)?;
        
        // Append payload
        buf.extend_from_slice(&payload_buf);

        Ok(())
    }

    fn encode_payload(&self, payload: &PacketPayload, buf: &mut BytesMut) -> Result<()> {
        match payload {
            PacketPayload::Connect(connect) => self.encode_connect(connect, buf),
            PacketPayload::ConnAck(connack) => self.encode_connack(connack, buf),
            PacketPayload::Publish(publish) => self.encode_publish(publish, buf),
            PacketPayload::PubAck(puback) => self.encode_puback(puback, buf),
            PacketPayload::PubRec(pubrec) => self.encode_pubrec(pubrec, buf),
            PacketPayload::PubRel(pubrel) => self.encode_pubrel(pubrel, buf),
            PacketPayload::PubComp(pubcomp) => self.encode_pubcomp(pubcomp, buf),
            PacketPayload::Subscribe(subscribe) => self.encode_subscribe(subscribe, buf),
            PacketPayload::SubAck(suback) => self.encode_suback(suback, buf),
            PacketPayload::Unsubscribe(unsubscribe) => self.encode_unsubscribe(unsubscribe, buf),
            PacketPayload::UnsubAck(unsuback) => self.encode_unsuback(unsuback, buf),
            PacketPayload::PingReq => Ok(()),
            PacketPayload::PingResp => Ok(()),
            PacketPayload::Disconnect(disconnect) => self.encode_disconnect(disconnect, buf),
            PacketPayload::Auth(auth) => self.encode_auth(auth, buf),
        }
    }

    fn decode_payload(&self, header: &PacketHeader, buf: &mut BytesMut) -> Result<PacketPayload> {
        match header.packet_type {
            PacketType::Connect => self.decode_connect(buf),
            PacketType::ConnAck => self.decode_connack(buf),
            PacketType::Publish => self.decode_publish(header, buf),
            PacketType::PubAck => self.decode_puback(buf),
            PacketType::PubRec => self.decode_pubrec(buf),
            PacketType::PubRel => self.decode_pubrel(buf),
            PacketType::PubComp => self.decode_pubcomp(buf),
            PacketType::Subscribe => self.decode_subscribe(buf),
            PacketType::SubAck => self.decode_suback(buf),
            PacketType::Unsubscribe => self.decode_unsubscribe(buf),
            PacketType::UnsubAck => self.decode_unsuback(buf),
            PacketType::PingReq => Ok(PacketPayload::PingReq),
            PacketType::PingResp => Ok(PacketPayload::PingResp),
            PacketType::Disconnect => self.decode_disconnect(buf),
            PacketType::Auth => self.decode_auth(buf),
        }
    }

    // Connect packet encoding/decoding
    fn encode_connect(&self, connect: &ConnectPacket, buf: &mut BytesMut) -> Result<()> {
        // Protocol name
        self.encode_string(&connect.protocol_name, buf)?;
        
        // Protocol version
        buf.put_u8(connect.protocol_version);
        
        // Connect flags
        let mut connect_flags = 0u8;
        if connect.clean_session {
            connect_flags |= 0x02;
        }
        if connect.will_flag {
            connect_flags |= 0x04;
            connect_flags |= (connect.will_qos & 0x03) << 3;
            if connect.will_retain {
                connect_flags |= 0x20;
            }
        }
        if connect.password_flag {
            connect_flags |= 0x40;
        }
        if connect.username_flag {
            connect_flags |= 0x80;
        }
        buf.put_u8(connect_flags);
        
        // Keep alive
        buf.put_u16(connect.keep_alive);
        
        // Client ID
        self.encode_string(&connect.client_id, buf)?;
        
        // Will topic and message
        if connect.will_flag {
            if let Some(ref will_topic) = connect.will_topic {
                self.encode_string(will_topic, buf)?;
            }
            if let Some(ref will_message) = connect.will_message {
                self.encode_bytes(will_message, buf)?;
            }
        }
        
        // Username and password
        if connect.username_flag {
            if let Some(ref username) = connect.username {
                self.encode_string(username, buf)?;
            }
        }
        if connect.password_flag {
            if let Some(ref password) = connect.password {
                self.encode_string(password, buf)?;
            }
        }
        
        // MQTT 5.0 properties
        if self.protocol_version == 5 {
            if let Some(ref properties) = connect.properties {
                self.encode_connect_properties(properties, buf)?;
            }
        }
        
        Ok(())
    }

    fn decode_connect(&self, buf: &mut BytesMut) -> Result<PacketPayload> {
        // Protocol name
        let protocol_name = self.decode_string(buf)?;
        
        // Protocol version
        let protocol_version = buf.get_u8();
        
        // Connect flags
        let connect_flags = buf.get_u8();
        let clean_session = (connect_flags & 0x02) != 0;
        let will_flag = (connect_flags & 0x04) != 0;
        let will_qos = (connect_flags & 0x18) >> 3;
        let will_retain = (connect_flags & 0x20) != 0;
        let password_flag = (connect_flags & 0x40) != 0;
        let username_flag = (connect_flags & 0x80) != 0;
        
        // Keep alive
        let keep_alive = buf.get_u16();
        
        // Client ID
        let client_id = self.decode_string(buf)?;
        
        // Will topic and message
        let will_topic = if will_flag {
            Some(self.decode_string(buf)?)
        } else {
            None
        };
        
        let will_message = if will_flag {
            Some(self.decode_bytes(buf)?)
        } else {
            None
        };
        
        // Username and password
        let username = if username_flag {
            Some(self.decode_string(buf)?)
        } else {
            None
        };
        
        let password = if password_flag {
            Some(self.decode_string(buf)?)
        } else {
            None
        };
        
        // MQTT 5.0 properties
        let properties = if self.protocol_version == 5 {
            Some(self.decode_connect_properties(buf)?)
        } else {
            None
        };
        
        let connect = ConnectPacket {
            protocol_name,
            protocol_version,
            clean_session,
            will_flag,
            will_qos,
            will_retain,
            password_flag,
            username_flag,
            keep_alive,
            client_id,
            will_topic,
            will_message,
            username,
            password,
            properties,
        };
        
        Ok(PacketPayload::Connect(connect))
    }

    // ConnAck packet encoding/decoding
    fn encode_connack(&self, connack: &ConnAckPacket, buf: &mut BytesMut) -> Result<()> {
        // Connect acknowledge flags
        let mut ack_flags = 0u8;
        if connack.session_present {
            ack_flags |= 0x01;
        }
        buf.put_u8(ack_flags);
        
        // Return code
        buf.put_u8(connack.return_code as u8);
        
        // MQTT 5.0 properties
        if self.protocol_version == 5 {
            if let Some(ref properties) = connack.properties {
                self.encode_connack_properties(properties, buf)?;
            }
        }
        
        Ok(())
    }

    fn decode_connack(&self, buf: &mut BytesMut) -> Result<PacketPayload> {
        // Connect acknowledge flags
        let ack_flags = buf.get_u8();
        let session_present = (ack_flags & 0x01) != 0;
        
        // Return code
        let return_code = ConnectReturnCode::from_u8(buf.get_u8())
            .ok_or_else(|| Error::InvalidPacket("Invalid connect return code".to_string()))?;
        
        // MQTT 5.0 properties
        let properties = if self.protocol_version == 5 {
            Some(self.decode_connack_properties(buf)?)
        } else {
            None
        };
        
        let connack = ConnAckPacket {
            session_present,
            return_code,
            properties,
        };
        
        Ok(PacketPayload::ConnAck(connack))
    }

    // Publish packet encoding/decoding
    fn encode_publish(&self, publish: &PublishPacket, buf: &mut BytesMut) -> Result<()> {
        // Topic name
        self.encode_string(&publish.topic_name, buf)?;
        
        // Packet ID (for QoS > 0)
        if publish.packet_id.is_some() {
            buf.put_u16(publish.packet_id.unwrap());
        }
        
        // MQTT 5.0 properties
        if self.protocol_version == 5 {
            if let Some(ref properties) = publish.properties {
                self.encode_publish_properties(properties, buf)?;
            }
        }
        
        // Payload
        buf.put_slice(&publish.payload);
        
        Ok(())
    }

    fn decode_publish(&self, header: &PacketHeader, buf: &mut BytesMut) -> Result<PacketPayload> {
        // Topic name
        let topic_name = self.decode_string(buf)?;
        
        // Packet ID (for QoS > 0)
        let packet_id = if header.qos > 0 {
            Some(buf.get_u16())
        } else {
            None
        };
        
        // MQTT 5.0 properties
        let properties = if self.protocol_version == 5 {
            Some(self.decode_publish_properties(buf)?)
        } else {
            None
        };
        
        // Payload (remaining bytes)
        let payload = if buf.is_empty() {
            Bytes::new()
        } else {
            buf.split_to(buf.len()).freeze()
        };
        
        let publish = PublishPacket {
            topic_name,
            packet_id,
            payload,
            properties,
        };
        
        Ok(PacketPayload::Publish(publish))
    }

    // Utility methods for encoding/decoding
    fn encode_string(&self, s: &str, buf: &mut BytesMut) -> Result<()> {
        let bytes = s.as_bytes();
        if bytes.len() > 65535 {
            return Err(Error::InvalidPacket("String too long".to_string()));
        }
        buf.put_u16(bytes.len() as u16);
        buf.put_slice(bytes);
        Ok(())
    }

    fn decode_string(&self, buf: &mut BytesMut) -> Result<String> {
        if buf.len() < 2 {
            return Err(Error::InvalidPacket("Insufficient data for string length".to_string()));
        }
        let len = buf.get_u16() as usize;
        log::debug!("Decoding string with length: {}", len);
        if buf.len() < len {
            return Err(Error::InvalidPacket(format!("Insufficient data for string: need {}, have {}", len, buf.len())));
        }
        let bytes = buf.split_to(len);
        String::from_utf8(bytes.to_vec())
            .map_err(|e| Error::InvalidPacket(format!("Invalid UTF-8: {}", e)))
    }

    fn encode_bytes(&self, data: &[u8], buf: &mut BytesMut) -> Result<()> {
        if data.len() > 65535 {
            return Err(Error::InvalidPacket("Data too long".to_string()));
        }
        buf.put_u16(data.len() as u16);
        buf.put_slice(data);
        Ok(())
    }

    fn decode_bytes(&self, buf: &mut BytesMut) -> Result<Bytes> {
        let len = buf.get_u16() as usize;
        if buf.len() < len {
            return Err(Error::InvalidPacket("Insufficient data for bytes".to_string()));
        }
        Ok(buf.split_to(len).freeze())
    }

    fn encode_remaining_length(&self, len: usize, buf: &mut BytesMut) -> Result<()> {
        if len > 268_435_455 {
            return Err(Error::InvalidPacket("Remaining length too large".to_string()));
        }
        
        let mut value = len;
        loop {
            let mut byte = (value % 128) as u8;
            value /= 128;
            if value > 0 {
                byte |= 0x80;
            }
            buf.put_u8(byte);
            if value == 0 {
                break;
            }
        }
        Ok(())
    }

    fn decode_remaining_length(&self, buf: &mut BytesMut) -> Result<usize> {
        let mut value = 0usize;
        let mut multiplier = 1usize;
        
        log::debug!("Decoding remaining length, buffer size: {}", buf.len());
        
        for i in 0..4 {
            if buf.len() < 1 {
                return Err(Error::InvalidPacket("Insufficient data for remaining length".to_string()));
            }
            
            let byte = buf[0];
            buf.advance(1);
            
            log::debug!("Remaining length byte {}: 0x{:02x}", i, byte);
            
            value += ((byte & 0x7F) as usize) * multiplier;
            multiplier *= 128;
            
            if (byte & 0x80) == 0 {
                break;
            }
        }
        
        log::debug!("Decoded remaining length: {}", value);
        Ok(value)
    }

    // Placeholder methods for MQTT 5.0 properties
    fn encode_connect_properties(&self, _properties: &ConnectProperties, _buf: &mut BytesMut) -> Result<()> {
        // TODO: Implement MQTT 5.0 connect properties encoding
        Ok(())
    }

    fn decode_connect_properties(&self, _buf: &mut BytesMut) -> Result<ConnectProperties> {
        // TODO: Implement MQTT 5.0 connect properties decoding
        Ok(ConnectProperties {
            session_expiry_interval: None,
            receive_maximum: None,
            max_packet_size: None,
            topic_alias_maximum: None,
            request_response_information: None,
            request_problem_information: None,
            user_properties: HashMap::new(),
            authentication_method: None,
            authentication_data: None,
        })
    }

    fn encode_connack_properties(&self, _properties: &ConnAckProperties, _buf: &mut BytesMut) -> Result<()> {
        // TODO: Implement MQTT 5.0 connack properties encoding
        Ok(())
    }

    fn decode_connack_properties(&self, _buf: &mut BytesMut) -> Result<ConnAckProperties> {
        // TODO: Implement MQTT 5.0 connack properties decoding
        Ok(ConnAckProperties {
            session_expiry_interval: None,
            receive_maximum: None,
            max_qos: None,
            retain_available: None,
            max_packet_size: None,
            assigned_client_identifier: None,
            topic_alias_maximum: None,
            reason_string: None,
            user_properties: HashMap::new(),
            wildcard_subscription_available: None,
            subscription_identifiers_available: None,
            shared_subscription_available: None,
            server_keep_alive: None,
            response_information: None,
            server_reference: None,
            authentication_method: None,
            authentication_data: None,
        })
    }

    fn encode_publish_properties(&self, _properties: &PublishProperties, _buf: &mut BytesMut) -> Result<()> {
        // TODO: Implement MQTT 5.0 publish properties encoding
        Ok(())
    }

    fn decode_publish_properties(&self, _buf: &mut BytesMut) -> Result<PublishProperties> {
        // TODO: Implement MQTT 5.0 publish properties decoding
        Ok(PublishProperties {
            payload_format_indicator: None,
            message_expiry_interval: None,
            topic_alias: None,
            response_topic: None,
            correlation_data: None,
            user_properties: HashMap::new(),
            subscription_identifier: None,
            content_type: None,
        })
    }

    // Placeholder methods for other packet types
    fn encode_puback(&self, puback: &PubAckPacket, buf: &mut BytesMut) -> Result<()> {
        // Packet ID
        buf.put_u16(puback.packet_id);
        Ok(())
    }

    fn decode_puback(&self, buf: &mut BytesMut) -> Result<PacketPayload> {
        // Packet ID
        let packet_id = buf.get_u16();
        Ok(PacketPayload::PubAck(PubAckPacket {
            packet_id,
            reason_code: Some(0), // Success
            properties: None,
        }))
    }

    fn encode_pubrec(&self, _pubrec: &PubRecPacket, _buf: &mut BytesMut) -> Result<()> {
        // TODO: Implement pubrec encoding
        Ok(())
    }

    fn decode_pubrec(&self, _buf: &mut BytesMut) -> Result<PacketPayload> {
        // TODO: Implement pubrec decoding
        Err(Error::Protocol("PubRec not implemented".to_string()))
    }

    fn encode_pubrel(&self, _pubrel: &PubRelPacket, _buf: &mut BytesMut) -> Result<()> {
        // TODO: Implement pubrel encoding
        Ok(())
    }

    fn decode_pubrel(&self, _buf: &mut BytesMut) -> Result<PacketPayload> {
        // TODO: Implement pubrel decoding
        Err(Error::Protocol("PubRel not implemented".to_string()))
    }

    fn encode_pubcomp(&self, _pubcomp: &PubCompPacket, _buf: &mut BytesMut) -> Result<()> {
        // TODO: Implement pubcomp encoding
        Ok(())
    }

    fn decode_pubcomp(&self, _buf: &mut BytesMut) -> Result<PacketPayload> {
        // TODO: Implement pubcomp decoding
        Err(Error::Protocol("PubComp not implemented".to_string()))
    }

    fn encode_subscribe(&self, subscribe: &SubscribePacket, buf: &mut BytesMut) -> Result<()> {
        // Packet ID
        buf.put_u16(subscribe.packet_id);
        
        // Topic filters
        for topic_filter in &subscribe.topic_filters {
            self.encode_string(&topic_filter.topic, buf)?;
            buf.put_u8(topic_filter.qos);
        }
        
        Ok(())
    }

    fn decode_subscribe(&self, buf: &mut BytesMut) -> Result<PacketPayload> {
        // Packet ID
        let packet_id = buf.get_u16();
        
        // Topic filters
        let mut topic_filters = Vec::new();
        while buf.has_remaining() {
            let topic = self.decode_string(buf)?;
            let qos = buf.get_u8();
            
            topic_filters.push(TopicFilter {
                topic,
                qos,
                no_local: false,
                retain_as_published: false,
                retain_handling: 0,
            });
        }
        
        Ok(PacketPayload::Subscribe(SubscribePacket {
            packet_id,
            topic_filters,
            properties: None,
        }))
    }

    fn encode_suback(&self, suback: &SubAckPacket, buf: &mut BytesMut) -> Result<()> {
        // Packet ID
        buf.put_u16(suback.packet_id);
        
        // Return codes
        for &return_code in &suback.return_codes {
            buf.put_u8(return_code);
        }
        
        Ok(())
    }

    fn decode_suback(&self, buf: &mut BytesMut) -> Result<PacketPayload> {
        // Packet ID
        let packet_id = buf.get_u16();
        
        // Return codes
        let mut return_codes = Vec::new();
        while buf.has_remaining() {
            return_codes.push(buf.get_u8());
        }
        
        Ok(PacketPayload::SubAck(SubAckPacket {
            packet_id,
            return_codes,
            properties: None,
        }))
    }

    fn encode_unsubscribe(&self, unsubscribe: &UnsubscribePacket, buf: &mut BytesMut) -> Result<()> {
        // Packet ID
        buf.put_u16(unsubscribe.packet_id);
        
        // Topic filters
        for topic_filter in &unsubscribe.topic_filters {
            self.encode_string(topic_filter, buf)?;
        }
        
        Ok(())
    }

    fn decode_unsubscribe(&self, buf: &mut BytesMut) -> Result<PacketPayload> {
        // Packet ID
        let packet_id = buf.get_u16();
        
        // Topic filters
        let mut topic_filters = Vec::new();
        while buf.has_remaining() {
            topic_filters.push(self.decode_string(buf)?);
        }
        
        Ok(PacketPayload::Unsubscribe(UnsubscribePacket {
            packet_id,
            topic_filters,
            properties: None,
        }))
    }

    fn encode_unsuback(&self, unsuback: &UnsubAckPacket, buf: &mut BytesMut) -> Result<()> {
        // Packet ID
        buf.put_u16(unsuback.packet_id);
        Ok(())
    }

    fn decode_unsuback(&self, buf: &mut BytesMut) -> Result<PacketPayload> {
        // Packet ID
        let packet_id = buf.get_u16();
        Ok(PacketPayload::UnsubAck(UnsubAckPacket {
            packet_id,
            reason_codes: Vec::new(), // Empty for MQTT 3.1.1
            properties: None,
        }))
    }

    fn encode_disconnect(&self, _disconnect: &DisconnectPacket, _buf: &mut BytesMut) -> Result<()> {
        // DISCONNECT packet has no payload in MQTT 3.1.1
        Ok(())
    }

    fn decode_disconnect(&self, _buf: &mut BytesMut) -> Result<PacketPayload> {
        // DISCONNECT packet has no payload in MQTT 3.1.1
        Ok(PacketPayload::Disconnect(DisconnectPacket {
            reason_code: None,
            properties: None,
        }))
    }

    fn encode_auth(&self, _auth: &AuthPacket, _buf: &mut BytesMut) -> Result<()> {
        // TODO: Implement auth encoding
        Ok(())
    }

    fn decode_auth(&self, _buf: &mut BytesMut) -> Result<PacketPayload> {
        // TODO: Implement auth decoding
        Err(Error::Protocol("Auth not implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_codec_creation() {
        let codec = MqttCodec::new(4);
        assert_eq!(codec.protocol_version, 4);
    }

    #[test]
    fn test_encode_decode_string() {
        let codec = MqttCodec::new(4);
        let mut buf = BytesMut::new();
        
        // Test encoding
        codec.encode_string("test", &mut buf).unwrap();
        assert_eq!(buf.len(), 6); // 2 bytes for length + 4 bytes for string
        
        // Test decoding
        let decoded = codec.decode_string(&mut buf).unwrap();
        assert_eq!(decoded, "test");
    }

    #[test]
    fn test_encode_decode_bytes() {
        let codec = MqttCodec::new(4);
        let mut buf = BytesMut::new();
        let data = b"test data";
        
        // Test encoding
        codec.encode_bytes(data, &mut buf).unwrap();
        assert_eq!(buf.len(), 11); // 2 bytes for length + 9 bytes for data
        
        // Test decoding
        let decoded = codec.decode_bytes(&mut buf).unwrap();
        assert_eq!(decoded, Bytes::from(data.as_ref()));
    }

    #[test]
    fn test_encode_decode_remaining_length() {
        let codec = MqttCodec::new(4);
        let mut buf = BytesMut::new();
        
        // Test small values
        codec.encode_remaining_length(0, &mut buf).unwrap();
        assert_eq!(buf.len(), 1);
        assert_eq!(codec.decode_remaining_length(&mut buf).unwrap(), 0);
        
        // Test medium values
        buf.clear();
        codec.encode_remaining_length(127, &mut buf).unwrap();
        assert_eq!(buf.len(), 1);
        assert_eq!(codec.decode_remaining_length(&mut buf).unwrap(), 127);
        
        // Test larger values
        buf.clear();
        codec.encode_remaining_length(128, &mut buf).unwrap();
        assert_eq!(buf.len(), 2);
        assert_eq!(codec.decode_remaining_length(&mut buf).unwrap(), 128);
        
        // Test maximum value
        buf.clear();
        codec.encode_remaining_length(268_435_455, &mut buf).unwrap();
        assert_eq!(buf.len(), 4);
        assert_eq!(codec.decode_remaining_length(&mut buf).unwrap(), 268_435_455);
    }

    #[test]
    fn test_encode_decode_connect_packet() {
        let codec = MqttCodec::new(4);
        
        let connect = ConnectPacket {
            protocol_name: "MQTT".to_string(),
            protocol_version: 4,
            clean_session: true,
            will_flag: false,
            will_qos: 0,
            will_retain: false,
            password_flag: false,
            username_flag: false,
            keep_alive: 60,
            client_id: "test_client".to_string(),
            will_topic: None,
            will_message: None,
            username: None,
            password: None,
            properties: None,
        };
        
        let packet = Packet {
            header: PacketHeader {
                packet_type: PacketType::Connect,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::Connect(connect),
        };
        
        // Test encoding
        let encoded = codec.encode(&packet).unwrap();
        assert!(!encoded.is_empty());
        
        println!("Encoded packet: {:?}", encoded);
        println!("Encoded packet length: {}", encoded.len());
        
        // Test decoding
        let mut buf = BytesMut::from(encoded.as_ref());
        let decoded = codec.decode(&mut buf).unwrap().unwrap();
        
        assert_eq!(decoded.header.packet_type, PacketType::Connect);
        assert_eq!(decoded.header.dup, false);
        assert_eq!(decoded.header.qos, 0);
        assert_eq!(decoded.header.retain, false);
        
        if let PacketPayload::Connect(decoded_connect) = decoded.payload {
            assert_eq!(decoded_connect.protocol_name, "MQTT");
            assert_eq!(decoded_connect.protocol_version, 4);
            assert_eq!(decoded_connect.clean_session, true);
            assert_eq!(decoded_connect.client_id, "test_client");
            assert_eq!(decoded_connect.keep_alive, 60);
        } else {
            panic!("Expected Connect payload");
        }
    }

    #[test]
    fn test_encode_decode_connack_packet() {
        let codec = MqttCodec::new(4);
        
        let connack = ConnAckPacket {
            session_present: false,
            return_code: ConnectReturnCode::Accepted,
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
        
        // Test encoding
        let encoded = codec.encode(&packet).unwrap();
        assert!(!encoded.is_empty());
        
        // Test decoding
        let mut buf = BytesMut::from(encoded.as_ref());
        let decoded = codec.decode(&mut buf).unwrap().unwrap();
        
        assert_eq!(decoded.header.packet_type, PacketType::ConnAck);
        
        if let PacketPayload::ConnAck(decoded_connack) = decoded.payload {
            assert_eq!(decoded_connack.session_present, false);
            assert_eq!(decoded_connack.return_code, ConnectReturnCode::Accepted);
        } else {
            panic!("Expected ConnAck payload");
        }
    }

    #[test]
    fn test_encode_decode_ping_packets() {
        let codec = MqttCodec::new(4);
        
        // Test PingReq
        let pingreq = Packet {
            header: PacketHeader {
                packet_type: PacketType::PingReq,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::PingReq,
        };
        
        let encoded = codec.encode(&pingreq).unwrap();
        let mut buf = BytesMut::from(encoded.as_ref());
        let decoded = codec.decode(&mut buf).unwrap().unwrap();
        
        assert_eq!(decoded.header.packet_type, PacketType::PingReq);
        assert!(matches!(decoded.payload, PacketPayload::PingReq));
        
        // Test PingResp
        let pingresp = Packet {
            header: PacketHeader {
                packet_type: PacketType::PingResp,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::PingResp,
        };
        
        let encoded = codec.encode(&pingresp).unwrap();
        let mut buf = BytesMut::from(encoded.as_ref());
        let decoded = codec.decode(&mut buf).unwrap().unwrap();
        
        assert_eq!(decoded.header.packet_type, PacketType::PingResp);
        assert!(matches!(decoded.payload, PacketPayload::PingResp));
    }

    #[test]
    fn test_invalid_remaining_length() {
        let codec = MqttCodec::new(4);
        let mut buf = BytesMut::new();
        
        // Test too large value
        let result = codec.encode_remaining_length(268_435_456, &mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_string_length() {
        let codec = MqttCodec::new(4);
        let mut buf = BytesMut::new();
        
        // Test string too long
        let long_string = "a".repeat(65536);
        let result = codec.encode_string(&long_string, &mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_incomplete_packet() {
        let codec = MqttCodec::new(4);
        let mut buf = BytesMut::new();
        
        // Add incomplete packet data
        buf.put_u8(0x10); // CONNECT packet type
        buf.put_u8(0x01); // Remaining length = 1, but no data
        
        let result = codec.decode(&mut buf);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // Should return None for incomplete packet
    }
} 