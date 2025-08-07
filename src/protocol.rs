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