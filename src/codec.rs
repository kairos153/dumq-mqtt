use crate::error::{Error, Result};
use crate::types::{Packet, PacketType, PacketHeader, PacketPayload, ConnectPacket, ConnAckPacket, PublishPacket, PubAckPacket, PubRecPacket, PubRelPacket, PubCompPacket, SubscribePacket, SubAckPacket, UnsubscribePacket, UnsubAckPacket, DisconnectPacket, AuthPacket, ConnectProperties, ConnAckProperties, PublishProperties, TopicFilter, ConnectReturnCode};

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
            if buf.is_empty() {
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
    fn encode_connect_properties(&self, properties: &ConnectProperties, buf: &mut BytesMut) -> Result<()> {
        // Properties length (will be calculated)
        let properties_start = buf.len();
        buf.put_u8(0); // Placeholder for properties length
        
        // Session Expiry Interval (0x11)
        if let Some(session_expiry) = properties.session_expiry_interval {
            buf.put_u8(0x11);
            buf.put_u32(session_expiry);
        }
        
        // Receive Maximum (0x21)
        if let Some(receive_max) = properties.receive_maximum {
            buf.put_u8(0x21);
            buf.put_u16(receive_max);
        }
        
        // Maximum Packet Size (0x27)
        if let Some(max_packet_size) = properties.max_packet_size {
            buf.put_u8(0x27);
            buf.put_u32(max_packet_size);
        }
        
        // Topic Alias Maximum (0x22)
        if let Some(topic_alias_max) = properties.topic_alias_maximum {
            buf.put_u8(0x22);
            buf.put_u16(topic_alias_max);
        }
        
        // Request Response Information (0x19)
        if let Some(request_response_info) = properties.request_response_information {
            buf.put_u8(0x19);
            buf.put_u8(if request_response_info { 1 } else { 0 });
        }
        
        // Request Problem Information (0x17)
        if let Some(request_problem_info) = properties.request_problem_information {
            buf.put_u8(0x17);
            buf.put_u8(if request_problem_info { 1 } else { 0 });
        }
        
        // User Properties (0x26)
        for (key, value) in &properties.user_properties {
            buf.put_u8(0x26);
            self.encode_string(key, buf)?;
            self.encode_string(value, buf)?;
        }
        
        // Authentication Method (0x15)
        if let Some(ref auth_method) = properties.authentication_method {
            buf.put_u8(0x15);
            self.encode_string(auth_method, buf)?;
        }
        
        // Authentication Data (0x16)
        if let Some(ref auth_data) = properties.authentication_data {
            buf.put_u8(0x16);
            self.encode_bytes(auth_data, buf)?;
        }
        
        // Update properties length
        let properties_len = buf.len() - properties_start - 1;
        if properties_len > 0 {
            buf[properties_start] = properties_len as u8;
        }
        
        Ok(())
    }

    fn decode_connect_properties(&self, buf: &mut BytesMut) -> Result<ConnectProperties> {
        let mut properties = ConnectProperties {
            session_expiry_interval: None,
            receive_maximum: None,
            max_packet_size: None,
            topic_alias_maximum: None,
            request_response_information: None,
            request_problem_information: None,
            user_properties: HashMap::new(),
            authentication_method: None,
            authentication_data: None,
        };
        
        // Properties length
        let properties_length = buf.get_u8() as usize;
        if properties_length == 0 {
            return Ok(properties);
        }
        
        let _properties_end = buf.len() - properties_length;
        let mut properties_buf = buf.split_to(properties_length);
        
        while properties_buf.has_remaining() {
            let property_id = properties_buf.get_u8();
            
            match property_id {
                0x11 => { // Session Expiry Interval
                    properties.session_expiry_interval = Some(properties_buf.get_u32());
                }
                0x21 => { // Receive Maximum
                    properties.receive_maximum = Some(properties_buf.get_u16());
                }
                0x27 => { // Maximum Packet Size
                    properties.max_packet_size = Some(properties_buf.get_u32());
                }
                0x22 => { // Topic Alias Maximum
                    properties.topic_alias_maximum = Some(properties_buf.get_u16());
                }
                0x19 => { // Request Response Information
                    properties.request_response_information = Some(properties_buf.get_u8() != 0);
                }
                0x17 => { // Request Problem Information
                    properties.request_problem_information = Some(properties_buf.get_u8() != 0);
                }
                0x26 => { // User Properties
                    let key = self.decode_string(&mut properties_buf)?;
                    let value = self.decode_string(&mut properties_buf)?;
                    properties.user_properties.insert(key, value);
                }
                0x15 => { // Authentication Method
                    properties.authentication_method = Some(self.decode_string(&mut properties_buf)?);
                }
                0x16 => { // Authentication Data
                    properties.authentication_data = Some(self.decode_bytes(&mut properties_buf)?);
                }
                _ => {
                    // Unknown property, skip it
                    log::warn!("Unknown connect property ID: 0x{:02x}", property_id);
                    // Try to skip the property value (this is a simplified approach)
                    if properties_buf.has_remaining() {
                        properties_buf.advance(1);
                    }
                }
            }
        }
        
        Ok(properties)
    }

    fn encode_connack_properties(&self, properties: &ConnAckProperties, buf: &mut BytesMut) -> Result<()> {
        // Properties length (will be calculated)
        let properties_start = buf.len();
        buf.put_u8(0); // Placeholder for properties length
        
        // Session Expiry Interval (0x11)
        if let Some(session_expiry) = properties.session_expiry_interval {
            buf.put_u8(0x11);
            buf.put_u32(session_expiry);
        }
        
        // Receive Maximum (0x21)
        if let Some(receive_max) = properties.receive_maximum {
            buf.put_u8(0x21);
            buf.put_u16(receive_max);
        }
        
        // Maximum QoS (0x24)
        if let Some(max_qos) = properties.max_qos {
            buf.put_u8(0x24);
            buf.put_u8(max_qos);
        }
        
        // Retain Available (0x25)
        if let Some(retain_available) = properties.retain_available {
            buf.put_u8(0x25);
            buf.put_u8(if retain_available { 1 } else { 0 });
        }
        
        // Maximum Packet Size (0x27)
        if let Some(max_packet_size) = properties.max_packet_size {
            buf.put_u8(0x27);
            buf.put_u32(max_packet_size);
        }
        
        // Assigned Client Identifier (0x12)
        if let Some(ref assigned_client_id) = properties.assigned_client_identifier {
            buf.put_u8(0x12);
            self.encode_string(assigned_client_id, buf)?;
        }
        
        // Topic Alias Maximum (0x22)
        if let Some(topic_alias_max) = properties.topic_alias_maximum {
            buf.put_u8(0x22);
            buf.put_u16(topic_alias_max);
        }
        
        // Reason String (0x1F)
        if let Some(ref reason_string) = properties.reason_string {
            buf.put_u8(0x1F);
            self.encode_string(reason_string, buf)?;
        }
        
        // User Properties (0x26)
        for (key, value) in &properties.user_properties {
            buf.put_u8(0x26);
            self.encode_string(key, buf)?;
            self.encode_string(value, buf)?;
        }
        
        // Wildcard Subscription Available (0x28)
        if let Some(wildcard_sub_available) = properties.wildcard_subscription_available {
            buf.put_u8(0x28);
            buf.put_u8(if wildcard_sub_available { 1 } else { 0 });
        }
        
        // Subscription Identifiers Available (0x29)
        if let Some(sub_id_available) = properties.subscription_identifiers_available {
            buf.put_u8(0x29);
            buf.put_u8(if sub_id_available { 1 } else { 0 });
        }
        
        // Shared Subscription Available (0x2A)
        if let Some(shared_sub_available) = properties.shared_subscription_available {
            buf.put_u8(0x2A);
            buf.put_u8(if shared_sub_available { 1 } else { 0 });
        }
        
        // Server Keep Alive (0x13)
        if let Some(server_keep_alive) = properties.server_keep_alive {
            buf.put_u8(0x13);
            buf.put_u16(server_keep_alive);
        }
        
        // Response Information (0x1A)
        if let Some(ref response_info) = properties.response_information {
            buf.put_u8(0x1A);
            self.encode_string(response_info, buf)?;
        }
        
        // Server Reference (0x1C)
        if let Some(ref server_ref) = properties.server_reference {
            buf.put_u8(0x1C);
            self.encode_string(server_ref, buf)?;
        }
        
        // Authentication Method (0x15)
        if let Some(ref auth_method) = properties.authentication_method {
            buf.put_u8(0x15);
            self.encode_string(auth_method, buf)?;
        }
        
        // Authentication Data (0x16)
        if let Some(ref auth_data) = properties.authentication_data {
            buf.put_u8(0x16);
            self.encode_bytes(auth_data, buf)?;
        }
        
        // Update properties length
        let properties_len = buf.len() - properties_start - 1;
        if properties_len > 0 {
            buf[properties_start] = properties_len as u8;
        }
        
        Ok(())
    }

    fn decode_connack_properties(&self, buf: &mut BytesMut) -> Result<ConnAckProperties> {
        let mut properties = ConnAckProperties {
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
        };
        
        // Properties length
        let properties_length = buf.get_u8() as usize;
        if properties_length == 0 {
            return Ok(properties);
        }
        
        let mut properties_buf = buf.split_to(properties_length);
        
        while properties_buf.has_remaining() {
            let property_id = properties_buf.get_u8();
            
            match property_id {
                0x11 => { // Session Expiry Interval
                    properties.session_expiry_interval = Some(properties_buf.get_u32());
                }
                0x21 => { // Receive Maximum
                    properties.receive_maximum = Some(properties_buf.get_u16());
                }
                0x24 => { // Maximum QoS
                    properties.max_qos = Some(properties_buf.get_u8());
                }
                0x25 => { // Retain Available
                    properties.retain_available = Some(properties_buf.get_u8() != 0);
                }
                0x27 => { // Maximum Packet Size
                    properties.max_packet_size = Some(properties_buf.get_u32());
                }
                0x12 => { // Assigned Client Identifier
                    properties.assigned_client_identifier = Some(self.decode_string(&mut properties_buf)?);
                }
                0x22 => { // Topic Alias Maximum
                    properties.topic_alias_maximum = Some(properties_buf.get_u16());
                }
                0x1F => { // Reason String
                    properties.reason_string = Some(self.decode_string(&mut properties_buf)?);
                }
                0x26 => { // User Properties
                    let key = self.decode_string(&mut properties_buf)?;
                    let value = self.decode_string(&mut properties_buf)?;
                    properties.user_properties.insert(key, value);
                }
                0x28 => { // Wildcard Subscription Available
                    properties.wildcard_subscription_available = Some(properties_buf.get_u8() != 0);
                }
                0x29 => { // Subscription Identifiers Available
                    properties.subscription_identifiers_available = Some(properties_buf.get_u8() != 0);
                }
                0x2A => { // Shared Subscription Available
                    properties.shared_subscription_available = Some(properties_buf.get_u8() != 0);
                }
                0x13 => { // Server Keep Alive
                    properties.server_keep_alive = Some(properties_buf.get_u16());
                }
                0x1A => { // Response Information
                    properties.response_information = Some(self.decode_string(&mut properties_buf)?);
                }
                0x1C => { // Server Reference
                    properties.server_reference = Some(self.decode_string(&mut properties_buf)?);
                }
                0x15 => { // Authentication Method
                    properties.authentication_method = Some(self.decode_string(&mut properties_buf)?);
                }
                0x16 => { // Authentication Data
                    properties.authentication_data = Some(self.decode_bytes(&mut properties_buf)?);
                }
                _ => {
                    // Unknown property, skip it
                    log::warn!("Unknown connack property ID: 0x{:02x}", property_id);
                    // Try to skip the property value (this is a simplified approach)
                    if properties_buf.has_remaining() {
                        properties_buf.advance(1);
                    }
                }
            }
        }
        
        Ok(properties)
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

    fn encode_pubrec(&self, pubrec: &PubRecPacket, buf: &mut BytesMut) -> Result<()> {
        // Packet ID
        buf.put_u16(pubrec.packet_id);
        
        // Reason code (MQTT 5.0)
        if let Some(reason_code) = pubrec.reason_code {
            buf.put_u8(reason_code);
        }
        
        // Properties (MQTT 5.0) - placeholder for now
        if let Some(_properties) = &pubrec.properties {
            // TODO: Implement properties encoding
        }
        
        Ok(())
    }

    fn decode_pubrec(&self, buf: &mut BytesMut) -> Result<PacketPayload> {
        // Packet ID
        let packet_id = buf.get_u16();
        
        // Reason code (MQTT 5.0) - default to 0 (Success)
        let reason_code = if buf.has_remaining() {
            Some(buf.get_u8())
        } else {
            None
        };
        
        // Properties (MQTT 5.0) - placeholder for now
        let properties = None;
        
        Ok(PacketPayload::PubRec(PubRecPacket {
            packet_id,
            reason_code,
            properties,
        }))
    }

    fn encode_pubrel(&self, pubrel: &PubRelPacket, buf: &mut BytesMut) -> Result<()> {
        // Packet ID
        buf.put_u16(pubrel.packet_id);
        
        // Reason code (MQTT 5.0)
        if let Some(reason_code) = pubrel.reason_code {
            buf.put_u8(reason_code);
        }
        
        // Properties (MQTT 5.0) - placeholder for now
        if let Some(_properties) = &pubrel.properties {
            // TODO: Implement properties encoding
        }
        
        Ok(())
    }

    fn decode_pubrel(&self, buf: &mut BytesMut) -> Result<PacketPayload> {
        // Packet ID
        let packet_id = buf.get_u16();
        
        // Reason code (MQTT 5.0) - default to 0 (Success)
        let reason_code = if buf.has_remaining() {
            Some(buf.get_u8())
        } else {
            None
        };
        
        // Properties (MQTT 5.0) - placeholder for now
        let properties = None;
        
        Ok(PacketPayload::PubRel(PubRelPacket {
            packet_id,
            reason_code,
            properties,
        }))
    }

    fn encode_pubcomp(&self, pubcomp: &PubCompPacket, buf: &mut BytesMut) -> Result<()> {
        // Packet ID
        buf.put_u16(pubcomp.packet_id);
        
        // Reason code (MQTT 5.0)
        if let Some(reason_code) = pubcomp.reason_code {
            buf.put_u8(reason_code);
        }
        
        // Properties (MQTT 5.0) - placeholder for now
        if let Some(_properties) = &pubcomp.properties {
            // TODO: Implement properties encoding
        }
        
        Ok(())
    }

    fn decode_pubcomp(&self, buf: &mut BytesMut) -> Result<PacketPayload> {
        // Packet ID
        let packet_id = buf.get_u16();
        
        // Reason code (MQTT 5.0) - default to 0 (Success)
        let reason_code = if buf.has_remaining() {
            Some(buf.get_u8())
        } else {
            None
        };
        
        // Properties (MQTT 5.0) - placeholder for now
        let properties = None;
        
        Ok(PacketPayload::PubComp(PubCompPacket {
            packet_id,
            reason_code,
            properties,
        }))
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

    #[test]
    fn test_encode_decode_puback_packet() {
        let codec = MqttCodec::new(4);
        
        let puback = Packet {
            header: PacketHeader {
                packet_type: PacketType::PubAck,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 2, // 2 bytes for packet ID
            },
            payload: PacketPayload::PubAck(PubAckPacket {
                packet_id: 12345,
                reason_code: None,
                properties: None,
            }),
        };
        
        // Test encoding
        let encoded = codec.encode(&puback).unwrap();
        assert!(!encoded.is_empty());
        
        // Test decoding
        let mut buf = BytesMut::from(encoded.as_ref());
        let decoded = codec.decode(&mut buf).unwrap().unwrap();
        
        assert_eq!(decoded.header.packet_type, PacketType::PubAck);
        
        if let PacketPayload::PubAck(decoded_puback) = decoded.payload {
            assert_eq!(decoded_puback.packet_id, 12345);
        } else {
            panic!("Expected PubAck payload");
        }
    }

    #[test]
    fn test_encode_decode_pubrec_packet() {
        let codec = MqttCodec::new(4);
        
        let pubrec = Packet {
            header: PacketHeader {
                packet_type: PacketType::PubRec,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 2, // 2 bytes for packet ID
            },
            payload: PacketPayload::PubRec(PubRecPacket {
                packet_id: 54321,
                reason_code: None,
                properties: None,
            }),
        };
        
        // Test encoding
        let encoded = codec.encode(&pubrec).unwrap();
        assert!(!encoded.is_empty());
        
        // Test decoding
        let mut buf = BytesMut::from(encoded.as_ref());
        let decoded = codec.decode(&mut buf).unwrap().unwrap();
        
        assert_eq!(decoded.header.packet_type, PacketType::PubRec);
        
        if let PacketPayload::PubRec(decoded_pubrec) = decoded.payload {
            assert_eq!(decoded_pubrec.packet_id, 54321);
        } else {
            panic!("Expected PubRec payload");
        }
    }

    #[test]
    fn test_encode_decode_pubrel_packet() {
        let codec = MqttCodec::new(4);
        
        let pubrel = Packet {
            header: PacketHeader {
                packet_type: PacketType::PubRel,
                dup: false,
                qos: 1, // PUBREL must use QoS 1
                retain: false,
                remaining_length: 2, // 2 bytes for packet ID
            },
            payload: PacketPayload::PubRel(PubRelPacket {
                packet_id: 54321,
                reason_code: None,
                properties: None,
            }),
        };
        
        // Test encoding
        let encoded = codec.encode(&pubrel).unwrap();
        assert!(!encoded.is_empty());
        
        // Test decoding
        let mut buf = BytesMut::from(encoded.as_ref());
        let decoded = codec.decode(&mut buf).unwrap().unwrap();
        
        assert_eq!(decoded.header.packet_type, PacketType::PubRel);
        assert_eq!(decoded.header.qos, 1); // PUBREL must have QoS 1
        
        if let PacketPayload::PubRel(decoded_pubrel) = decoded.payload {
            assert_eq!(decoded_pubrel.packet_id, 54321);
        } else {
            panic!("Expected PubRel payload");
        }
    }

    #[test]
    fn test_encode_decode_pubcomp_packet() {
        let codec = MqttCodec::new(4);
        
        let pubcomp = Packet {
            header: PacketHeader {
                packet_type: PacketType::PubComp,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 2, // 2 bytes for packet ID
            },
            payload: PacketPayload::PubComp(PubCompPacket {
                packet_id: 11111,
                reason_code: None,
                properties: None,
            }),
        };
        
        // Test encoding
        let encoded = codec.encode(&pubcomp).unwrap();
        assert!(!encoded.is_empty());
        
        // Test decoding
        let mut buf = BytesMut::from(encoded.as_ref());
        let decoded = codec.decode(&mut buf).unwrap().unwrap();
        
        assert_eq!(decoded.header.packet_type, PacketType::PubComp);
        
        if let PacketPayload::PubComp(decoded_pubcomp) = decoded.payload {
            assert_eq!(decoded_pubcomp.packet_id, 11111);
        } else {
            panic!("Expected PubComp payload");
        }
    }

    #[test]
    fn test_encode_decode_publish_qos1_packet() {
        let codec = MqttCodec::new(4);
        
        let publish = Packet {
            header: PacketHeader {
                packet_type: PacketType::Publish,
                dup: false,
                qos: 1, // QoS 1
                retain: false,
                remaining_length: 0, // Will be calculated
            },
            payload: PacketPayload::Publish(PublishPacket {
                topic_name: "test/topic".to_string(),
                packet_id: Some(12345), // QoS 1 requires packet ID
                payload: Bytes::from("Hello QoS 1"),
                properties: None,
            }),
        };
        
        // Test encoding
        let encoded = codec.encode(&publish).unwrap();
        assert!(!encoded.is_empty());
        
        // Test decoding
        let mut buf = BytesMut::from(encoded.as_ref());
        let decoded = codec.decode(&mut buf).unwrap().unwrap();
        
        assert_eq!(decoded.header.packet_type, PacketType::Publish);
        assert_eq!(decoded.header.qos, 1);
        
        if let PacketPayload::Publish(decoded_publish) = decoded.payload {
            assert_eq!(decoded_publish.topic_name, "test/topic");
            assert_eq!(decoded_publish.packet_id, Some(12345));
            assert_eq!(decoded_publish.payload, "Hello QoS 1");
        } else {
            panic!("Expected Publish payload");
        }
    }

    #[test]
    fn test_encode_decode_publish_qos2_packet() {
        let codec = MqttCodec::new(4);
        
        let publish = Packet {
            header: PacketHeader {
                packet_type: PacketType::Publish,
                dup: false,
                qos: 2, // QoS 2
                retain: false,
                remaining_length: 0, // Will be calculated
            },
            payload: PacketPayload::Publish(PublishPacket {
                topic_name: "test/topic/qos2".to_string(),
                packet_id: Some(54321), // QoS 2 requires packet ID
                payload: Bytes::from("Hello QoS 2"),
                properties: None,
            }),
        };
        
        // Test encoding
        let encoded = codec.encode(&publish).unwrap();
        assert!(!encoded.is_empty());
        
        // Test decoding
        let mut buf = BytesMut::from(encoded.as_ref());
        let decoded = codec.decode(&mut buf).unwrap().unwrap();
        
        assert_eq!(decoded.header.packet_type, PacketType::Publish);
        assert_eq!(decoded.header.qos, 2);
        
        if let PacketPayload::Publish(decoded_publish) = decoded.payload {
            assert_eq!(decoded_publish.topic_name, "test/topic/qos2");
            assert_eq!(decoded_publish.packet_id, Some(54321));
            assert_eq!(decoded_publish.payload, "Hello QoS 2");
        } else {
            panic!("Expected Publish payload");
        }
    }

    #[test]
    fn test_encode_decode_subscribe_packet() {
        let codec = MqttCodec::new(4);
        
        let subscribe = Packet {
            header: PacketHeader {
                packet_type: PacketType::Subscribe,
                dup: false,
                qos: 1, // Subscribe packets use QoS 1
                retain: false,
                remaining_length: 0, // Will be calculated
            },
            payload: PacketPayload::Subscribe(SubscribePacket {
                packet_id: 12345,
                topic_filters: vec![
                    TopicFilter {
                        topic: "topic1".to_string(),
                        qos: 0,
                        no_local: false,
                        retain_as_published: false,
                        retain_handling: 0,
                    },
                    TopicFilter {
                        topic: "topic2".to_string(),
                        qos: 1,
                        no_local: false,
                        retain_as_published: false,
                        retain_handling: 0,
                    },
                    TopicFilter {
                        topic: "topic3".to_string(),
                        qos: 2,
                        no_local: false,
                        retain_as_published: false,
                        retain_handling: 0,
                    },
                ],
                properties: None,
            }),
        };
        
        // Test encoding
        let encoded = codec.encode(&subscribe).unwrap();
        assert!(!encoded.is_empty());
        
        // Test decoding
        let mut buf = BytesMut::from(encoded.as_ref());
        let decoded = codec.decode(&mut buf).unwrap().unwrap();
        
        assert_eq!(decoded.header.packet_type, PacketType::Subscribe);
        assert_eq!(decoded.header.qos, 1);
        
        if let PacketPayload::Subscribe(decoded_subscribe) = decoded.payload {
            assert_eq!(decoded_subscribe.packet_id, 12345);
            assert_eq!(decoded_subscribe.topic_filters.len(), 3);
            assert_eq!(decoded_subscribe.topic_filters[0].topic, "topic1");
            assert_eq!(decoded_subscribe.topic_filters[0].qos, 0);
            assert_eq!(decoded_subscribe.topic_filters[1].topic, "topic2");
            assert_eq!(decoded_subscribe.topic_filters[1].qos, 1);
            assert_eq!(decoded_subscribe.topic_filters[2].topic, "topic3");
            assert_eq!(decoded_subscribe.topic_filters[2].qos, 2);
        } else {
            panic!("Expected Subscribe payload");
        }
    }

    #[test]
    fn test_encode_decode_suback_packet() {
        let codec = MqttCodec::new(4);
        
        let suback = Packet {
            header: PacketHeader {
                packet_type: PacketType::SubAck,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 0, // Will be calculated
            },
            payload: PacketPayload::SubAck(SubAckPacket {
                packet_id: 12345,
                return_codes: vec![0, 1, 2], // QoS 0, 1, 2
                properties: None,
            }),
        };
        
        // Test encoding
        let encoded = codec.encode(&suback).unwrap();
        assert!(!encoded.is_empty());
        
        // Test decoding
        let mut buf = BytesMut::from(encoded.as_ref());
        let decoded = codec.decode(&mut buf).unwrap().unwrap();
        
        assert_eq!(decoded.header.packet_type, PacketType::SubAck);
        
        if let PacketPayload::SubAck(decoded_suback) = decoded.payload {
            assert_eq!(decoded_suback.packet_id, 12345);
            assert_eq!(decoded_suback.return_codes, vec![0, 1, 2]);
        } else {
            panic!("Expected SubAck payload");
        }
    }

    #[test]
    fn test_qos_flow_integration() {
        let codec = MqttCodec::new(4);
        
        // Test complete QoS 2 flow: PUBLISH -> PUBREC -> PUBREL -> PUBCOMP
        let publish = Packet {
            header: PacketHeader {
                packet_type: PacketType::Publish,
                dup: false,
                qos: 2,
                retain: false,
                remaining_length: 0,
            },
            payload: PacketPayload::Publish(PublishPacket {
                topic_name: "test/qos2".to_string(),
                packet_id: Some(65535),
                payload: Bytes::from("QoS 2 message"),
                properties: None,
            }),
        };
        
        let pubrec = Packet {
            header: PacketHeader {
                packet_type: PacketType::PubRec,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 2,
            },
            payload: PacketPayload::PubRec(PubRecPacket {
                packet_id: 65535,
                reason_code: None,
                properties: None,
            }),
        };
        
        let pubrel = Packet {
            header: PacketHeader {
                packet_type: PacketType::PubRel,
                dup: false,
                qos: 1,
                retain: false,
                remaining_length: 2,
            },
            payload: PacketPayload::PubRel(PubRelPacket {
                packet_id: 65535,
                reason_code: None,
                properties: None,
            }),
        };
        
        let pubcomp = Packet {
            header: PacketHeader {
                packet_type: PacketType::PubComp,
                dup: false,
                qos: 0,
                retain: false,
                remaining_length: 2,
            },
            payload: PacketPayload::PubComp(PubCompPacket {
                packet_id: 65535,
                reason_code: None,
                properties: None,
            }),
        };
        
        // Test encoding and decoding of all packets in the flow
        let packets = vec![publish, pubrec, pubrel, pubcomp];
        
        for packet in packets {
            let encoded = codec.encode(&packet).unwrap();
            let mut buf = BytesMut::from(encoded.as_ref());
            let decoded = codec.decode(&mut buf).unwrap().unwrap();
            
            assert_eq!(decoded.header.packet_type, packet.header.packet_type);
            assert_eq!(decoded.header.qos, packet.header.qos);
            
            // Verify packet ID consistency for QoS 2 flow
            if let PacketPayload::Publish(p) = &packet.payload {
                if let PacketPayload::Publish(d) = &decoded.payload {
                    assert_eq!(d.packet_id, p.packet_id);
                }
            }
        }
    }
} 