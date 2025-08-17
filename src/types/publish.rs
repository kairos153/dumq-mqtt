//! # MQTT Publish Types
//! 
//! This module defines MQTT publish-related structures including publish packets,
//! acknowledgments, and related functionality.

use bytes::Bytes;
use super::properties::*;
use super::constants::*;

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

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

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
    fn test_publish_packet_edge_cases() {
        // Test with empty topic and payload
        let empty_payload = Bytes::from("");
        let packet = PublishPacket::new("".to_string(), empty_payload.clone());
        
        assert_eq!(packet.topic_name, "");
        assert_eq!(packet.payload, empty_payload);
        assert_eq!(packet.qos(), 0);
        assert!(!packet.has_properties());

        // Test with maximum packet ID
        let packet = packet.packet_id(u16::MAX);
        assert_eq!(packet.packet_id, Some(u16::MAX));
        assert_eq!(packet.qos(), 1);
    }
}
