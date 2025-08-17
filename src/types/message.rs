//! # MQTT Message Types
//! 
//! This module defines MQTT message-related structures for application-level
//! message handling and processing.

use bytes::Bytes;

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
    fn test_message_edge_cases() {
        // Test with empty topic and payload
        let empty_payload = Bytes::from("");
        let message = Message {
            topic: "".to_string(),
            payload: empty_payload.clone(),
            qos: 0,
            retain: false,
            dup: false,
            packet_id: None,
        };

        assert_eq!(message.topic, "");
        assert_eq!(message.payload, empty_payload);
        assert_eq!(message.qos, 0);
        assert_eq!(message.retain, false);
        assert_eq!(message.dup, false);
        assert_eq!(message.packet_id, None);

        // Test with maximum values
        let max_message = Message {
            topic: "test/max".to_string(),
            payload: Bytes::from("max payload"),
            qos: 2,
            retain: true,
            dup: true,
            packet_id: Some(u16::MAX),
        };

        assert_eq!(max_message.qos, 2);
        assert_eq!(max_message.retain, true);
        assert_eq!(max_message.dup, true);
        assert_eq!(max_message.packet_id, Some(u16::MAX));
    }

    #[test]
    fn test_message_clone() {
        let original = Message {
            topic: "test/clone".to_string(),
            payload: Bytes::from("clone test"),
            qos: 1,
            retain: false,
            dup: false,
            packet_id: Some(456),
        };

        let cloned = original.clone();

        assert_eq!(original.topic, cloned.topic);
        assert_eq!(original.payload, cloned.payload);
        assert_eq!(original.qos, cloned.qos);
        assert_eq!(original.retain, cloned.retain);
        assert_eq!(original.dup, cloned.dup);
        assert_eq!(original.packet_id, cloned.packet_id);
    }

    #[test]
    fn test_message_different_qos_levels() {
        // Test QoS 0
        let qos0_message = Message {
            topic: "test/qos0".to_string(),
            payload: Bytes::from("QoS 0"),
            qos: 0,
            retain: false,
            dup: false,
            packet_id: None,
        };
        assert_eq!(qos0_message.qos, 0);
        assert_eq!(qos0_message.packet_id, None);

        // Test QoS 1
        let qos1_message = Message {
            topic: "test/qos1".to_string(),
            payload: Bytes::from("QoS 1"),
            qos: 1,
            retain: false,
            dup: false,
            packet_id: Some(123),
        };
        assert_eq!(qos1_message.qos, 1);
        assert_eq!(qos1_message.packet_id, Some(123));

        // Test QoS 2
        let qos2_message = Message {
            topic: "test/qos2".to_string(),
            payload: Bytes::from("QoS 2"),
            qos: 2,
            retain: false,
            dup: false,
            packet_id: Some(456),
        };
        assert_eq!(qos2_message.qos, 2);
        assert_eq!(qos2_message.packet_id, Some(456));
    }
}
