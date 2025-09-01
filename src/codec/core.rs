//! # MQTT Codec Core
//! 
//! This module contains the main MqttCodec struct and core functionality for
//! encoding and decoding MQTT packets. It orchestrates the various packet-specific
//! codec modules to provide a unified interface.

use crate::error::{Error, Result};
use crate::types::{Packet, PacketType, PacketHeader, PacketPayload, DisconnectPacket, AuthPacket};
use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::utils::decode_remaining_length;
use super::connect::{encode_connect, decode_connect, encode_connack, decode_connack};
use super::publish::{encode_publish, decode_publish, encode_puback, decode_puback, encode_pubrec, decode_pubrec, encode_pubrel, decode_pubrel, encode_pubcomp, decode_pubcomp};
use super::subscribe::{encode_subscribe, decode_subscribe, encode_suback, decode_suback, encode_unsubscribe, decode_unsubscribe, encode_unsuback, decode_unsuback};

/// MQTT packet encoder/decoder
/// 
/// The MqttCodec handles the binary encoding and decoding of MQTT packets
/// according to the MQTT specification. It supports both MQTT 3.1.1 and MQTT 5.0
/// protocols with automatic version-specific feature handling.
#[derive(Clone, Debug)]
pub struct MqttCodec {
    protocol_version: u8,
}

impl MqttCodec {
    /// Create a new MqttCodec instance
    /// 
    /// # Arguments
    /// 
    /// * `protocol_version` - MQTT protocol version (4 for 3.1.1, 5 for 5.0)
    pub fn new(protocol_version: u8) -> Self {
        Self { protocol_version }
    }

    /// Encode a packet into bytes
    /// 
    /// Converts an MQTT packet structure into its binary wire format representation.
    /// 
    /// # Arguments
    /// 
    /// * `packet` - The MQTT packet to encode
    /// 
    /// # Returns
    /// 
    /// Returns the encoded packet as bytes, or an error if encoding fails.
    pub fn encode(&self, packet: &Packet) -> Result<Bytes> {
        let mut buf = BytesMut::new();
        self.encode_packet(packet, &mut buf)?;
        Ok(buf.freeze())
    }

    /// Decode bytes into a packet
    /// 
    /// Attempts to decode a complete MQTT packet from the provided buffer.
    /// Returns None if the buffer contains insufficient data for a complete packet.
    /// 
    /// # Arguments
    /// 
    /// * `buf` - Buffer containing the packet data
    /// 
    /// # Returns
    /// 
    /// Returns Some(packet) if a complete packet was decoded, None if more data
    /// is needed, or an error if the data is malformed.
    pub fn decode(&self, buf: &mut BytesMut) -> Result<Option<Packet>> {
        if buf.len() < 2 {
            return Ok(None);
        }

        log::debug!("Decoding packet, buffer size: {}", buf.len());

        // Read fixed header
        let first_byte = buf.get_u8();
        let packet_type = PacketType::from_u8(first_byte >> 4)
            .ok_or_else(|| Error::InvalidPacket("Invalid packet type".to_string()))?;
        
        let dup = (first_byte & 0x08) != 0;
        let qos = (first_byte & 0x06) >> 1;
        let retain = (first_byte & 0x01) != 0;

        log::debug!("Packet type: {:?}, dup: {}, qos: {}, retain: {}", packet_type, dup, qos, retain);

        // Read remaining length
        let remaining_length = decode_remaining_length(buf)?;
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
        let payload = if remaining_length > 0 {
            if buf.len() < remaining_length {
                return Err(Error::InvalidPacket(format!("Insufficient data for payload: need {}, have {}", remaining_length, buf.len())));
            }
            let mut payload_buf = buf.split_to(remaining_length);
            self.decode_payload(&header, &mut payload_buf)?
        } else {
            // No payload for this packet type
            self.decode_payload(&header, &mut BytesMut::new())?
        };

        let packet = Packet { header, payload };
        Ok(Some(packet))
    }

    /// Encode a complete packet (fixed header + variable header + payload)
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
        super::utils::encode_remaining_length(payload_len, buf)?;
        
        // Append payload
        buf.extend_from_slice(&payload_buf);

        Ok(())
    }

    /// Encode packet payload based on packet type
    fn encode_payload(&self, payload: &PacketPayload, buf: &mut BytesMut) -> Result<()> {
        match payload {
            PacketPayload::Connect(connect) => encode_connect(connect, buf, self.protocol_version),
            PacketPayload::ConnAck(connack) => encode_connack(connack, buf, self.protocol_version),
            PacketPayload::Publish(publish) => encode_publish(publish, buf, self.protocol_version),
            PacketPayload::PubAck(puback) => encode_puback(puback, buf, self.protocol_version),
            PacketPayload::PubRec(pubrec) => encode_pubrec(pubrec, buf, self.protocol_version),
            PacketPayload::PubRel(pubrel) => encode_pubrel(pubrel, buf, self.protocol_version),
            PacketPayload::PubComp(pubcomp) => encode_pubcomp(pubcomp, buf, self.protocol_version),
            PacketPayload::Subscribe(subscribe) => encode_subscribe(subscribe, buf, self.protocol_version),
            PacketPayload::SubAck(suback) => encode_suback(suback, buf, self.protocol_version),
            PacketPayload::Unsubscribe(unsubscribe) => encode_unsubscribe(unsubscribe, buf, self.protocol_version),
            PacketPayload::UnsubAck(unsuback) => encode_unsuback(unsuback, buf, self.protocol_version),
            PacketPayload::PingReq => Ok(()),
            PacketPayload::PingResp => Ok(()),
            PacketPayload::Disconnect(disconnect) => self.encode_disconnect(disconnect, buf),
            PacketPayload::Auth(auth) => self.encode_auth(auth, buf),
        }
    }

    /// Decode packet payload based on packet type
    fn decode_payload(&self, header: &PacketHeader, buf: &mut BytesMut) -> Result<PacketPayload> {
        match header.packet_type {
            PacketType::Connect => decode_connect(buf, self.protocol_version),
            PacketType::ConnAck => decode_connack(buf, self.protocol_version),
            PacketType::Publish => decode_publish(header, buf, self.protocol_version),
            PacketType::PubAck => decode_puback(buf, self.protocol_version),
            PacketType::PubRec => decode_pubrec(buf, self.protocol_version),
            PacketType::PubRel => decode_pubrel(buf, self.protocol_version),
            PacketType::PubComp => decode_pubcomp(buf, self.protocol_version),
            PacketType::Subscribe => decode_subscribe(buf, self.protocol_version),
            PacketType::SubAck => decode_suback(buf, self.protocol_version),
            PacketType::Unsubscribe => decode_unsubscribe(buf, self.protocol_version),
            PacketType::UnsubAck => decode_unsuback(buf, self.protocol_version),
            PacketType::PingReq => Ok(PacketPayload::PingReq),
            PacketType::PingResp => Ok(PacketPayload::PingResp),
            PacketType::Disconnect => self.decode_disconnect(buf),
            PacketType::Auth => self.decode_auth(buf),
        }
    }

    /// Encode Disconnect packet payload
    fn encode_disconnect(&self, _disconnect: &DisconnectPacket, _buf: &mut BytesMut) -> Result<()> {
        // DISCONNECT packet has no payload in MQTT 3.1.1
        // TODO: Implement MQTT 5.0 disconnect properties
        Ok(())
    }

    /// Decode Disconnect packet payload
    fn decode_disconnect(&self, _buf: &mut BytesMut) -> Result<PacketPayload> {
        // DISCONNECT packet has no payload in MQTT 3.1.1
        // TODO: Implement MQTT 5.0 disconnect properties
        Ok(PacketPayload::Disconnect(DisconnectPacket {
            reason_code: None,
            properties: None,
        }))
    }

    /// Encode Auth packet payload
    fn encode_auth(&self, _auth: &AuthPacket, _buf: &mut BytesMut) -> Result<()> {
        // TODO: Implement auth encoding
        Ok(())
    }

    /// Decode Auth packet payload
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
