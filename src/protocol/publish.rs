//! Publish options for MQTT messages
//! 
//! This module provides publish configuration options for MQTT clients,
//! including topic, payload, QoS, retain flags, and packet identification.

use super::qos::QoS;

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
    /// Create new publish options with default values
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

    /// Set QoS level
    pub fn qos(mut self, qos: QoS) -> Self {
        self.qos = qos;
        self
    }

    /// Set retain flag
    pub fn retain(mut self, retain: bool) -> Self {
        self.retain = retain;
        self
    }

    /// Set duplicate flag
    pub fn dup(mut self, dup: bool) -> Self {
        self.dup = dup;
        self
    }

    /// Set packet ID
    pub fn packet_id(mut self, packet_id: u16) -> Self {
        self.packet_id = Some(packet_id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
