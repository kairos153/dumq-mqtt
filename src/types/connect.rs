//! # MQTT Connection Types
//! 
//! This module defines MQTT connection-related structures including connect,
//! connection acknowledgment, disconnect, and authentication packets.

use bytes::Bytes;
use super::properties::*;

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

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

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
    fn test_conn_ack_packet_builder() {
        let conn_ack = ConnAckPacket::new(ConnectReturnCode::Accepted)
            .session_present(true)
            .return_code(ConnectReturnCode::Accepted);

        assert_eq!(conn_ack.session_present, true);
        assert_eq!(conn_ack.return_code, ConnectReturnCode::Accepted);
        assert!(conn_ack.is_success());
        assert!(!conn_ack.is_error());
        assert_eq!(conn_ack.error_message(), None);
    }

    #[test]
    fn test_conn_ack_packet_error() {
        let conn_ack = ConnAckPacket::new(ConnectReturnCode::BadUsernameOrPassword);
        
        assert!(!conn_ack.is_success());
        assert!(conn_ack.is_error());
        assert_eq!(conn_ack.error_message(), Some("Bad username or password"));
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
    }
}
