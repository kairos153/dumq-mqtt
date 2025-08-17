//! # MQTT Subscribe Types
//! 
//! This module defines MQTT subscription-related structures including subscribe,
//! unsubscribe, and acknowledgment packets.

use super::properties::*;

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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_edge_cases() {
        // Test with empty topic filters
        let empty_subscribe = SubscribePacket {
            packet_id: 0,
            topic_filters: vec![],
            properties: None,
        };

        assert_eq!(empty_subscribe.packet_id, 0);
        assert_eq!(empty_subscribe.topic_filters.len(), 0);

        // Test with maximum packet ID
        let max_subscribe = SubscribePacket {
            packet_id: u16::MAX,
            topic_filters: vec![TopicFilter {
                topic: "test/topic".to_string(),
                qos: 0,
                no_local: false,
                retain_as_published: false,
                retain_handling: 0,
            }],
            properties: None,
        };

        assert_eq!(max_subscribe.packet_id, u16::MAX);
        assert_eq!(max_subscribe.topic_filters.len(), 1);

        // Test with empty topic string
        let empty_topic_filter = TopicFilter {
            topic: "".to_string(),
            qos: 0,
            no_local: false,
            retain_as_published: false,
            retain_handling: 0,
        };

        assert_eq!(empty_topic_filter.topic, "");
        assert_eq!(empty_topic_filter.qos, 0);
    }

    #[test]
    fn test_topic_filter_qos_values() {
        // Test all valid QoS values
        for qos in 0..=2 {
            let filter = TopicFilter {
                topic: format!("test/qos{}", qos),
                qos,
                no_local: false,
                retain_as_published: false,
                retain_handling: 0,
            };
            assert_eq!(filter.qos, qos);
        }

        // Test retain handling values
        for handling in 0..=2 {
            let filter = TopicFilter {
                topic: "test/handling".to_string(),
                qos: 0,
                no_local: false,
                retain_as_published: false,
                retain_handling: handling,
            };
            assert_eq!(filter.retain_handling, handling);
        }
    }
}
