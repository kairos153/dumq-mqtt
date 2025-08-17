//! # MQTT 5.0 Properties
//! 
//! This module defines all MQTT 5.0 property structures for enhanced functionality
//! including connection properties, publish properties, and other packet properties.

use bytes::Bytes;
use std::collections::HashMap;
use super::constants::*;

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

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

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
    fn test_conn_ack_properties() {
        let mut props = ConnAckProperties::new();
        assert!(props.is_empty());

        props = props
            .session_expiry_interval(3600)
            .receive_maximum(100)
            .max_qos(2)
            .retain_available(true)
            .max_packet_size(1024)
            .assigned_client_identifier("client123".to_string())
            .topic_alias_maximum(10)
            .reason_string("Success".to_string())
            .user_property("key1".to_string(), "value1".to_string())
            .user_property("key2".to_string(), "value2".to_string());

        assert!(!props.is_empty());
        assert_eq!(props.session_expiry_interval, Some(3600));
        assert_eq!(props.receive_maximum, Some(100));
        assert_eq!(props.max_qos, Some(2));
        assert_eq!(props.retain_available, Some(true));
        assert_eq!(props.max_packet_size, Some(1024));
        assert_eq!(props.assigned_client_identifier, Some("client123".to_string()));
        assert_eq!(props.topic_alias_maximum, Some(10));
        assert_eq!(props.reason_string, Some("Success".to_string()));
        assert_eq!(props.user_properties.len(), 2);
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
