//! Message routing module

use crate::types::Message;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;


/// Message router for handling message distribution and retained messages
pub struct MessageRouter {
    retained_messages: Arc<RwLock<HashMap<String, Message>>>,
}

impl MessageRouter {
    pub fn new() -> Self {
        Self {
            retained_messages: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Store a retained message
    pub async fn store_retained_message(&self, topic: String, message: Message) {
        let mut retained = self.retained_messages.write().await;
        retained.insert(topic, message);
    }

    /// Clear a retained message (empty payload with retain flag)
    pub async fn clear_retained_message(&self, topic: &str) {
        let mut retained = self.retained_messages.write().await;
        retained.remove(topic);
    }

    /// Get a retained message for a topic
    pub async fn get_retained_message(&self, topic: &str) -> Option<Message> {
        let retained = self.retained_messages.read().await;
        retained.get(topic).cloned()
    }

    /// Get all retained messages
    pub async fn get_all_retained_messages(&self) -> HashMap<String, Message> {
        let retained = self.retained_messages.read().await;
        retained.clone()
    }

    /// Check if a topic matches a topic filter (with wildcards)
    pub fn topic_matches(filter: &str, topic: &str) -> bool {
        // MQTT topic matching with wildcards
        let filter_parts: Vec<&str> = filter.split('/').collect();
        let topic_parts: Vec<&str> = topic.split('/').collect();
        
        let mut filter_idx = 0;
        let mut topic_idx = 0;
        
        while filter_idx < filter_parts.len() && topic_idx < topic_parts.len() {
            let filter_part = filter_parts[filter_idx];
            let topic_part = topic_parts[topic_idx];
            
            if filter_part == "#" {
                // Multi-level wildcard - matches everything
                return true;
            } else if filter_part == "+" {
                // Single-level wildcard - matches any single level
                filter_idx += 1;
                topic_idx += 1;
            } else if filter_part == topic_part {
                // Exact match
                filter_idx += 1;
                topic_idx += 1;
            } else {
                // No match
                return false;
            }
        }
        
        // Check if we've processed all parts
        if filter_idx == filter_parts.len() && topic_idx == topic_parts.len() {
            return true;
        }
        
        // Handle trailing # wildcard
        if filter_idx == filter_parts.len() - 1 && filter_parts[filter_idx] == "#" {
            return true;
        }
        
        false
    }

    /// Find matching topics for a given topic filter
    pub async fn find_matching_topics(&self, topic_filter: &str) -> Vec<String> {
        let retained = self.retained_messages.read().await;
        let mut matching_topics = Vec::new();
        
        for topic in retained.keys() {
            if Self::topic_matches(topic_filter, topic) {
                matching_topics.push(topic.clone());
            }
        }
        
        matching_topics
    }

    /// Get retained messages for matching topic filters
    pub async fn get_retained_messages_for_filters(&self, topic_filters: &[String]) -> Vec<Message> {
        let mut messages = Vec::new();
        
        for topic_filter in topic_filters {
            let matching_topics = self.find_matching_topics(topic_filter).await;
            for topic in matching_topics {
                if let Some(message) = self.get_retained_message(&topic).await {
                    messages.push(message);
                }
            }
        }
        
        messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn test_topic_matches() {
        // Exact match
        assert!(MessageRouter::topic_matches("home/temp", "home/temp"));
        
        // Single level wildcard
        assert!(MessageRouter::topic_matches("home/+/temp", "home/living/temp"));
        assert!(MessageRouter::topic_matches("home/+/temp", "home/bedroom/temp"));
        assert!(!MessageRouter::topic_matches("home/+/temp", "home/living/bedroom/temp"));
        
        // Multi-level wildcard
        assert!(MessageRouter::topic_matches("home/#", "home/living/temp"));
        assert!(MessageRouter::topic_matches("home/#", "home/bedroom/humidity"));
        assert!(MessageRouter::topic_matches("home/#", "home"));
        assert!(!MessageRouter::topic_matches("home/#", "office/temp"));
        
        // Edge cases
        assert!(MessageRouter::topic_matches("#", "any/topic"));
        assert!(MessageRouter::topic_matches("+", "single"));
        assert!(!MessageRouter::topic_matches("home/+/temp", "home/temp"));
    }

    #[tokio::test]
    async fn test_message_router() {
        let router = MessageRouter::new();
        
        // Create a test message
        let message = Message {
            topic: "test/topic".to_string(),
            payload: Bytes::from("test message"),
            qos: 0,
            retain: true,
            dup: false,
            packet_id: None,
        };
        
        // Store retained message
        router.store_retained_message("test/topic".to_string(), message.clone()).await;
        
        // Get retained message
        let retrieved = router.get_retained_message("test/topic").await;
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.topic, "test/topic");
        assert_eq!(retrieved.payload, Bytes::from("test message"));
        
        // Find matching topics
        let matching = router.find_matching_topics("test/+").await;
        assert_eq!(matching.len(), 1);
        assert_eq!(matching[0], "test/topic");
        
        // Clear retained message
        router.clear_retained_message("test/topic").await;
        let retrieved = router.get_retained_message("test/topic").await;
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_retained_messages_for_filters() {
        let router = MessageRouter::new();
        
        // Create test messages
        let message1 = Message {
            topic: "home/living/temp".to_string(),
            payload: Bytes::from("22.5"),
            qos: 0,
            retain: true,
            dup: false,
            packet_id: None,
        };
        
        let message2 = Message {
            topic: "home/bedroom/temp".to_string(),
            payload: Bytes::from("20.0"),
            qos: 0,
            retain: true,
            dup: false,
            packet_id: None,
        };
        
        // Store messages
        router.store_retained_message("home/living/temp".to_string(), message1).await;
        router.store_retained_message("home/bedroom/temp".to_string(), message2).await;
        
        // Get messages for wildcard filter
        let topic_filters = vec!["home/+/temp".to_string()];
        let messages = router.get_retained_messages_for_filters(&topic_filters).await;
        assert_eq!(messages.len(), 2);
        
        // Get messages for specific topic
        let topic_filters = vec!["home/living/temp".to_string()];
        let messages = router.get_retained_messages_for_filters(&topic_filters).await;
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].topic, "home/living/temp");
    }
}
