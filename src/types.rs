use bytes::{Bytes};
use std::collections::HashMap;

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
    ConnAck(ConnAckPacket),
    Publish(PublishPacket),
    PubAck(PubAckPacket),
    PubRec(PubRecPacket),
    PubRel(PubRelPacket),
    PubComp(PubCompPacket),
    Subscribe(SubscribePacket),
    SubAck(SubAckPacket),
    Unsubscribe(UnsubscribePacket),
    UnsubAck(UnsubAckPacket),
    PingReq,
    PingResp,
    Disconnect(DisconnectPacket),
    Auth(AuthPacket), // MQTT 5.0 only
}

/// Connect packet
#[derive(Debug, Clone)]
pub struct ConnectPacket {
    pub protocol_name: String,
    pub protocol_version: u8,
    pub clean_session: bool,
    pub will_flag: bool,
    pub will_qos: u8,
    pub will_retain: bool,
    pub password_flag: bool,
    pub username_flag: bool,
    pub keep_alive: u16,
    pub client_id: String,
    pub will_topic: Option<String>,
    pub will_message: Option<Bytes>,
    pub username: Option<String>,
    pub password: Option<String>,
    // MQTT 5.0 properties
    pub properties: Option<ConnectProperties>,
}

/// Connect acknowledgment packet
#[derive(Debug, Clone)]
pub struct ConnAckPacket {
    pub session_present: bool,
    pub return_code: ConnectReturnCode,
    // MQTT 5.0 properties
    pub properties: Option<ConnAckProperties>,
}

/// Connect return codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectReturnCode {
    Accepted = 0,
    UnacceptableProtocolVersion = 1,
    IdentifierRejected = 2,
    ServerUnavailable = 3,
    BadUsernameOrPassword = 4,
    NotAuthorized = 5,
}

impl ConnectReturnCode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ConnectReturnCode::Accepted),
            1 => Some(ConnectReturnCode::UnacceptableProtocolVersion),
            2 => Some(ConnectReturnCode::IdentifierRejected),
            3 => Some(ConnectReturnCode::ServerUnavailable),
            4 => Some(ConnectReturnCode::BadUsernameOrPassword),
            5 => Some(ConnectReturnCode::NotAuthorized),
            _ => None,
        }
    }
}

/// Publish packet
#[derive(Debug, Clone)]
pub struct PublishPacket {
    pub topic_name: String,
    pub packet_id: Option<u16>,
    pub payload: Bytes,
    // MQTT 5.0 properties
    pub properties: Option<PublishProperties>,
}

/// Publish acknowledgment packet
#[derive(Debug, Clone)]
pub struct PubAckPacket {
    pub packet_id: u16,
    pub reason_code: Option<u8>, // MQTT 5.0
    pub properties: Option<PubAckProperties>, // MQTT 5.0
}

/// Publish receive packet
#[derive(Debug, Clone)]
pub struct PubRecPacket {
    pub packet_id: u16,
    pub reason_code: Option<u8>, // MQTT 5.0
    pub properties: Option<PubRecProperties>, // MQTT 5.0
}

/// Publish release packet
#[derive(Debug, Clone)]
pub struct PubRelPacket {
    pub packet_id: u16,
    pub reason_code: Option<u8>, // MQTT 5.0
    pub properties: Option<PubRelProperties>, // MQTT 5.0
}

/// Publish complete packet
#[derive(Debug, Clone)]
pub struct PubCompPacket {
    pub packet_id: u16,
    pub reason_code: Option<u8>, // MQTT 5.0
    pub properties: Option<PubCompProperties>, // MQTT 5.0
}

/// Subscribe packet
#[derive(Debug, Clone)]
pub struct SubscribePacket {
    pub packet_id: u16,
    pub topic_filters: Vec<TopicFilter>,
    pub properties: Option<SubscribeProperties>, // MQTT 5.0
}

/// Topic filter with QoS
#[derive(Debug, Clone)]
pub struct TopicFilter {
    pub topic: String,
    pub qos: u8,
    pub no_local: bool, // MQTT 5.0
    pub retain_as_published: bool, // MQTT 5.0
    pub retain_handling: u8, // MQTT 5.0
}

/// Subscribe acknowledgment packet
#[derive(Debug, Clone)]
pub struct SubAckPacket {
    pub packet_id: u16,
    pub return_codes: Vec<u8>,
    pub properties: Option<SubAckProperties>, // MQTT 5.0
}

/// Unsubscribe packet
#[derive(Debug, Clone)]
pub struct UnsubscribePacket {
    pub packet_id: u16,
    pub topic_filters: Vec<String>,
    pub properties: Option<UnsubscribeProperties>, // MQTT 5.0
}

/// Unsubscribe acknowledgment packet
#[derive(Debug, Clone)]
pub struct UnsubAckPacket {
    pub packet_id: u16,
    pub reason_codes: Vec<u8>, // MQTT 5.0
    pub properties: Option<UnsubAckProperties>, // MQTT 5.0
}

/// Disconnect packet
#[derive(Debug, Clone)]
pub struct DisconnectPacket {
    pub reason_code: Option<u8>, // MQTT 5.0
    pub properties: Option<DisconnectProperties>, // MQTT 5.0
}

/// Authentication packet (MQTT 5.0 only)
#[derive(Debug, Clone)]
pub struct AuthPacket {
    pub reason_code: u8,
    pub properties: Option<AuthProperties>,
}

// MQTT 5.0 Properties
#[derive(Debug, Clone)]
pub struct ConnectProperties {
    pub session_expiry_interval: Option<u32>,
    pub receive_maximum: Option<u16>,
    pub max_packet_size: Option<u32>,
    pub topic_alias_maximum: Option<u16>,
    pub request_response_information: Option<bool>,
    pub request_problem_information: Option<bool>,
    pub user_properties: HashMap<String, String>,
    pub authentication_method: Option<String>,
    pub authentication_data: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub struct ConnAckProperties {
    pub session_expiry_interval: Option<u32>,
    pub receive_maximum: Option<u16>,
    pub max_qos: Option<u8>,
    pub retain_available: Option<bool>,
    pub max_packet_size: Option<u32>,
    pub assigned_client_identifier: Option<String>,
    pub topic_alias_maximum: Option<u16>,
    pub reason_string: Option<String>,
    pub user_properties: HashMap<String, String>,
    pub wildcard_subscription_available: Option<bool>,
    pub subscription_identifiers_available: Option<bool>,
    pub shared_subscription_available: Option<bool>,
    pub server_keep_alive: Option<u16>,
    pub response_information: Option<String>,
    pub server_reference: Option<String>,
    pub authentication_method: Option<String>,
    pub authentication_data: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub struct PublishProperties {
    pub payload_format_indicator: Option<u8>,
    pub message_expiry_interval: Option<u32>,
    pub topic_alias: Option<u16>,
    pub response_topic: Option<String>,
    pub correlation_data: Option<Bytes>,
    pub user_properties: HashMap<String, String>,
    pub subscription_identifier: Option<u32>,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PubAckProperties {
    pub reason_string: Option<String>,
    pub user_properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PubRecProperties {
    pub reason_string: Option<String>,
    pub user_properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PubRelProperties {
    pub reason_string: Option<String>,
    pub user_properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PubCompProperties {
    pub reason_string: Option<String>,
    pub user_properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SubscribeProperties {
    pub subscription_identifier: Option<u32>,
    pub user_properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SubAckProperties {
    pub reason_string: Option<String>,
    pub user_properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct UnsubscribeProperties {
    pub user_properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct UnsubAckProperties {
    pub reason_string: Option<String>,
    pub user_properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct DisconnectProperties {
    pub session_expiry_interval: Option<u32>,
    pub reason_string: Option<String>,
    pub user_properties: HashMap<String, String>,
    pub server_reference: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthProperties {
    pub authentication_method: Option<String>,
    pub authentication_data: Option<Bytes>,
    pub reason_string: Option<String>,
    pub user_properties: HashMap<String, String>,
}

/// MQTT message
#[derive(Debug, Clone)]
pub struct Message {
    pub topic: String,
    pub payload: Bytes,
    pub qos: u8,
    pub retain: bool,
    pub dup: bool,
    pub packet_id: Option<u16>,
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
    }

    #[test]
    fn test_connect_return_code_from_u8() {
        assert_eq!(ConnectReturnCode::from_u8(0), Some(ConnectReturnCode::Accepted));
        assert_eq!(ConnectReturnCode::from_u8(1), Some(ConnectReturnCode::UnacceptableProtocolVersion));
        assert_eq!(ConnectReturnCode::from_u8(2), Some(ConnectReturnCode::IdentifierRejected));
        assert_eq!(ConnectReturnCode::from_u8(3), Some(ConnectReturnCode::ServerUnavailable));
        assert_eq!(ConnectReturnCode::from_u8(4), Some(ConnectReturnCode::BadUsernameOrPassword));
        assert_eq!(ConnectReturnCode::from_u8(5), Some(ConnectReturnCode::NotAuthorized));
        assert_eq!(ConnectReturnCode::from_u8(6), None);
    }

    #[test]
    fn test_packet_header_creation() {
        let header = PacketHeader {
            packet_type: PacketType::Connect,
            dup: false,
            qos: 0,
            retain: false,
            remaining_length: 10,
        };
        
        assert_eq!(header.packet_type, PacketType::Connect);
        assert_eq!(header.dup, false);
        assert_eq!(header.qos, 0);
        assert_eq!(header.retain, false);
        assert_eq!(header.remaining_length, 10);
    }

    #[test]
    fn test_connect_packet_creation() {
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
        
        assert_eq!(connect.protocol_name, "MQTT");
        assert_eq!(connect.protocol_version, 4);
        assert_eq!(connect.clean_session, true);
        assert_eq!(connect.client_id, "test_client");
        assert_eq!(connect.keep_alive, 60);
    }

    #[test]
    fn test_message_creation() {
        let message = Message {
            topic: "test/topic".to_string(),
            payload: Bytes::from("Hello, MQTT!"),
            qos: 1,
            retain: false,
            dup: false,
            packet_id: Some(123),
        };
        
        assert_eq!(message.topic, "test/topic");
        assert_eq!(message.payload, Bytes::from("Hello, MQTT!"));
        assert_eq!(message.qos, 1);
        assert_eq!(message.retain, false);
        assert_eq!(message.dup, false);
        assert_eq!(message.packet_id, Some(123));
    }

    #[test]
    fn test_topic_filter_creation() {
        let filter = TopicFilter {
            topic: "test/+/topic".to_string(),
            qos: 1,
            no_local: false,
            retain_as_published: false,
            retain_handling: 0,
        };
        
        assert_eq!(filter.topic, "test/+/topic");
        assert_eq!(filter.qos, 1);
        assert_eq!(filter.no_local, false);
        assert_eq!(filter.retain_as_published, false);
        assert_eq!(filter.retain_handling, 0);
    }
} 