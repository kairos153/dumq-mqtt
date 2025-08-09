use crate::types::{TopicFilter};
use std::time::Duration;

/// Quality of Service levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

impl QoS {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(QoS::AtMostOnce),
            1 => Some(QoS::AtLeastOnce),
            2 => Some(QoS::ExactlyOnce),
            _ => None,
        }
    }
}

/// Retain flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetainFlag {
    NotRetained = 0,
    Retained = 1,
}

/// Duplicate flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DupFlag {
    NotDuplicate = 0,
    Duplicate = 1,
}

/// Connect options for client
#[derive(Debug, Clone)]
pub struct ConnectOptions {
    pub client_id: String,
    pub clean_session: bool,
    pub keep_alive: Duration,
    pub username: Option<String>,
    pub password: Option<String>,
    pub will_topic: Option<String>,
    pub will_message: Option<Vec<u8>>,
    pub will_qos: QoS,
    pub will_retain: bool,
    pub protocol_version: u8,
}

impl ConnectOptions {
    pub fn new(client_id: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            clean_session: true,
            keep_alive: Duration::from_secs(60),
            username: None,
            password: None,
            will_topic: None,
            will_message: None,
            will_qos: QoS::AtMostOnce,
            will_retain: false,
            protocol_version: 4, // MQTT 3.1.1
        }
    }

    pub fn clean_session(mut self, clean: bool) -> Self {
        self.clean_session = clean;
        self
    }

    pub fn keep_alive(mut self, duration: Duration) -> Self {
        self.keep_alive = duration;
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn will(mut self, topic: impl Into<String>, message: impl Into<Vec<u8>>, qos: QoS, retain: bool) -> Self {
        self.will_topic = Some(topic.into());
        self.will_message = Some(message.into());
        self.will_qos = qos;
        self.will_retain = retain;
        self
    }

    pub fn protocol_version(mut self, version: u8) -> Self {
        self.protocol_version = version;
        self
    }
}

/// Subscribe options
#[derive(Debug, Clone)]
pub struct SubscribeOptions {
    pub topic_filters: Vec<TopicFilter>,
    pub packet_id: u16,
}

impl SubscribeOptions {
    pub fn new(topic_filters: Vec<TopicFilter>, packet_id: u16) -> Self {
        Self {
            topic_filters,
            packet_id,
        }
    }
}

/// Publish options
#[derive(Debug, Clone)]
pub struct PublishOptions {
    pub topic: String,
    pub payload: Vec<u8>,
    pub qos: QoS,
    pub retain: bool,
    pub dup: bool,
    pub packet_id: Option<u16>,
}

impl PublishOptions {
    pub fn new(topic: impl Into<String>, payload: impl Into<Vec<u8>>) -> Self {
        Self {
            topic: topic.into(),
            payload: payload.into(),
            qos: QoS::AtMostOnce,
            retain: false,
            dup: false,
            packet_id: None,
        }
    }

    pub fn qos(mut self, qos: QoS) -> Self {
        self.qos = qos;
        self
    }

    pub fn retain(mut self, retain: bool) -> Self {
        self.retain = retain;
        self
    }

    pub fn dup(mut self, dup: bool) -> Self {
        self.dup = dup;
        self
    }

    pub fn packet_id(mut self, packet_id: u16) -> Self {
        self.packet_id = Some(packet_id);
        self
    }
}

/// MQTT protocol constants
pub const MQTT_PROTOCOL_NAME_V3_1: &str = "MQIsdp";
pub const MQTT_PROTOCOL_NAME_V3_1_1: &str = "MQTT";
pub const MQTT_PROTOCOL_NAME_V5_0: &str = "MQTT";

pub const MQTT_PROTOCOL_VERSION_V3_1: u8 = 3;
pub const MQTT_PROTOCOL_VERSION_V3_1_1: u8 = 4;
pub const MQTT_PROTOCOL_VERSION_V5_0: u8 = 5;

/// Maximum packet size
pub const MAX_PACKET_SIZE: usize = 268_435_455; // 256MB

/// Maximum topic length
pub const MAX_TOPIC_LENGTH: usize = 65535;

/// Maximum client ID length
pub const MAX_CLIENT_ID_LENGTH: usize = 23;

/// Default keep alive interval (60 seconds)
pub const DEFAULT_KEEP_ALIVE: u16 = 60;

/// Maximum keep alive interval (18 hours)
pub const MAX_KEEP_ALIVE: u16 = 65535;

/// Default connection timeout
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

/// Default read timeout
pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(30);

/// Default write timeout
pub const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(30);

/// MQTT 5.0 Reason Codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasonCode {
    Success = 0,
    GrantedQoS1 = 1,
    GrantedQoS2 = 2,
    DisconnectWithWillMessage = 4,
    NoMatchingSubscribers = 16,
    NoSubscriptionExisted = 17,
    ContinueAuthentication = 24,
    ReAuthenticate = 25,
    UnspecifiedError = 128,
    MalformedPacket = 129,
    ProtocolError = 130,
    ImplementationSpecificError = 131,
    UnsupportedProtocolVersion = 132,
    ClientIdentifierNotValid = 133,
    BadUserNameOrPassword = 134,
    NotAuthorized = 135,
    ServerUnavailable = 136,
    ServerBusy = 137,
    Banned = 138,
    ServerShuttingDown = 139,
    BadAuthenticationMethod = 140,
    KeepAliveTimeout = 141,
    SessionTakenOver = 142,
    TopicFilterInvalid = 143,
    TopicNameInvalid = 144,
    PacketIdentifierInUse = 145,
    PacketIdentifierNotFound = 146,
    ReceiveMaximumExceeded = 147,
    TopicAliasInvalid = 148,
    PacketTooLarge = 149,
    MessageRateTooHigh = 150,
    QuotaExceeded = 151,
    AdministrativeAction = 152,
    PayloadFormatInvalid = 153,
    RetainNotSupported = 154,
    QoSNotSupported = 155,
    UseAnotherServer = 156,
    ServerMoved = 157,
    SharedSubscriptionsNotSupported = 158,
    ConnectionRateExceeded = 159,
    MaximumConnectTime = 160,
    SubscriptionIdentifiersNotSupported = 161,
    WildcardSubscriptionsNotSupported = 162,
}

impl ReasonCode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ReasonCode::Success),
            1 => Some(ReasonCode::GrantedQoS1),
            2 => Some(ReasonCode::GrantedQoS2),
            4 => Some(ReasonCode::DisconnectWithWillMessage),
            16 => Some(ReasonCode::NoMatchingSubscribers),
            17 => Some(ReasonCode::NoSubscriptionExisted),
            24 => Some(ReasonCode::ContinueAuthentication),
            25 => Some(ReasonCode::ReAuthenticate),
            128 => Some(ReasonCode::UnspecifiedError),
            129 => Some(ReasonCode::MalformedPacket),
            130 => Some(ReasonCode::ProtocolError),
            131 => Some(ReasonCode::ImplementationSpecificError),
            132 => Some(ReasonCode::UnsupportedProtocolVersion),
            133 => Some(ReasonCode::ClientIdentifierNotValid),
            134 => Some(ReasonCode::BadUserNameOrPassword),
            135 => Some(ReasonCode::NotAuthorized),
            136 => Some(ReasonCode::ServerUnavailable),
            137 => Some(ReasonCode::ServerBusy),
            138 => Some(ReasonCode::Banned),
            139 => Some(ReasonCode::ServerShuttingDown),
            140 => Some(ReasonCode::BadAuthenticationMethod),
            141 => Some(ReasonCode::KeepAliveTimeout),
            142 => Some(ReasonCode::SessionTakenOver),
            143 => Some(ReasonCode::TopicFilterInvalid),
            144 => Some(ReasonCode::TopicNameInvalid),
            145 => Some(ReasonCode::PacketIdentifierInUse),
            146 => Some(ReasonCode::PacketIdentifierNotFound),
            147 => Some(ReasonCode::ReceiveMaximumExceeded),
            148 => Some(ReasonCode::TopicAliasInvalid),
            149 => Some(ReasonCode::PacketTooLarge),
            150 => Some(ReasonCode::MessageRateTooHigh),
            151 => Some(ReasonCode::QuotaExceeded),
            152 => Some(ReasonCode::AdministrativeAction),
            153 => Some(ReasonCode::PayloadFormatInvalid),
            154 => Some(ReasonCode::RetainNotSupported),
            155 => Some(ReasonCode::QoSNotSupported),
            156 => Some(ReasonCode::UseAnotherServer),
            157 => Some(ReasonCode::ServerMoved),
            158 => Some(ReasonCode::SharedSubscriptionsNotSupported),
            159 => Some(ReasonCode::ConnectionRateExceeded),
            160 => Some(ReasonCode::MaximumConnectTime),
            161 => Some(ReasonCode::SubscriptionIdentifiersNotSupported),
            162 => Some(ReasonCode::WildcardSubscriptionsNotSupported),
            _ => None,
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TopicFilter;

    #[test]
    fn test_qos_from_u8() {
        assert_eq!(QoS::from_u8(0), Some(QoS::AtMostOnce));
        assert_eq!(QoS::from_u8(1), Some(QoS::AtLeastOnce));
        assert_eq!(QoS::from_u8(2), Some(QoS::ExactlyOnce));
        assert_eq!(QoS::from_u8(3), None);
        assert_eq!(QoS::from_u8(255), None);
    }

    #[test]
    fn test_qos_values() {
        assert_eq!(QoS::AtMostOnce as u8, 0);
        assert_eq!(QoS::AtLeastOnce as u8, 1);
        assert_eq!(QoS::ExactlyOnce as u8, 2);
    }

    #[test]
    fn test_qos_clone() {
        let qos = QoS::AtLeastOnce;
        let cloned_qos = qos;
        assert_eq!(qos, cloned_qos);
    }

    #[test]
    fn test_retain_flag() {
        assert_eq!(RetainFlag::NotRetained as u8, 0);
        assert_eq!(RetainFlag::Retained as u8, 1);
    }

    #[test]
    fn test_dup_flag() {
        assert_eq!(DupFlag::NotDuplicate as u8, 0);
        assert_eq!(DupFlag::Duplicate as u8, 1);
    }

    #[test]
    fn test_connect_options_new() {
        let options = ConnectOptions::new("test_client");
        
        assert_eq!(options.client_id, "test_client");
        assert_eq!(options.clean_session, true);
        assert_eq!(options.keep_alive, Duration::from_secs(60));
        assert_eq!(options.username, None);
        assert_eq!(options.password, None);
        assert_eq!(options.will_topic, None);
        assert_eq!(options.will_message, None);
        assert_eq!(options.will_qos, QoS::AtMostOnce);
        assert_eq!(options.will_retain, false);
        assert_eq!(options.protocol_version, 4);
    }

    #[test]
    fn test_connect_options_builder_pattern() {
        let options = ConnectOptions::new("test_client")
            .clean_session(false)
            .keep_alive(Duration::from_secs(120))
            .username("test_user")
            .password("test_pass")
            .will("test/will", b"will message", QoS::AtLeastOnce, true)
            .protocol_version(5);

        assert_eq!(options.client_id, "test_client");
        assert_eq!(options.clean_session, false);
        assert_eq!(options.keep_alive, Duration::from_secs(120));
        assert_eq!(options.username, Some("test_user".to_string()));
        assert_eq!(options.password, Some("test_pass".to_string()));
        assert_eq!(options.will_topic, Some("test/will".to_string()));
        assert_eq!(options.will_message, Some(b"will message".to_vec()));
        assert_eq!(options.will_qos, QoS::AtLeastOnce);
        assert_eq!(options.will_retain, true);
        assert_eq!(options.protocol_version, 5);
    }

    #[test]
    fn test_connect_options_clone() {
        let options = ConnectOptions::new("test_client")
            .username("test_user")
            .password("test_pass");
        
        let cloned_options = options.clone();
        assert_eq!(options.client_id, cloned_options.client_id);
        assert_eq!(options.username, cloned_options.username);
        assert_eq!(options.password, cloned_options.password);
    }

    #[test]
    fn test_connect_options_with_empty_strings() {
        let options = ConnectOptions::new("")
            .username("")
            .password("");

        assert_eq!(options.client_id, "");
        assert_eq!(options.username, Some("".to_string()));
        assert_eq!(options.password, Some("".to_string()));
    }

    #[test]
    fn test_connect_options_with_unicode() {
        let options = ConnectOptions::new("한국어_클라이언트")
            .username("한국어_사용자")
            .password("한국어_비밀번호");

        assert_eq!(options.client_id, "한국어_클라이언트");
        assert_eq!(options.username, Some("한국어_사용자".to_string()));
        assert_eq!(options.password, Some("한국어_비밀번호".to_string()));
    }

    #[test]
    fn test_subscribe_options() {
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

        let options = SubscribeOptions::new(topic_filters.clone(), 123);

        assert_eq!(options.topic_filters.len(), 2);
        assert_eq!(options.topic_filters[0].topic, "test/topic1");
        assert_eq!(options.topic_filters[0].qos, 0);
        assert_eq!(options.topic_filters[1].topic, "test/topic2");
        assert_eq!(options.topic_filters[1].qos, 1);
        assert_eq!(options.packet_id, 123);
    }

    #[test]
    fn test_subscribe_options_clone() {
        let topic_filters = vec![TopicFilter {
            topic: "test/topic".to_string(),
            qos: 1,
            no_local: false,
            retain_as_published: false,
            retain_handling: 0,
        }];

        let options = SubscribeOptions::new(topic_filters, 456);
        let cloned_options = options.clone();

        assert_eq!(options.packet_id, cloned_options.packet_id);
        assert_eq!(options.topic_filters.len(), cloned_options.topic_filters.len());
    }

    #[test]
    fn test_publish_options_new() {
        let options = PublishOptions::new("test/topic", b"Hello MQTT!");

        assert_eq!(options.topic, "test/topic");
        assert_eq!(options.payload, b"Hello MQTT!");
        assert_eq!(options.qos, QoS::AtMostOnce);
        assert_eq!(options.retain, false);
        assert_eq!(options.dup, false);
        assert_eq!(options.packet_id, None);
    }

    #[test]
    fn test_publish_options_builder_pattern() {
        let options = PublishOptions::new("test/topic", b"Hello MQTT!")
            .qos(QoS::AtLeastOnce)
            .retain(true)
            .dup(true)
            .packet_id(789);

        assert_eq!(options.topic, "test/topic");
        assert_eq!(options.payload, b"Hello MQTT!");
        assert_eq!(options.qos, QoS::AtLeastOnce);
        assert_eq!(options.retain, true);
        assert_eq!(options.dup, true);
        assert_eq!(options.packet_id, Some(789));
    }

    #[test]
    fn test_publish_options_clone() {
        let options = PublishOptions::new("test/topic", b"Hello MQTT!")
            .qos(QoS::ExactlyOnce)
            .retain(true);

        let cloned_options = options.clone();
        assert_eq!(options.topic, cloned_options.topic);
        assert_eq!(options.payload, cloned_options.payload);
        assert_eq!(options.qos, cloned_options.qos);
        assert_eq!(options.retain, cloned_options.retain);
    }

    #[test]
    fn test_publish_options_with_empty_payload() {
        let options = PublishOptions::new("test/topic", b"");

        assert_eq!(options.topic, "test/topic");
        assert_eq!(options.payload, b"");
        assert_eq!(options.qos, QoS::AtMostOnce);
    }

    #[test]
    fn test_protocol_constants() {
        assert_eq!(MQTT_PROTOCOL_NAME_V3_1, "MQIsdp");
        assert_eq!(MQTT_PROTOCOL_NAME_V3_1_1, "MQTT");
        assert_eq!(MQTT_PROTOCOL_NAME_V5_0, "MQTT");
        
        assert_eq!(MQTT_PROTOCOL_VERSION_V3_1, 3);
        assert_eq!(MQTT_PROTOCOL_VERSION_V3_1_1, 4);
        assert_eq!(MQTT_PROTOCOL_VERSION_V5_0, 5);
        
        assert_eq!(MAX_PACKET_SIZE, 268_435_455);
        assert_eq!(MAX_TOPIC_LENGTH, 65535);
        assert_eq!(MAX_CLIENT_ID_LENGTH, 23);
        assert_eq!(DEFAULT_KEEP_ALIVE, 60);
        assert_eq!(MAX_KEEP_ALIVE, 65535);
        
        assert_eq!(DEFAULT_CONNECT_TIMEOUT, Duration::from_secs(30));
        assert_eq!(DEFAULT_READ_TIMEOUT, Duration::from_secs(30));
        assert_eq!(DEFAULT_WRITE_TIMEOUT, Duration::from_secs(30));
    }

    #[test]
    fn test_reason_code_from_u8() {
        assert_eq!(ReasonCode::from_u8(0), Some(ReasonCode::Success));
        assert_eq!(ReasonCode::from_u8(1), Some(ReasonCode::GrantedQoS1));
        assert_eq!(ReasonCode::from_u8(2), Some(ReasonCode::GrantedQoS2));
        assert_eq!(ReasonCode::from_u8(4), Some(ReasonCode::DisconnectWithWillMessage));
        assert_eq!(ReasonCode::from_u8(16), Some(ReasonCode::NoMatchingSubscribers));
        assert_eq!(ReasonCode::from_u8(17), Some(ReasonCode::NoSubscriptionExisted));
        assert_eq!(ReasonCode::from_u8(24), Some(ReasonCode::ContinueAuthentication));
        assert_eq!(ReasonCode::from_u8(25), Some(ReasonCode::ReAuthenticate));
        assert_eq!(ReasonCode::from_u8(128), Some(ReasonCode::UnspecifiedError));
        assert_eq!(ReasonCode::from_u8(129), Some(ReasonCode::MalformedPacket));
        assert_eq!(ReasonCode::from_u8(130), Some(ReasonCode::ProtocolError));
        assert_eq!(ReasonCode::from_u8(131), Some(ReasonCode::ImplementationSpecificError));
        assert_eq!(ReasonCode::from_u8(132), Some(ReasonCode::UnsupportedProtocolVersion));
        assert_eq!(ReasonCode::from_u8(133), Some(ReasonCode::ClientIdentifierNotValid));
        assert_eq!(ReasonCode::from_u8(134), Some(ReasonCode::BadUserNameOrPassword));
        assert_eq!(ReasonCode::from_u8(135), Some(ReasonCode::NotAuthorized));
        assert_eq!(ReasonCode::from_u8(136), Some(ReasonCode::ServerUnavailable));
        assert_eq!(ReasonCode::from_u8(137), Some(ReasonCode::ServerBusy));
        assert_eq!(ReasonCode::from_u8(138), Some(ReasonCode::Banned));
        assert_eq!(ReasonCode::from_u8(139), Some(ReasonCode::ServerShuttingDown));
        assert_eq!(ReasonCode::from_u8(140), Some(ReasonCode::BadAuthenticationMethod));
        assert_eq!(ReasonCode::from_u8(141), Some(ReasonCode::KeepAliveTimeout));
        assert_eq!(ReasonCode::from_u8(142), Some(ReasonCode::SessionTakenOver));
        assert_eq!(ReasonCode::from_u8(143), Some(ReasonCode::TopicFilterInvalid));
        assert_eq!(ReasonCode::from_u8(144), Some(ReasonCode::TopicNameInvalid));
        assert_eq!(ReasonCode::from_u8(145), Some(ReasonCode::PacketIdentifierInUse));
        assert_eq!(ReasonCode::from_u8(146), Some(ReasonCode::PacketIdentifierNotFound));
        assert_eq!(ReasonCode::from_u8(147), Some(ReasonCode::ReceiveMaximumExceeded));
        assert_eq!(ReasonCode::from_u8(148), Some(ReasonCode::TopicAliasInvalid));
        assert_eq!(ReasonCode::from_u8(149), Some(ReasonCode::PacketTooLarge));
        assert_eq!(ReasonCode::from_u8(150), Some(ReasonCode::MessageRateTooHigh));
        assert_eq!(ReasonCode::from_u8(151), Some(ReasonCode::QuotaExceeded));
        assert_eq!(ReasonCode::from_u8(152), Some(ReasonCode::AdministrativeAction));
        assert_eq!(ReasonCode::from_u8(153), Some(ReasonCode::PayloadFormatInvalid));
        assert_eq!(ReasonCode::from_u8(154), Some(ReasonCode::RetainNotSupported));
        assert_eq!(ReasonCode::from_u8(155), Some(ReasonCode::QoSNotSupported));
        assert_eq!(ReasonCode::from_u8(156), Some(ReasonCode::UseAnotherServer));
        assert_eq!(ReasonCode::from_u8(157), Some(ReasonCode::ServerMoved));
        assert_eq!(ReasonCode::from_u8(158), Some(ReasonCode::SharedSubscriptionsNotSupported));
        assert_eq!(ReasonCode::from_u8(159), Some(ReasonCode::ConnectionRateExceeded));
        assert_eq!(ReasonCode::from_u8(160), Some(ReasonCode::MaximumConnectTime));
        assert_eq!(ReasonCode::from_u8(161), Some(ReasonCode::SubscriptionIdentifiersNotSupported));
        assert_eq!(ReasonCode::from_u8(162), Some(ReasonCode::WildcardSubscriptionsNotSupported));
        
        // Test invalid values
        assert_eq!(ReasonCode::from_u8(3), None);
        assert_eq!(ReasonCode::from_u8(15), None);
        assert_eq!(ReasonCode::from_u8(23), None);
        assert_eq!(ReasonCode::from_u8(26), None);
        assert_eq!(ReasonCode::from_u8(127), None);
        assert_eq!(ReasonCode::from_u8(163), None);
        assert_eq!(ReasonCode::from_u8(255), None);
    }

    #[test]
    fn test_reason_code_values() {
        assert_eq!(ReasonCode::Success as u8, 0);
        assert_eq!(ReasonCode::GrantedQoS1 as u8, 1);
        assert_eq!(ReasonCode::GrantedQoS2 as u8, 2);
        assert_eq!(ReasonCode::DisconnectWithWillMessage as u8, 4);
        assert_eq!(ReasonCode::NoMatchingSubscribers as u8, 16);
        assert_eq!(ReasonCode::NoSubscriptionExisted as u8, 17);
        assert_eq!(ReasonCode::ContinueAuthentication as u8, 24);
        assert_eq!(ReasonCode::ReAuthenticate as u8, 25);
        assert_eq!(ReasonCode::UnspecifiedError as u8, 128);
        assert_eq!(ReasonCode::MalformedPacket as u8, 129);
        assert_eq!(ReasonCode::ProtocolError as u8, 130);
        assert_eq!(ReasonCode::ImplementationSpecificError as u8, 131);
        assert_eq!(ReasonCode::UnsupportedProtocolVersion as u8, 132);
        assert_eq!(ReasonCode::ClientIdentifierNotValid as u8, 133);
        assert_eq!(ReasonCode::BadUserNameOrPassword as u8, 134);
        assert_eq!(ReasonCode::NotAuthorized as u8, 135);
        assert_eq!(ReasonCode::ServerUnavailable as u8, 136);
        assert_eq!(ReasonCode::ServerBusy as u8, 137);
        assert_eq!(ReasonCode::Banned as u8, 138);
        assert_eq!(ReasonCode::ServerShuttingDown as u8, 139);
        assert_eq!(ReasonCode::BadAuthenticationMethod as u8, 140);
        assert_eq!(ReasonCode::KeepAliveTimeout as u8, 141);
        assert_eq!(ReasonCode::SessionTakenOver as u8, 142);
        assert_eq!(ReasonCode::TopicFilterInvalid as u8, 143);
        assert_eq!(ReasonCode::TopicNameInvalid as u8, 144);
        assert_eq!(ReasonCode::PacketIdentifierInUse as u8, 145);
        assert_eq!(ReasonCode::PacketIdentifierNotFound as u8, 146);
        assert_eq!(ReasonCode::ReceiveMaximumExceeded as u8, 147);
        assert_eq!(ReasonCode::TopicAliasInvalid as u8, 148);
        assert_eq!(ReasonCode::PacketTooLarge as u8, 149);
        assert_eq!(ReasonCode::MessageRateTooHigh as u8, 150);
        assert_eq!(ReasonCode::QuotaExceeded as u8, 151);
        assert_eq!(ReasonCode::AdministrativeAction as u8, 152);
        assert_eq!(ReasonCode::PayloadFormatInvalid as u8, 153);
        assert_eq!(ReasonCode::RetainNotSupported as u8, 154);
        assert_eq!(ReasonCode::QoSNotSupported as u8, 155);
        assert_eq!(ReasonCode::UseAnotherServer as u8, 156);
        assert_eq!(ReasonCode::ServerMoved as u8, 157);
        assert_eq!(ReasonCode::SharedSubscriptionsNotSupported as u8, 158);
        assert_eq!(ReasonCode::ConnectionRateExceeded as u8, 159);
        assert_eq!(ReasonCode::MaximumConnectTime as u8, 160);
        assert_eq!(ReasonCode::SubscriptionIdentifiersNotSupported as u8, 161);
        assert_eq!(ReasonCode::WildcardSubscriptionsNotSupported as u8, 162);
    }

    #[test]
    fn test_reason_code_clone() {
        let reason_code = ReasonCode::Success;
        let cloned_reason_code = reason_code;
        assert_eq!(reason_code, cloned_reason_code);
    }

    #[test]
    fn test_edge_cases() {
        // Test with maximum values
        let options = ConnectOptions::new("test")
            .keep_alive(Duration::from_secs(u64::MAX))
            .protocol_version(u8::MAX);

        assert_eq!(options.keep_alive, Duration::from_secs(u64::MAX));
        assert_eq!(options.protocol_version, u8::MAX);

        // Test with minimum values
        let options = ConnectOptions::new("test")
            .keep_alive(Duration::from_secs(0))
            .protocol_version(0);

        assert_eq!(options.keep_alive, Duration::from_secs(0));
        assert_eq!(options.protocol_version, 0);
    }

    #[test]
    fn test_duration_conversions() {
        let options = ConnectOptions::new("test")
            .keep_alive(Duration::from_millis(50000)); // 50 seconds

        assert_eq!(options.keep_alive, Duration::from_secs(50));

        let options = ConnectOptions::new("test")
            .keep_alive(Duration::from_micros(60_000_000)); // 60 seconds

        assert_eq!(options.keep_alive, Duration::from_secs(60));
    }
} 