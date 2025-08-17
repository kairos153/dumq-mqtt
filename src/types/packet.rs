//! # MQTT Packet Types
//! 
//! This module defines the core MQTT packet structures including packet types,
//! headers, and payloads.

use super::connect::ConnectPacket;
use super::publish::PublishPacket;
use super::subscribe::SubscribePacket;

/// MQTT packet types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    Connect = 1,
    ConnAck = 2,
    Publish = 3,
    PubAck = 4,
    PubRec = 5,
    PubRel = 6,
    PubComp = 7,
    Subscribe = 8,
    SubAck = 9,
    Unsubscribe = 10,
    UnsubAck = 11,
    PingReq = 12,
    PingResp = 13,
    Disconnect = 14,
    Auth = 15, // MQTT 5.0 only
}

impl PacketType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(PacketType::Connect),
            2 => Some(PacketType::ConnAck),
            3 => Some(PacketType::Publish),
            4 => Some(PacketType::PubAck),
            5 => Some(PacketType::PubRec),
            6 => Some(PacketType::PubRel),
            7 => Some(PacketType::PubComp),
            8 => Some(PacketType::Subscribe),
            9 => Some(PacketType::SubAck),
            10 => Some(PacketType::Unsubscribe),
            11 => Some(PacketType::UnsubAck),
            12 => Some(PacketType::PingReq),
            13 => Some(PacketType::PingResp),
            14 => Some(PacketType::Disconnect),
            15 => Some(PacketType::Auth),
            _ => None,
        }
    }
}

/// MQTT packet header
#[derive(Debug, Clone)]
pub struct PacketHeader {
    pub packet_type: PacketType,
    pub dup: bool,
    pub qos: u8,
    pub retain: bool,
    pub remaining_length: usize,
}

/// MQTT packet
#[derive(Debug, Clone)]
pub struct Packet {
    pub header: PacketHeader,
    pub payload: PacketPayload,
}

/// MQTT packet payload
#[derive(Debug, Clone)]
pub enum PacketPayload {
    Connect(ConnectPacket),
    ConnAck(super::connect::ConnAckPacket),
    Publish(PublishPacket),
    PubAck(super::publish::PubAckPacket),
    PubRec(super::publish::PubRecPacket),
    PubRel(super::publish::PubRelPacket),
    PubComp(super::publish::PubCompPacket),
    Subscribe(SubscribePacket),
    SubAck(super::subscribe::SubAckPacket),
    Unsubscribe(super::subscribe::UnsubscribePacket),
    UnsubAck(super::subscribe::UnsubAckPacket),
    PingReq,
    PingResp,
    Disconnect(super::connect::DisconnectPacket),
    Auth(super::connect::AuthPacket), // MQTT 5.0 only
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_type_from_u8() {
        assert_eq!(PacketType::from_u8(1), Some(PacketType::Connect));
        assert_eq!(PacketType::from_u8(2), Some(PacketType::ConnAck));
        assert_eq!(PacketType::from_u8(3), Some(PacketType::Publish));
        assert_eq!(PacketType::from_u8(4), Some(PacketType::PubAck));
        assert_eq!(PacketType::from_u8(5), Some(PacketType::PubRec));
        assert_eq!(PacketType::from_u8(6), Some(PacketType::PubRel));
        assert_eq!(PacketType::from_u8(7), Some(PacketType::PubComp));
        assert_eq!(PacketType::from_u8(8), Some(PacketType::Subscribe));
        assert_eq!(PacketType::from_u8(9), Some(PacketType::SubAck));
        assert_eq!(PacketType::from_u8(10), Some(PacketType::Unsubscribe));
        assert_eq!(PacketType::from_u8(11), Some(PacketType::UnsubAck));
        assert_eq!(PacketType::from_u8(12), Some(PacketType::PingReq));
        assert_eq!(PacketType::from_u8(13), Some(PacketType::PingResp));
        assert_eq!(PacketType::from_u8(14), Some(PacketType::Disconnect));
        assert_eq!(PacketType::from_u8(15), Some(PacketType::Auth));
        assert_eq!(PacketType::from_u8(0), None);
        assert_eq!(PacketType::from_u8(16), None);
        assert_eq!(PacketType::from_u8(255), None);
    }

    #[test]
    fn test_packet_type_values() {
        assert_eq!(PacketType::Connect as u8, 1);
        assert_eq!(PacketType::ConnAck as u8, 2);
        assert_eq!(PacketType::Publish as u8, 3);
        assert_eq!(PacketType::PubAck as u8, 4);
        assert_eq!(PacketType::PubRec as u8, 5);
        assert_eq!(PacketType::PubRel as u8, 6);
        assert_eq!(PacketType::PubComp as u8, 7);
        assert_eq!(PacketType::Subscribe as u8, 8);
        assert_eq!(PacketType::SubAck as u8, 9);
        assert_eq!(PacketType::Unsubscribe as u8, 10);
        assert_eq!(PacketType::UnsubAck as u8, 11);
        assert_eq!(PacketType::PingReq as u8, 12);
        assert_eq!(PacketType::PingResp as u8, 13);
        assert_eq!(PacketType::Disconnect as u8, 14);
        assert_eq!(PacketType::Auth as u8, 15);
    }

    #[test]
    fn test_packet_header_creation() {
        let header = PacketHeader {
            packet_type: PacketType::Publish,
            dup: false,
            qos: 1,
            retain: false,
            remaining_length: 100,
        };

        assert_eq!(header.packet_type, PacketType::Publish);
        assert_eq!(header.dup, false);
        assert_eq!(header.qos, 1);
        assert_eq!(header.retain, false);
        assert_eq!(header.remaining_length, 100);
    }

    #[test]
    fn test_packet_header_clone() {
        let header = PacketHeader {
            packet_type: PacketType::Subscribe,
            dup: false,
            qos: 1,
            retain: false,
            remaining_length: 200,
        };

        let cloned_header = header.clone();
        assert_eq!(header.packet_type, cloned_header.packet_type);
        assert_eq!(header.dup, cloned_header.dup);
        assert_eq!(header.qos, cloned_header.qos);
        assert_eq!(header.retain, cloned_header.retain);
        assert_eq!(header.remaining_length, cloned_header.remaining_length);
    }

    #[test]
    fn test_packet_creation() {
        let header = PacketHeader {
            packet_type: PacketType::Publish,
            dup: false,
            qos: 1,
            retain: false,
            remaining_length: 50,
        };

        let payload = PacketPayload::Publish(PublishPacket {
            topic_name: "test/topic".to_string(),
            packet_id: Some(123),
            payload: bytes::Bytes::from("Hello"),
            properties: None,
        });

        let packet = Packet {
            header: header.clone(),
            payload: payload.clone(),
        };

        assert_eq!(packet.header.packet_type, PacketType::Publish);
        assert_eq!(packet.header.qos, 1);
        assert_eq!(packet.header.remaining_length, 50);
        
        if let PacketPayload::Publish(publish) = packet.payload {
            assert_eq!(publish.topic_name, "test/topic");
            assert_eq!(publish.packet_id, Some(123));
        } else {
            panic!("Expected Publish payload");
        }
    }

    #[test]
    fn test_packet_clone() {
        let header = PacketHeader {
            packet_type: PacketType::Subscribe,
            dup: false,
            qos: 1,
            retain: false,
            remaining_length: 100,
        };

        let payload = PacketPayload::Subscribe(SubscribePacket {
            packet_id: 456,
            topic_filters: vec![crate::types::subscribe::TopicFilter {
                topic: "test/topic".to_string(),
                qos: 1,
                no_local: false,
                retain_as_published: false,
                retain_handling: 0,
            }],
            properties: None,
        });

        let packet = Packet { header, payload };
        let cloned_packet = packet.clone();

        assert_eq!(packet.header.packet_type, cloned_packet.header.packet_type);
        assert_eq!(packet.header.qos, cloned_packet.header.qos);
    }

    #[test]
    fn test_edge_cases() {
        // Test with maximum values
        let header = PacketHeader {
            packet_type: PacketType::Publish,
            dup: true,
            qos: 2,
            retain: true,
            remaining_length: usize::MAX,
        };

        assert_eq!(header.dup, true);
        assert_eq!(header.qos, 2);
        assert_eq!(header.retain, true);
        assert_eq!(header.remaining_length, usize::MAX);
    }
}
