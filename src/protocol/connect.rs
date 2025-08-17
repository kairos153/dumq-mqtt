//! Connection options for MQTT clients
//! 
//! This module provides comprehensive connection configuration for MQTT clients,
//! including authentication, will messages, keep-alive settings, and MQTT 5.0 properties.

use crate::types::ConnectProperties;
use bytes::Bytes;
use std::time::Duration;
use super::qos::QoS;

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
    pub properties: Option<ConnectProperties>,
}

impl ConnectOptions {
    /// Create new connect options with default values
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
            properties: None,
        }
    }

    /// Set clean session flag
    pub fn clean_session(mut self, clean: bool) -> Self {
        self.clean_session = clean;
        self
    }

    /// Set keep alive duration
    pub fn keep_alive(mut self, duration: Duration) -> Self {
        self.keep_alive = duration;
        self
    }

    /// Set username for authentication
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    /// Set password for authentication
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Set will message configuration
    pub fn will(mut self, topic: impl Into<String>, message: impl Into<Vec<u8>>, qos: QoS, retain: bool) -> Self {
        self.will_topic = Some(topic.into());
        self.will_message = Some(message.into());
        self.will_qos = qos;
        self.will_retain = retain;
        self
    }

    /// Set protocol version
    pub fn protocol_version(mut self, version: u8) -> Self {
        self.protocol_version = version;
        self
    }

    // MQTT 5.0 Properties
    /// Set session expiry interval (MQTT 5.0)
    pub fn session_expiry_interval(mut self, interval: u32) -> Self {
        if self.protocol_version == 5 {
            if self.properties.is_none() {
                self.properties = Some(ConnectProperties::default());
            }
            if let Some(ref mut props) = self.properties {
                props.session_expiry_interval = Some(interval);
            }
        }
        self
    }

    /// Set receive maximum (MQTT 5.0)
    pub fn receive_maximum(mut self, max: u16) -> Self {
        if self.protocol_version == 5 {
            if self.properties.is_none() {
                self.properties = Some(ConnectProperties::default());
            }
            if let Some(ref mut props) = self.properties {
                props.receive_maximum = Some(max);
            }
        }
        self
    }

    /// Set maximum packet size (MQTT 5.0)
    pub fn max_packet_size(mut self, size: u32) -> Self {
        if self.protocol_version == 5 {
            if self.properties.is_none() {
                self.properties = Some(ConnectProperties::default());
            }
            if let Some(ref mut props) = self.properties {
                props.max_packet_size = Some(size);
            }
        }
        self
    }

    /// Set topic alias maximum (MQTT 5.0)
    pub fn topic_alias_maximum(mut self, max: u16) -> Self {
        if self.protocol_version == 5 {
            if self.properties.is_none() {
                self.properties = Some(ConnectProperties::default());
            }
            if let Some(ref mut props) = self.properties {
                props.topic_alias_maximum = Some(max);
            }
        }
        self
    }

    /// Set request response information flag (MQTT 5.0)
    pub fn request_response_information(mut self, request: bool) -> Self {
        if self.protocol_version == 5 {
            if self.properties.is_none() {
                self.properties = Some(ConnectProperties::default());
            }
            if let Some(ref mut props) = self.properties {
                props.request_response_information = Some(request);
            }
        }
        self
    }

    /// Set request problem information flag (MQTT 5.0)
    pub fn request_problem_information(mut self, request: bool) -> Self {
        if self.protocol_version == 5 {
            if self.properties.is_none() {
                self.properties = Some(ConnectProperties::default());
            }
            if let Some(ref mut props) = self.properties {
                props.request_problem_information = Some(request);
            }
        }
        self
    }

    /// Add user property (MQTT 5.0)
    pub fn user_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.protocol_version == 5 {
            if self.properties.is_none() {
                self.properties = Some(ConnectProperties::default());
            }
            if let Some(ref mut props) = self.properties {
                props.user_properties.insert(key.into(), value.into());
            }
        }
        self
    }

    /// Set authentication method (MQTT 5.0)
    pub fn authentication_method(mut self, method: impl Into<String>) -> Self {
        if self.protocol_version == 5 {
            if self.properties.is_none() {
                self.properties = Some(ConnectProperties::default());
            }
            if let Some(ref mut props) = self.properties {
                props.authentication_method = Some(method.into());
            }
        }
        self
    }

    /// Set authentication data (MQTT 5.0)
    pub fn authentication_data(mut self, data: impl Into<Vec<u8>>) -> Self {
        if self.protocol_version == 5 {
            if self.properties.is_none() {
                self.properties = Some(ConnectProperties::default());
            }
            if let Some(ref mut props) = self.properties {
                props.authentication_data = Some(Bytes::from(data.into()));
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(options.properties.is_none());
    }

    #[test]
    fn test_connect_options_builder_pattern() {
        let options = ConnectOptions::new("test_client")
            .clean_session(false)
            .keep_alive(Duration::from_secs(120))
            .username("test_user")
            .password("test_pass")
            .will("test/will", b"will message", QoS::AtLeastOnce, true)
            .protocol_version(5)
            .session_expiry_interval(3600)
            .receive_maximum(100)
            .max_packet_size(1_000_000)
            .topic_alias_maximum(20)
            .request_response_information(true)
            .request_problem_information(false)
            .user_property("test_key", "test_value")
            .authentication_method("test_method")
            .authentication_data(b"test_data");

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
        assert_eq!(options.properties.as_ref().unwrap().session_expiry_interval, Some(3600));
        assert_eq!(options.properties.as_ref().unwrap().receive_maximum, Some(100));
        assert_eq!(options.properties.as_ref().unwrap().max_packet_size, Some(1_000_000));
        assert_eq!(options.properties.as_ref().unwrap().topic_alias_maximum, Some(20));
        assert_eq!(options.properties.as_ref().unwrap().request_response_information, Some(true));
        assert_eq!(options.properties.as_ref().unwrap().request_problem_information, Some(false));
        assert_eq!(options.properties.as_ref().unwrap().user_properties.get("test_key"), Some(&"test_value".to_string()));
        assert_eq!(options.properties.as_ref().unwrap().authentication_method, Some("test_method".to_string()));
        assert_eq!(options.properties.as_ref().unwrap().authentication_data.as_ref().unwrap(), &Bytes::from(&b"test_data"[..]));
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
        assert_eq!(options.properties.is_some(), cloned_options.properties.is_some());
    }

    #[test]
    fn test_connect_options_with_empty_strings() {
        let options = ConnectOptions::new("")
            .username("")
            .password("");

        assert_eq!(options.client_id, "");
        assert_eq!(options.username, Some("".to_string()));
        assert_eq!(options.password, Some("".to_string()));
        assert!(options.properties.is_none());
    }

    #[test]
    fn test_connect_options_with_unicode() {
        let options = ConnectOptions::new("한국어_클라이언트")
            .username("한국어_사용자")
            .password("한국어_비밀번호");

        assert_eq!(options.client_id, "한국어_클라이언트");
        assert_eq!(options.username, Some("한국어_사용자".to_string()));
        assert_eq!(options.password, Some("한국어_비밀번호".to_string()));
        assert!(options.properties.is_none());
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
