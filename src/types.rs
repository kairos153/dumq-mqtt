use bytes::{Bytes};
use std::collections::HashMap;

// MQTT v5 Property Identifiers
pub const PROPERTY_PAYLOAD_FORMAT_INDICATOR: u8 = 0x01;
pub const PROPERTY_MESSAGE_EXPIRY_INTERVAL: u8 = 0x02;
pub const PROPERTY_CONTENT_TYPE: u8 = 0x03;
pub const PROPERTY_RESPONSE_TOPIC: u8 = 0x08;
pub const PROPERTY_CORRELATION_DATA: u8 = 0x09;
pub const PROPERTY_SUBSCRIPTION_IDENTIFIER: u8 = 0x0B;
pub const PROPERTY_SESSION_EXPIRY_INTERVAL: u8 = 0x11;
pub const PROPERTY_ASSIGNED_CLIENT_IDENTIFIER: u8 = 0x12;
pub const PROPERTY_SERVER_KEEP_ALIVE: u8 = 0x13;
pub const PROPERTY_AUTHENTICATION_METHOD: u8 = 0x15;
pub const PROPERTY_AUTHENTICATION_DATA: u8 = 0x16;
pub const PROPERTY_REQUEST_PROBLEM_INFORMATION: u8 = 0x17;
pub const PROPERTY_WILL_DELAY_INTERVAL: u8 = 0x18;
pub const PROPERTY_REQUEST_RESPONSE_INFORMATION: u8 = 0x19;
pub const PROPERTY_RESPONSE_INFORMATION: u8 = 0x1A;
pub const PROPERTY_SERVER_REFERENCE: u8 = 0x1C;
pub const PROPERTY_REASON_STRING: u8 = 0x1F;
pub const PROPERTY_RECEIVE_MAXIMUM: u8 = 0x21;
pub const PROPERTY_TOPIC_ALIAS_MAXIMUM: u8 = 0x22;
pub const PROPERTY_TOPIC_ALIAS: u8 = 0x23;
pub const PROPERTY_MAXIMUM_QOS: u8 = 0x24;
pub const PROPERTY_RETAIN_AVAILABLE: u8 = 0x25;
pub const PROPERTY_USER_PROPERTY: u8 = 0x26;
pub const PROPERTY_MAXIMUM_PACKET_SIZE: u8 = 0x27;
pub const PROPERTY_WILDCARD_SUBSCRIPTION_AVAILABLE: u8 = 0x28;
pub const PROPERTY_SUBSCRIPTION_IDENTIFIERS_AVAILABLE: u8 = 0x29;
pub const PROPERTY_SHARED_SUBSCRIPTION_AVAILABLE: u8 = 0x2A;

// MQTT v5 Payload Format Indicator values
pub const PAYLOAD_FORMAT_INDICATOR_UNSPECIFIED: u8 = 0x00;
pub const PAYLOAD_FORMAT_INDICATOR_UTF8: u8 = 0x01;

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

impl ConnAckPacket {
    /// Create a new ConnAckPacket with default values
    pub fn new(return_code: ConnectReturnCode) -> Self {
        Self {
            session_present: false,
            return_code,
            properties: None,
        }
    }

    /// Create a new ConnAckPacket with session present
    pub fn with_session(return_code: ConnectReturnCode, session_present: bool) -> Self {
        Self {
            session_present,
            return_code,
            properties: None,
        }
    }

    /// Set session present
    pub fn session_present(mut self, present: bool) -> Self {
        self.session_present = present;
        self
    }

    /// Set return code
    pub fn return_code(mut self, code: ConnectReturnCode) -> Self {
        self.return_code = code;
        self
    }

    /// Set properties
    pub fn properties(mut self, properties: ConnAckProperties) -> Self {
        self.properties = Some(properties);
        self
    }

    /// Check if this is a successful connection
    pub fn is_success(&self) -> bool {
        matches!(self.return_code, ConnectReturnCode::Accepted)
    }

    /// Check if this is an error connection
    pub fn is_error(&self) -> bool {
        !self.is_success()
    }

    /// Get the error message if this is an error connection
    pub fn error_message(&self) -> Option<&'static str> {
        match self.return_code {
            ConnectReturnCode::Accepted => None,
            ConnectReturnCode::UnacceptableProtocolVersion => Some("Unacceptable protocol version"),
            ConnectReturnCode::IdentifierRejected => Some("Identifier rejected"),
            ConnectReturnCode::ServerUnavailable => Some("Server unavailable"),
            ConnectReturnCode::BadUsernameOrPassword => Some("Bad username or password"),
            ConnectReturnCode::NotAuthorized => Some("Not authorized"),
            ConnectReturnCode::MalformedPacket => Some("Malformed packet"),
            ConnectReturnCode::ProtocolError => Some("Protocol error"),
            ConnectReturnCode::ImplementationSpecificError => Some("Implementation specific error"),
            ConnectReturnCode::UnsupportedProtocolVersion => Some("Unsupported protocol version"),
            ConnectReturnCode::ClientIdentifierNotValid => Some("Client identifier not valid"),
            ConnectReturnCode::BadUsernameOrPasswordV5 => Some("Bad username or password (MQTT 5.0)"),
            ConnectReturnCode::NotAuthorizedV5 => Some("Not authorized (MQTT 5.0)"),
            ConnectReturnCode::ServerUnavailableV5 => Some("Server unavailable (MQTT 5.0)"),
            ConnectReturnCode::ServerBusy => Some("Server busy"),
            ConnectReturnCode::Banned => Some("Banned"),
            ConnectReturnCode::BadAuthenticationMethod => Some("Bad authentication method"),
            ConnectReturnCode::TopicNameInvalid => Some("Topic name invalid"),
            ConnectReturnCode::PacketTooLarge => Some("Packet too large"),
            ConnectReturnCode::QuotaExceeded => Some("Quota exceeded"),
            ConnectReturnCode::PayloadFormatInvalid => Some("Payload format invalid"),
            ConnectReturnCode::RetainNotSupported => Some("Retain not supported"),
            ConnectReturnCode::QoSNotSupported => Some("QoS not supported"),
            ConnectReturnCode::UseAnotherServer => Some("Use another server"),
            ConnectReturnCode::ServerMoved => Some("Server moved"),
            ConnectReturnCode::ConnectionRateExceeded => Some("Connection rate exceeded"),
        }
    }
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
    // MQTT 5.0 additional return codes
    MalformedPacket = 128,
    ProtocolError = 130,
    ImplementationSpecificError = 131,
    UnsupportedProtocolVersion = 132,
    ClientIdentifierNotValid = 133,
    BadUsernameOrPasswordV5 = 134,
    NotAuthorizedV5 = 135,
    ServerUnavailableV5 = 136,
    ServerBusy = 137,
    Banned = 138,
    BadAuthenticationMethod = 140,
    TopicNameInvalid = 144,
    PacketTooLarge = 149,
    QuotaExceeded = 151,
    PayloadFormatInvalid = 153,
    RetainNotSupported = 154,
    QoSNotSupported = 155,
    UseAnotherServer = 156,
    ServerMoved = 157,
    ConnectionRateExceeded = 159,
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
            128 => Some(ConnectReturnCode::MalformedPacket),
            130 => Some(ConnectReturnCode::ProtocolError),
            131 => Some(ConnectReturnCode::ImplementationSpecificError),
            132 => Some(ConnectReturnCode::UnsupportedProtocolVersion),
            133 => Some(ConnectReturnCode::ClientIdentifierNotValid),
            134 => Some(ConnectReturnCode::BadUsernameOrPasswordV5),
            135 => Some(ConnectReturnCode::NotAuthorizedV5),
            136 => Some(ConnectReturnCode::ServerUnavailableV5),
            137 => Some(ConnectReturnCode::ServerBusy),
            138 => Some(ConnectReturnCode::Banned),
            140 => Some(ConnectReturnCode::BadAuthenticationMethod),
            144 => Some(ConnectReturnCode::TopicNameInvalid),
            149 => Some(ConnectReturnCode::PacketTooLarge),
            151 => Some(ConnectReturnCode::QuotaExceeded),
            153 => Some(ConnectReturnCode::PayloadFormatInvalid),
            154 => Some(ConnectReturnCode::RetainNotSupported),
            155 => Some(ConnectReturnCode::QoSNotSupported),
            156 => Some(ConnectReturnCode::UseAnotherServer),
            157 => Some(ConnectReturnCode::ServerMoved),
            159 => Some(ConnectReturnCode::ConnectionRateExceeded),
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

impl PublishPacket {
    /// Create a new PublishPacket
    pub fn new(topic_name: String, payload: Bytes) -> Self {
        Self {
            topic_name,
            packet_id: None,
            payload,
            properties: None,
        }
    }

    /// Create a new PublishPacket with QoS and packet ID
    pub fn with_qos(topic_name: String, payload: Bytes, qos: u8, packet_id: u16) -> Self {
        Self {
            topic_name,
            packet_id: if qos > 0 { Some(packet_id) } else { None },
            payload,
            properties: None,
        }
    }

    /// Set the packet ID
    pub fn packet_id(mut self, id: u16) -> Self {
        self.packet_id = Some(id);
        self
    }

    /// Set the properties
    pub fn properties(mut self, properties: PublishProperties) -> Self {
        self.properties = Some(properties);
        self
    }

    /// Set payload format indicator
    pub fn payload_format_indicator(mut self, indicator: u8) -> Self {
        if self.properties.is_none() {
            self.properties = Some(PublishProperties::new());
        }
        if let Some(props) = &mut self.properties {
            *props = props.clone().payload_format_indicator(indicator);
        }
        self
    }

    /// Set message expiry interval
    pub fn message_expiry_interval(mut self, interval: u32) -> Self {
        if self.properties.is_none() {
            self.properties = Some(PublishProperties::new());
        }
        if let Some(props) = &mut self.properties {
            *props = props.clone().message_expiry_interval(interval);
        }
        self
    }

    /// Set topic alias
    pub fn topic_alias(mut self, alias: u16) -> Self {
        if self.properties.is_none() {
            self.properties = Some(PublishProperties::new());
        }
        if let Some(props) = &mut self.properties {
            *props = props.clone().topic_alias(alias);
        }
        self
    }

    /// Set response topic
    pub fn response_topic(mut self, topic: String) -> Self {
        if self.properties.is_none() {
            self.properties = Some(PublishProperties::new());
        }
        if let Some(props) = &mut self.properties {
            *props = props.clone().response_topic(topic);
        }
        self
    }

    /// Set correlation data
    pub fn correlation_data(mut self, data: Bytes) -> Self {
        if self.properties.is_none() {
            self.properties = Some(PublishProperties::new());
        }
        if let Some(props) = &mut self.properties {
            *props = props.clone().correlation_data(data);
        }
        self
    }

    /// Set subscription identifier
    pub fn subscription_identifier(mut self, id: u32) -> Self {
        if self.properties.is_none() {
            self.properties = Some(PublishProperties::new());
        }
        if let Some(props) = &mut self.properties {
            *props = props.clone().subscription_identifier(id);
        }
        self
    }

    /// Set content type
    pub fn content_type(mut self, content_type: String) -> Self {
        if self.properties.is_none() {
            self.properties = Some(PublishProperties::new());
        }
        if let Some(props) = &mut self.properties {
            *props = props.clone().content_type(content_type);
        }
        self
    }

    /// Add user property
    pub fn user_property(mut self, key: String, value: String) -> Self {
        if self.properties.is_none() {
            self.properties = Some(PublishProperties::new());
        }
        if let Some(props) = &mut self.properties {
            *props = props.clone().user_property(key, value);
        }
        self
    }

    /// Check if the packet has any properties
    pub fn has_properties(&self) -> bool {
        self.properties.as_ref().map_or(false, |p| !p.is_empty())
    }

    /// Get QoS level (0 if no packet_id, 1 or 2 if packet_id exists)
    pub fn qos(&self) -> u8 {
        if self.packet_id.is_some() { 1 } else { 0 }
    }
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
/// Connect properties for MQTT 5.0
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

impl Default for ConnectProperties {
    fn default() -> Self {
        Self {
            session_expiry_interval: None,
            receive_maximum: None,
            max_packet_size: None,
            topic_alias_maximum: None,
            request_response_information: None,
            request_problem_information: None,
            user_properties: HashMap::new(),
            authentication_method: None,
            authentication_data: None,
        }
    }
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

impl Default for ConnAckProperties {
    fn default() -> Self {
        Self {
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
        }
    }
}

impl ConnAckProperties {
    /// Create a new ConnAckProperties with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set session expiry interval
    pub fn session_expiry_interval(mut self, interval: u32) -> Self {
        self.session_expiry_interval = Some(interval);
        self
    }

    /// Set receive maximum
    pub fn receive_maximum(mut self, max: u16) -> Self {
        self.receive_maximum = Some(max);
        self
    }

    /// Set maximum QoS
    pub fn max_qos(mut self, qos: u8) -> Self {
        self.max_qos = Some(qos);
        self
    }

    /// Set retain available
    pub fn retain_available(mut self, available: bool) -> Self {
        self.retain_available = Some(available);
        self
    }

    /// Set maximum packet size
    pub fn max_packet_size(mut self, size: u32) -> Self {
        self.max_packet_size = Some(size);
        self
    }

    /// Set assigned client identifier
    pub fn assigned_client_identifier(mut self, id: String) -> Self {
        self.assigned_client_identifier = Some(id);
        self
    }

    /// Set topic alias maximum
    pub fn topic_alias_maximum(mut self, max: u16) -> Self {
        self.topic_alias_maximum = Some(max);
        self
    }

    /// Set reason string
    pub fn reason_string(mut self, reason: String) -> Self {
        self.reason_string = Some(reason);
        self
    }

    /// Add a user property
    pub fn user_property(mut self, key: String, value: String) -> Self {
        self.user_properties.insert(key, value);
        self
    }

    /// Set wildcard subscription available
    pub fn wildcard_subscription_available(mut self, available: bool) -> Self {
        self.wildcard_subscription_available = Some(available);
        self
    }

    /// Set subscription identifiers available
    pub fn subscription_identifiers_available(mut self, available: bool) -> Self {
        self.subscription_identifiers_available = Some(available);
        self
    }

    /// Set shared subscription available
    pub fn shared_subscription_available(mut self, available: bool) -> Self {
        self.shared_subscription_available = Some(available);
        self
    }

    /// Set server keep alive
    pub fn server_keep_alive(mut self, keep_alive: u16) -> Self {
        self.server_keep_alive = Some(keep_alive);
        self
    }

    /// Set response information
    pub fn response_information(mut self, info: String) -> Self {
        self.response_information = Some(info);
        self
    }

    /// Set server reference
    pub fn server_reference(mut self, reference: String) -> Self {
        self.server_reference = Some(reference);
        self
    }

    /// Set authentication method
    pub fn authentication_method(mut self, method: String) -> Self {
        self.authentication_method = Some(method);
        self
    }

    /// Set authentication data
    pub fn authentication_data(mut self, data: Bytes) -> Self {
        self.authentication_data = Some(data);
        self
    }

    /// Check if any properties are set
    pub fn is_empty(&self) -> bool {
        self.session_expiry_interval.is_none() &&
        self.receive_maximum.is_none() &&
        self.max_qos.is_none() &&
        self.retain_available.is_none() &&
        self.max_packet_size.is_none() &&
        self.assigned_client_identifier.is_none() &&
        self.topic_alias_maximum.is_none() &&
        self.reason_string.is_none() &&
        self.user_properties.is_empty() &&
        self.wildcard_subscription_available.is_none() &&
        self.subscription_identifiers_available.is_none() &&
        self.shared_subscription_available.is_none() &&
        self.server_keep_alive.is_none() &&
        self.response_information.is_none() &&
        self.server_reference.is_none() &&
        self.authentication_method.is_none() &&
        self.authentication_data.is_none()
    }
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

impl Default for PublishProperties {
    fn default() -> Self {
        Self {
            payload_format_indicator: None,
            message_expiry_interval: None,
            topic_alias: None,
            response_topic: None,
            correlation_data: None,
            user_properties: HashMap::new(),
            subscription_identifier: None,
            content_type: None,
        }
    }
}

impl PublishProperties {
    /// Create a new empty PublishProperties
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the payload format indicator
    pub fn payload_format_indicator(mut self, indicator: u8) -> Self {
        self.payload_format_indicator = Some(indicator);
        self
    }

    /// Set the message expiry interval in seconds
    pub fn message_expiry_interval(mut self, interval: u32) -> Self {
        self.message_expiry_interval = Some(interval);
        self
    }

    /// Set the topic alias
    pub fn topic_alias(mut self, alias: u16) -> Self {
        self.topic_alias = Some(alias);
        self
    }

    /// Set the response topic
    pub fn response_topic(mut self, topic: String) -> Self {
        self.response_topic = Some(topic);
        self
    }

    /// Set the correlation data
    pub fn correlation_data(mut self, data: Bytes) -> Self {
        self.correlation_data = Some(data);
        self
    }

    /// Set the subscription identifier
    pub fn subscription_identifier(mut self, id: u32) -> Self {
        self.subscription_identifier = Some(id);
        self
    }

    /// Set the content type
    pub fn content_type(mut self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }

    /// Add a user property
    pub fn user_property(mut self, key: String, value: String) -> Self {
        self.user_properties.insert(key, value);
        self
    }

    /// Check if all properties are None/empty
    pub fn is_empty(&self) -> bool {
        self.payload_format_indicator.is_none() &&
        self.message_expiry_interval.is_none() &&
        self.topic_alias.is_none() &&
        self.response_topic.is_none() &&
        self.correlation_data.is_none() &&
        self.user_properties.is_empty() &&
        self.subscription_identifier.is_none() &&
        self.content_type.is_none()
    }

    /// Get the payload format indicator as a string
    pub fn payload_format_indicator_str(&self) -> Option<&'static str> {
        match self.payload_format_indicator {
            Some(PAYLOAD_FORMAT_INDICATOR_UNSPECIFIED) => Some("Unspecified"),
            Some(PAYLOAD_FORMAT_INDICATOR_UTF8) => Some("UTF-8"),
            _ => None,
        }
    }

    /// Check if the payload is UTF-8 encoded
    pub fn is_utf8_payload(&self) -> bool {
        self.payload_format_indicator == Some(PAYLOAD_FORMAT_INDICATOR_UTF8)
    }

    /// Check if the payload format is unspecified
    pub fn is_unspecified_payload(&self) -> bool {
        self.payload_format_indicator == Some(PAYLOAD_FORMAT_INDICATOR_UNSPECIFIED)
    }
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
        assert_eq!(ConnectReturnCode::from_u8(128), Some(ConnectReturnCode::MalformedPacket));
        assert_eq!(ConnectReturnCode::from_u8(130), Some(ConnectReturnCode::ProtocolError));
        assert_eq!(ConnectReturnCode::from_u8(131), Some(ConnectReturnCode::ImplementationSpecificError));
        assert_eq!(ConnectReturnCode::from_u8(132), Some(ConnectReturnCode::UnsupportedProtocolVersion));
        assert_eq!(ConnectReturnCode::from_u8(133), Some(ConnectReturnCode::ClientIdentifierNotValid));
        assert_eq!(ConnectReturnCode::from_u8(134), Some(ConnectReturnCode::BadUsernameOrPasswordV5));
        assert_eq!(ConnectReturnCode::from_u8(135), Some(ConnectReturnCode::NotAuthorizedV5));
        assert_eq!(ConnectReturnCode::from_u8(136), Some(ConnectReturnCode::ServerUnavailableV5));
        assert_eq!(ConnectReturnCode::from_u8(137), Some(ConnectReturnCode::ServerBusy));
        assert_eq!(ConnectReturnCode::from_u8(138), Some(ConnectReturnCode::Banned));
        assert_eq!(ConnectReturnCode::from_u8(140), Some(ConnectReturnCode::BadAuthenticationMethod));
        assert_eq!(ConnectReturnCode::from_u8(144), Some(ConnectReturnCode::TopicNameInvalid));
        assert_eq!(ConnectReturnCode::from_u8(149), Some(ConnectReturnCode::PacketTooLarge));
        assert_eq!(ConnectReturnCode::from_u8(151), Some(ConnectReturnCode::QuotaExceeded));
        assert_eq!(ConnectReturnCode::from_u8(153), Some(ConnectReturnCode::PayloadFormatInvalid));
        assert_eq!(ConnectReturnCode::from_u8(154), Some(ConnectReturnCode::RetainNotSupported));
        assert_eq!(ConnectReturnCode::from_u8(155), Some(ConnectReturnCode::QoSNotSupported));
        assert_eq!(ConnectReturnCode::from_u8(156), Some(ConnectReturnCode::UseAnotherServer));
        assert_eq!(ConnectReturnCode::from_u8(157), Some(ConnectReturnCode::ServerMoved));
        assert_eq!(ConnectReturnCode::from_u8(159), Some(ConnectReturnCode::ConnectionRateExceeded));
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
        assert_eq!(ConnectReturnCode::MalformedPacket as u8, 128);
        assert_eq!(ConnectReturnCode::ProtocolError as u8, 130);
        assert_eq!(ConnectReturnCode::ImplementationSpecificError as u8, 131);
        assert_eq!(ConnectReturnCode::UnsupportedProtocolVersion as u8, 132);
        assert_eq!(ConnectReturnCode::ClientIdentifierNotValid as u8, 133);
        assert_eq!(ConnectReturnCode::BadUsernameOrPasswordV5 as u8, 134);
        assert_eq!(ConnectReturnCode::NotAuthorizedV5 as u8, 135);
        assert_eq!(ConnectReturnCode::ServerUnavailableV5 as u8, 136);
        assert_eq!(ConnectReturnCode::ServerBusy as u8, 137);
        assert_eq!(ConnectReturnCode::Banned as u8, 138);
        assert_eq!(ConnectReturnCode::BadAuthenticationMethod as u8, 140);
        assert_eq!(ConnectReturnCode::TopicNameInvalid as u8, 144);
        assert_eq!(ConnectReturnCode::PacketTooLarge as u8, 149);
        assert_eq!(ConnectReturnCode::QuotaExceeded as u8, 151);
        assert_eq!(ConnectReturnCode::PayloadFormatInvalid as u8, 153);
        assert_eq!(ConnectReturnCode::RetainNotSupported as u8, 154);
        assert_eq!(ConnectReturnCode::QoSNotSupported as u8, 155);
        assert_eq!(ConnectReturnCode::UseAnotherServer as u8, 156);
        assert_eq!(ConnectReturnCode::ServerMoved as u8, 157);
        assert_eq!(ConnectReturnCode::ConnectionRateExceeded as u8, 159);
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
        let mut props = PublishProperties::new();
        assert!(props.is_empty());

        props = props
            .payload_format_indicator(PAYLOAD_FORMAT_INDICATOR_UTF8)
            .message_expiry_interval(3600)
            .topic_alias(123)
            .response_topic("response/topic".to_string())
            .correlation_data(Bytes::from("correlation_data"))
            .subscription_identifier(456)
            .content_type("application/json".to_string())
            .user_property("key1".to_string(), "value1".to_string())
            .user_property("key2".to_string(), "value2".to_string());

        assert!(!props.is_empty());
        assert_eq!(props.payload_format_indicator, Some(PAYLOAD_FORMAT_INDICATOR_UTF8));
        assert_eq!(props.message_expiry_interval, Some(3600));
        assert_eq!(props.topic_alias, Some(123));
        assert_eq!(props.response_topic, Some("response/topic".to_string()));
        assert_eq!(props.correlation_data, Some(Bytes::from("correlation_data")));
        assert_eq!(props.subscription_identifier, Some(456));
        assert_eq!(props.content_type, Some("application/json".to_string()));
        assert_eq!(props.user_properties.len(), 2);
        assert_eq!(props.user_properties.get("key1"), Some(&"value1".to_string()));
        assert_eq!(props.user_properties.get("key2"), Some(&"value2".to_string()));

        // Test payload format indicator methods
        assert_eq!(props.payload_format_indicator_str(), Some("UTF-8"));
        assert!(props.is_utf8_payload());
        assert!(!props.is_unspecified_payload());

        // Test with unspecified payload format
        let mut props2 = PublishProperties::new();
        props2 = props2.payload_format_indicator(PAYLOAD_FORMAT_INDICATOR_UNSPECIFIED);
        assert_eq!(props2.payload_format_indicator_str(), Some("Unspecified"));
        assert!(props2.is_unspecified_payload());
        assert!(!props2.is_utf8_payload());
    }

    #[test]
    fn test_publish_packet_with_properties() {
        let payload = Bytes::from("Hello, MQTT v5!");
        let mut packet = PublishPacket::new("test/topic".to_string(), payload.clone());

        // Test basic creation
        assert_eq!(packet.topic_name, "test/topic");
        assert_eq!(packet.payload, payload);
        assert!(packet.packet_id.is_none());
        assert!(packet.properties.is_none());
        assert!(!packet.has_properties());
        assert_eq!(packet.qos(), 0);

        // Test with QoS and packet ID
        let packet_with_qos = PublishPacket::with_qos("test/topic2".to_string(), payload.clone(), 1, 123);
        assert_eq!(packet_with_qos.topic_name, "test/topic2");
        assert_eq!(packet_with_qos.packet_id, Some(123));
        assert_eq!(packet_with_qos.qos(), 1);

        // Test setting properties using builder pattern
        packet = packet
            .payload_format_indicator(PAYLOAD_FORMAT_INDICATOR_UTF8)
            .message_expiry_interval(7200)
            .topic_alias(456)
            .response_topic("response/topic".to_string())
            .correlation_data(Bytes::from("corr_data"))
            .subscription_identifier(789)
            .content_type("text/plain".to_string())
            .user_property("app".to_string(), "test".to_string());

        assert!(packet.has_properties());
        if let Some(props) = &packet.properties {
            assert_eq!(props.payload_format_indicator, Some(PAYLOAD_FORMAT_INDICATOR_UTF8));
            assert_eq!(props.message_expiry_interval, Some(7200));
            assert_eq!(props.topic_alias, Some(456));
            assert_eq!(props.response_topic, Some("response/topic".to_string()));
            assert_eq!(props.correlation_data, Some(Bytes::from("corr_data")));
            assert_eq!(props.subscription_identifier, Some(789));
            assert_eq!(props.content_type, Some("text/plain".to_string()));
            assert_eq!(props.user_properties.get("app"), Some(&"test".to_string()));
        }

        // Test setting packet ID
        packet = packet.packet_id(999);
        assert_eq!(packet.packet_id, Some(999));
        assert_eq!(packet.qos(), 1);
    }

    #[test]
    fn test_publish_properties_edge_cases() {
        // Test empty properties
        let props = PublishProperties::new();
        assert!(props.is_empty());
        assert_eq!(props.payload_format_indicator_str(), None);
        assert!(!props.is_utf8_payload());
        assert!(!props.is_unspecified_payload());

        // Test with invalid payload format indicator
        let mut props2 = PublishProperties::new();
        props2 = props2.payload_format_indicator(0xFF);
        assert_eq!(props2.payload_format_indicator_str(), None);
        assert!(!props2.is_utf8_payload());
        assert!(!props2.is_unspecified_payload());

        // Test user properties with different keys
        let mut props3 = PublishProperties::new();
        props3 = props3
            .user_property("empty_key".to_string(), "".to_string())
            .user_property("key".to_string(), "".to_string())
            .user_property("empty_value".to_string(), "value".to_string());
        
        assert_eq!(props3.user_properties.len(), 3);
        assert_eq!(props3.user_properties.get("empty_key"), Some(&"".to_string()));
        assert_eq!(props3.user_properties.get("key"), Some(&"".to_string()));
        assert_eq!(props3.user_properties.get("empty_value"), Some(&"value".to_string()));
    }

    #[test]
    fn test_publish_packet_properties_builder() {
        let payload = Bytes::from("Test payload");
        let packet = PublishPacket::new("test/topic".to_string(), payload)
            .payload_format_indicator(PAYLOAD_FORMAT_INDICATOR_UTF8)
            .message_expiry_interval(3600)
            .topic_alias(123)
            .response_topic("response/topic".to_string())
            .correlation_data(Bytes::from("corr_data"))
            .subscription_identifier(456)
            .content_type("application/json".to_string())
            .user_property("version".to_string(), "5.0".to_string())
            .user_property("priority".to_string(), "high".to_string());

        assert!(packet.has_properties());
        if let Some(props) = &packet.properties {
            assert!(!props.is_empty());
            assert_eq!(props.payload_format_indicator, Some(PAYLOAD_FORMAT_INDICATOR_UTF8));
            assert_eq!(props.message_expiry_interval, Some(3600));
            assert_eq!(props.topic_alias, Some(123));
            assert_eq!(props.response_topic, Some("response/topic".to_string()));
            assert_eq!(props.correlation_data, Some(Bytes::from("corr_data")));
            assert_eq!(props.subscription_identifier, Some(456));
            assert_eq!(props.content_type, Some("application/json".to_string()));
            assert_eq!(props.user_properties.len(), 2);
            assert_eq!(props.user_properties.get("version"), Some(&"5.0".to_string()));
            assert_eq!(props.user_properties.get("priority"), Some(&"high".to_string()));
        }
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