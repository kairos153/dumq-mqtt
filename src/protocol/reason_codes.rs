//! MQTT 5.0 Reason Codes
//! 
//! This module defines the reason codes used in MQTT 5.0 for various operations
//! including connection, subscription, and publish operations.

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
    /// Create ReasonCode from u8 value
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
}
