//! Subscribe options for MQTT subscriptions
//! 
//! This module provides subscription configuration options for MQTT clients,
//! including topic filters and packet identification.

use crate::types::TopicFilter;

/// Subscribe options
#[derive(Debug, Clone)]
pub struct SubscribeOptions {
    pub topic_filters: Vec<TopicFilter>,
    pub packet_id: u16,
}

impl SubscribeOptions {
    /// Create new subscribe options
    pub fn new(topic_filters: Vec<TopicFilter>, packet_id: u16) -> Self {
        Self {
            topic_filters,
            packet_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TopicFilter;

    #[test]
    fn test_subscribe_options() {
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

        let options = SubscribeOptions::new(topic_filters.clone(), 123);

        assert_eq!(options.topic_filters.len(), 2);
        assert_eq!(options.topic_filters[0].topic, "test/topic1");
        assert_eq!(options.topic_filters[0].qos, 0);
        assert_eq!(options.topic_filters[1].topic, "test/topic2");
        assert_eq!(options.topic_filters[1].qos, 1);
        assert_eq!(options.packet_id, 123);
    }

    #[test]
    fn test_subscribe_options_clone() {
        let topic_filters = vec![TopicFilter {
            topic: "test/topic".to_string(),
            qos: 1,
            no_local: false,
            retain_as_published: false,
            retain_handling: 0,
        }];

        let options = SubscribeOptions::new(topic_filters, 456);
        let cloned_options = options.clone();

        assert_eq!(options.packet_id, cloned_options.packet_id);
        assert_eq!(options.topic_filters.len(), cloned_options.topic_filters.len());
    }
}
