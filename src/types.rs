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
    use bytes::Bytes;

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
    fn test_connect_return_code_from_u8() {
        assert_eq!(ConnectReturnCode::from_u8(0), Some(ConnectReturnCode::Accepted));
        assert_eq!(ConnectReturnCode::from_u8(1), Some(ConnectReturnCode::UnacceptableProtocolVersion));
        assert_eq!(ConnectReturnCode::from_u8(2), Some(ConnectReturnCode::IdentifierRejected));
        assert_eq!(ConnectReturnCode::from_u8(3), Some(ConnectReturnCode::ServerUnavailable));
        assert_eq!(ConnectReturnCode::from_u8(4), Some(ConnectReturnCode::BadUsernameOrPassword));
        assert_eq!(ConnectReturnCode::from_u8(5), Some(ConnectReturnCode::NotAuthorized));
        assert_eq!(ConnectReturnCode::from_u8(6), None);
        assert_eq!(ConnectReturnCode::from_u8(255), None);
    }

    #[test]
    fn test_connect_return_code_values() {
        assert_eq!(ConnectReturnCode::Accepted as u8, 0);
        assert_eq!(ConnectReturnCode::UnacceptableProtocolVersion as u8, 1);
        assert_eq!(ConnectReturnCode::IdentifierRejected as u8, 2);
        assert_eq!(ConnectReturnCode::ServerUnavailable as u8, 3);
        assert_eq!(ConnectReturnCode::BadUsernameOrPassword as u8, 4);
        assert_eq!(ConnectReturnCode::NotAuthorized as u8, 5);
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
    fn test_connect_packet_creation() {
        let connect_packet = ConnectPacket {
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

        assert_eq!(connect_packet.protocol_name, "MQTT");
        assert_eq!(connect_packet.protocol_version, 4);
        assert_eq!(connect_packet.clean_session, true);
        assert_eq!(connect_packet.will_flag, false);
        assert_eq!(connect_packet.will_qos, 0);
        assert_eq!(connect_packet.will_retain, false);
        assert_eq!(connect_packet.password_flag, false);
        assert_eq!(connect_packet.username_flag, false);
        assert_eq!(connect_packet.keep_alive, 60);
        assert_eq!(connect_packet.client_id, "test_client");
        assert_eq!(connect_packet.will_topic, None);
        assert_eq!(connect_packet.will_message, None);
        assert_eq!(connect_packet.username, None);
        assert_eq!(connect_packet.password, None);
        assert!(connect_packet.properties.is_none());
    }

    #[test]
    fn test_connect_packet_with_will() {
        let will_message = Bytes::from("test will message");
        let connect_packet = ConnectPacket {
            protocol_name: "MQTT".to_string(),
            protocol_version: 5,
            clean_session: false,
            will_flag: true,
            will_qos: 1,
            will_retain: true,
            password_flag: true,
            username_flag: true,
            keep_alive: 120,
            client_id: "will_client".to_string(),
            will_topic: Some("test/will".to_string()),
            will_message: Some(will_message.clone()),
            username: Some("test_user".to_string()),
            password: Some("test_pass".to_string()),
            properties: None,
        };

        assert_eq!(connect_packet.will_flag, true);
        assert_eq!(connect_packet.will_topic, Some("test/will".to_string()));
        assert_eq!(connect_packet.will_message, Some(will_message));
        assert_eq!(connect_packet.username, Some("test_user".to_string()));
        assert_eq!(connect_packet.password, Some("test_pass".to_string()));
    }

    #[test]
    fn test_conn_ack_packet() {
        let conn_ack = ConnAckPacket {
            session_present: true,
            return_code: ConnectReturnCode::Accepted,
            properties: None,
        };

        assert_eq!(conn_ack.session_present, true);
        assert_eq!(conn_ack.return_code, ConnectReturnCode::Accepted);
        assert!(conn_ack.properties.is_none());
    }

    #[test]
    fn test_publish_packet() {
        let payload = Bytes::from("Hello MQTT!");
        let publish_packet = PublishPacket {
            topic_name: "test/topic".to_string(),
            packet_id: Some(123),
            payload: payload.clone(),
            properties: None,
        };

        assert_eq!(publish_packet.topic_name, "test/topic");
        assert_eq!(publish_packet.packet_id, Some(123));
        assert_eq!(publish_packet.payload, payload);
        assert!(publish_packet.properties.is_none());
    }

    #[test]
    fn test_publish_packet_qos0() {
        let payload = Bytes::from("QoS 0 message");
        let publish_packet = PublishPacket {
            topic_name: "test/qos0".to_string(),
            packet_id: None, // QoS 0 doesn't need packet ID
            payload: payload.clone(),
            properties: None,
        };

        assert_eq!(publish_packet.packet_id, None);
        assert_eq!(publish_packet.payload, payload);
    }

    #[test]
    fn test_pub_ack_packet() {
        let pub_ack = PubAckPacket {
            packet_id: 456,
            reason_code: Some(0),
            properties: None,
        };

        assert_eq!(pub_ack.packet_id, 456);
        assert_eq!(pub_ack.reason_code, Some(0));
        assert!(pub_ack.properties.is_none());
    }

    #[test]
    fn test_subscribe_packet() {
        let topic_filters = vec![
            TopicFilter {
                topic: "test/topic1".to_string(),
                qos: 0,
                no_local: false,
                retain_as_published: false,
                retain_handling: 0,
            },
            TopicFilter {
                topic: "test/topic2".to_string(),
                qos: 1,
                no_local: true,
                retain_as_published: true,
                retain_handling: 1,
            },
        ];

        let subscribe_packet = SubscribePacket {
            packet_id: 789,
            topic_filters: topic_filters.clone(),
            properties: None,
        };

        assert_eq!(subscribe_packet.packet_id, 789);
        assert_eq!(subscribe_packet.topic_filters.len(), 2);
        assert_eq!(subscribe_packet.topic_filters[0].topic, "test/topic1");
        assert_eq!(subscribe_packet.topic_filters[0].qos, 0);
        assert_eq!(subscribe_packet.topic_filters[1].topic, "test/topic2");
        assert_eq!(subscribe_packet.topic_filters[1].qos, 1);
    }

    #[test]
    fn test_sub_ack_packet() {
        let return_codes = vec![0, 1, 2, 128]; // 128 = failure
        let sub_ack = SubAckPacket {
            packet_id: 101,
            return_codes: return_codes.clone(),
            properties: None,
        };

        assert_eq!(sub_ack.packet_id, 101);
        assert_eq!(sub_ack.return_codes, return_codes);
        assert!(sub_ack.properties.is_none());
    }

    #[test]
    fn test_unsubscribe_packet() {
        let topic_filters = vec!["test/topic1".to_string(), "test/topic2".to_string()];
        let unsubscribe_packet = UnsubscribePacket {
            packet_id: 202,
            topic_filters: topic_filters.clone(),
            properties: None,
        };

        assert_eq!(unsubscribe_packet.packet_id, 202);
        assert_eq!(unsubscribe_packet.topic_filters, topic_filters);
        assert!(unsubscribe_packet.properties.is_none());
    }

    #[test]
    fn test_unsub_ack_packet() {
        let reason_codes = vec![0, 17]; // 0 = success, 17 = no subscription existed
        let unsub_ack = UnsubAckPacket {
            packet_id: 303,
            reason_codes: reason_codes.clone(),
            properties: None,
        };

        assert_eq!(unsub_ack.packet_id, 303);
        assert_eq!(unsub_ack.reason_codes, reason_codes);
        assert!(unsub_ack.properties.is_none());
    }

    #[test]
    fn test_disconnect_packet() {
        let disconnect = DisconnectPacket {
            reason_code: Some(0),
            properties: None,
        };

        assert_eq!(disconnect.reason_code, Some(0));
        assert!(disconnect.properties.is_none());
    }

    #[test]
    fn test_auth_packet() {
        let auth = AuthPacket {
            reason_code: 0,
            properties: None,
        };

        assert_eq!(auth.reason_code, 0);
        assert!(auth.properties.is_none());
    }

    #[test]
    fn test_connect_properties() {
        let mut user_properties = HashMap::new();
        user_properties.insert("key1".to_string(), "value1".to_string());
        user_properties.insert("key2".to_string(), "value2".to_string());

        let properties = ConnectProperties {
            session_expiry_interval: Some(3600),
            receive_maximum: Some(100),
            max_packet_size: Some(1024),
            topic_alias_maximum: Some(10),
            request_response_information: Some(true),
            request_problem_information: Some(false),
            user_properties: user_properties.clone(),
            authentication_method: Some("PLAIN".to_string()),
            authentication_data: Some(Bytes::from("auth_data")),
        };

        assert_eq!(properties.session_expiry_interval, Some(3600));
        assert_eq!(properties.receive_maximum, Some(100));
        assert_eq!(properties.max_packet_size, Some(1024));
        assert_eq!(properties.topic_alias_maximum, Some(10));
        assert_eq!(properties.request_response_information, Some(true));
        assert_eq!(properties.request_problem_information, Some(false));
        assert_eq!(properties.user_properties.len(), 2);
        assert_eq!(properties.authentication_method, Some("PLAIN".to_string()));
        assert_eq!(properties.authentication_data, Some(Bytes::from("auth_data")));
    }

    #[test]
    fn test_publish_properties() {
        let mut user_properties = HashMap::new();
        user_properties.insert("content-type".to_string(), "text/plain".to_string());

        let properties = PublishProperties {
            payload_format_indicator: Some(1),
            message_expiry_interval: Some(86400),
            topic_alias: Some(5),
            response_topic: Some("response/topic".to_string()),
            correlation_data: Some(Bytes::from("correlation_id")),
            user_properties: user_properties.clone(),
            subscription_identifier: Some(123),
            content_type: Some("application/json".to_string()),
        };

        assert_eq!(properties.payload_format_indicator, Some(1));
        assert_eq!(properties.message_expiry_interval, Some(86400));
        assert_eq!(properties.topic_alias, Some(5));
        assert_eq!(properties.response_topic, Some("response/topic".to_string()));
        assert_eq!(properties.correlation_data, Some(Bytes::from("correlation_id")));
        assert_eq!(properties.user_properties.len(), 1);
        assert_eq!(properties.subscription_identifier, Some(123));
        assert_eq!(properties.content_type, Some("application/json".to_string()));
    }

    #[test]
    fn test_message_creation() {
        let payload = Bytes::from("Test message");
        let message = Message {
            topic: "test/topic".to_string(),
            payload: payload.clone(),
            qos: 1,
            retain: false,
            dup: false,
            packet_id: Some(123),
        };

        assert_eq!(message.topic, "test/topic");
        assert_eq!(message.payload, payload);
        assert_eq!(message.qos, 1);
        assert_eq!(message.retain, false);
        assert_eq!(message.dup, false);
        assert_eq!(message.packet_id, Some(123));
    }

    #[test]
    fn test_message_qos0_no_packet_id() {
        let payload = Bytes::from("QoS 0 message");
        let message = Message {
            topic: "test/qos0".to_string(),
            payload: payload.clone(),
            qos: 0,
            retain: false,
            dup: false,
            packet_id: None,
        };

        assert_eq!(message.qos, 0);
        assert_eq!(message.packet_id, None);
    }

    #[test]
    fn test_topic_filter_creation() {
        let topic_filter = TopicFilter {
            topic: "test/+/wildcard".to_string(),
            qos: 2,
            no_local: true,
            retain_as_published: true,
            retain_handling: 2,
        };

        assert_eq!(topic_filter.topic, "test/+/wildcard");
        assert_eq!(topic_filter.qos, 2);
        assert_eq!(topic_filter.no_local, true);
        assert_eq!(topic_filter.retain_as_published, true);
        assert_eq!(topic_filter.retain_handling, 2);
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
            payload: Bytes::from("Hello"),
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
            topic_filters: vec![TopicFilter {
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
        // Test with empty strings
        let connect_packet = ConnectPacket {
            protocol_name: "".to_string(),
            protocol_version: 0,
            clean_session: false,
            will_flag: false,
            will_qos: 0,
            will_retain: false,
            password_flag: false,
            username_flag: false,
            keep_alive: 0,
            client_id: "".to_string(),
            will_topic: None,
            will_message: None,
            username: None,
            password: None,
            properties: None,
        };

        assert_eq!(connect_packet.protocol_name, "");
        assert_eq!(connect_packet.client_id, "");
        assert_eq!(connect_packet.keep_alive, 0);

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

    #[test]
    fn test_properties_optional_fields() {
        // Test properties with all None values
        let properties = ConnectProperties {
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

        assert_eq!(properties.session_expiry_interval, None);
        assert_eq!(properties.receive_maximum, None);
        assert_eq!(properties.user_properties.len(), 0);
        assert_eq!(properties.authentication_method, None);
    }
} 