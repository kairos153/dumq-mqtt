//! Session management module

use crate::protocol::QoS;
use crate::types::Message;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// MQTT session
#[derive(Debug, Clone)]
pub struct Session {
    pub client_id: String,
    pub username: Option<String>,
    pub clean_session: bool,
    pub subscriptions: HashMap<String, QoS>,
    pub pending_messages: Vec<Message>,
}

impl Session {
    pub fn new(client_id: String, username: Option<String>, clean_session: bool) -> Self {
        Self {
            client_id,
            username,
            clean_session,
            subscriptions: HashMap::new(),
            pending_messages: Vec::new(),
        }
    }
}

/// MQTT subscription
#[derive(Debug, Clone)]
pub struct Subscription {
    pub client_id: String,
    pub topic_filter: String,
    pub qos: QoS,
}

impl Subscription {
    pub fn new(client_id: String, topic_filter: String, qos: QoS) -> Self {
        Self {
            client_id,
            topic_filter,
            qos,
        }
    }
}

/// Session manager for handling multiple client sessions
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    subscriptions: Arc<RwLock<HashMap<String, Vec<Subscription>>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create or update a session
    pub async fn create_session(&self, client_id: String, username: Option<String>, clean_session: bool) {
        let mut sessions = self.sessions.write().await;
        let session = Session::new(client_id.clone(), username, clean_session);
        sessions.insert(client_id, session);
    }

    /// Get a session by client ID
    pub async fn get_session(&self, client_id: &str) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(client_id).cloned()
    }

    /// Remove a session
    pub async fn remove_session(&self, client_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(client_id);
    }

    /// Add a subscription
    pub async fn add_subscription(&self, client_id: String, topic_filter: String, qos: QoS) {
        let mut subscriptions = self.subscriptions.write().await;
        let subscription = Subscription::new(client_id.clone(), topic_filter.clone(), qos);
        
        subscriptions
            .entry(topic_filter.clone())
            .or_insert_with(Vec::new)
            .push(subscription);

        // Update session
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&client_id) {
            session.subscriptions.insert(topic_filter, qos);
        }
    }

    /// Remove a subscription
    pub async fn remove_subscription(&self, client_id: &str, topic_filter: &str) {
        let mut subscriptions = self.subscriptions.write().await;
        if let Some(subs) = subscriptions.get_mut(topic_filter) {
            subs.retain(|sub| sub.client_id != client_id);
            if subs.is_empty() {
                subscriptions.remove(topic_filter);
            }
        }

        // Update session
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(client_id) {
            session.subscriptions.remove(topic_filter);
        }
    }

    /// Get all subscriptions for a topic
    pub async fn get_subscriptions(&self, topic: &str) -> Vec<Subscription> {
        let subscriptions = self.subscriptions.read().await;
        subscriptions.get(topic).cloned().unwrap_or_default()
    }

    /// Get all subscriptions
    pub async fn get_all_subscriptions(&self) -> HashMap<String, Vec<Subscription>> {
        let subscriptions = self.subscriptions.read().await;
        subscriptions.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::QoS;

    #[test]
    fn test_session_new() {
        let session = Session::new("client1".to_string(), Some("user1".to_string()), true);
        assert_eq!(session.client_id, "client1");
        assert_eq!(session.username, Some("user1".to_string()));
        assert_eq!(session.clean_session, true);
        assert!(session.subscriptions.is_empty());
        assert!(session.pending_messages.is_empty());
    }

    #[test]
    fn test_session_add_subscription() {
        let mut session = Session::new("client1".to_string(), None, false);
        session.subscriptions.insert("topic1".to_string(), QoS::AtLeastOnce);
        session.subscriptions.insert("topic2".to_string(), QoS::ExactlyOnce);

        assert_eq!(session.subscriptions.len(), 2);
        assert_eq!(session.subscriptions.get("topic1"), Some(&QoS::AtLeastOnce));
        assert_eq!(session.subscriptions.get("topic2"), Some(&QoS::ExactlyOnce));
    }

    #[test]
    fn test_session_clone() {
        let session1 = Session::new("client1".to_string(), Some("user1".to_string()), true);
        let session2 = session1.clone();
        assert_eq!(session1.client_id, session2.client_id);
        assert_eq!(session1.username, session2.username);
        assert_eq!(session1.clean_session, session2.clean_session);
    }

    #[test]
    fn test_subscription_new() {
        let subscription = Subscription::new("client1".to_string(), "topic1".to_string(), QoS::AtMostOnce);
        assert_eq!(subscription.client_id, "client1");
        assert_eq!(subscription.topic_filter, "topic1");
        assert_eq!(subscription.qos, QoS::AtMostOnce);
    }

    #[test]
    fn test_subscription_clone() {
        let sub1 = Subscription::new("client1".to_string(), "topic1".to_string(), QoS::AtLeastOnce);
        let sub2 = sub1.clone();
        assert_eq!(sub1.client_id, sub2.client_id);
        assert_eq!(sub1.topic_filter, sub2.topic_filter);
        assert_eq!(sub1.qos, sub2.qos);
    }

    #[tokio::test]
    async fn test_session_manager() {
        let manager = SessionManager::new();
        
        // Create session
        manager.create_session("client1".to_string(), Some("user1".to_string()), true).await;
        
        // Get session
        let session = manager.get_session("client1").await;
        assert!(session.is_some());
        let session = session.unwrap();
        assert_eq!(session.client_id, "client1");
        assert_eq!(session.username, Some("user1".to_string()));
        
        // Add subscription
        manager.add_subscription("client1".to_string(), "topic1".to_string(), QoS::AtLeastOnce).await;
        
        // Get subscriptions
        let subs = manager.get_subscriptions("topic1").await;
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].client_id, "client1");
        assert_eq!(subs[0].topic_filter, "topic1");
        
        // Remove subscription
        manager.remove_subscription("client1", "topic1").await;
        let subs = manager.get_subscriptions("topic1").await;
        assert_eq!(subs.len(), 0);
    }
}
