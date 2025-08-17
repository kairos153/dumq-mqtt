use crate::types::Message;

/// Message handler function type
pub type MessageHandler = Box<dyn Fn(Message) + Send + Sync>;

/// Message handler trait for more flexible message processing
pub trait MessageProcessor: Send + Sync {
    /// Process a received message
    fn process(&self, message: Message);
}

impl<F> MessageProcessor for F
where
    F: Fn(Message) + Send + Sync,
{
    fn process(&self, message: Message) {
        self(message);
    }
}

/// Default message handler that logs messages
pub struct DefaultMessageHandler;

impl MessageProcessor for DefaultMessageHandler {
    fn process(&self, message: Message) {
        log::info!(
            "Received message on topic '{}' with QoS {:?}",
            message.topic,
            message.qos
        );
        
        if let Ok(payload_str) = String::from_utf8(message.payload.to_vec()) {
            log::debug!("Message payload: {}", payload_str);
        } else {
            log::debug!("Message payload: {:?} bytes", message.payload.len());
        }
    }
}

/// Message handler that can be enabled/disabled
pub struct ToggleableMessageHandler {
    enabled: bool,
    inner: Box<dyn MessageProcessor>,
}

impl ToggleableMessageHandler {
    /// Create a new toggleable message handler
    pub fn new(inner: Box<dyn MessageProcessor>) -> Self {
        Self {
            enabled: true,
            inner,
        }
    }

    /// Enable the message handler
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the message handler
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if the handler is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set the inner message processor
    pub fn set_processor(&mut self, processor: Box<dyn MessageProcessor>) {
        self.inner = processor;
    }
}

impl MessageProcessor for ToggleableMessageHandler {
    fn process(&self, message: Message) {
        if self.enabled {
            self.inner.process(message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Message;
use crate::protocol::QoS;
    use bytes::Bytes;

    #[test]
    fn test_default_message_handler() {
        let handler = DefaultMessageHandler;
        let message = Message {
            topic: "test/topic".to_string(),
            payload: Bytes::from("test message"),
            qos: QoS::AtLeastOnce as u8,
            retain: false,
            dup: false,
            packet_id: Some(1),
        };

        // Should not panic
        handler.process(message);
    }

    #[test]
    fn test_toggleable_message_handler() {
        let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
        let counter_clone = counter.clone();
        let inner_handler = Box::new(move |_msg: Message| {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
        });

        let mut handler = ToggleableMessageHandler::new(inner_handler);
        let message = Message {
            topic: "test/topic".to_string(),
            payload: Bytes::from("test message"),
            qos: QoS::AtMostOnce as u8,
            retain: false,
            dup: false,
            packet_id: None,
        };

        // Initially enabled
        assert!(handler.is_enabled());
        handler.process(message.clone());
        assert_eq!(*counter.lock().unwrap(), 1);

        // Disable
        handler.disable();
        assert!(!handler.is_enabled());
        handler.process(message.clone());
        assert_eq!(*counter.lock().unwrap(), 1); // Should not increment

        // Re-enable
        handler.enable();
        assert!(handler.is_enabled());
        handler.process(message);
        assert_eq!(*counter.lock().unwrap(), 2);
    }

    #[test]
    fn test_message_processor_trait() {
        let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
        let counter_clone = counter.clone();
        let processor: Box<dyn MessageProcessor> = Box::new(move |_msg: Message| {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
        });

        let message = Message {
            topic: "test/topic".to_string(),
            payload: Bytes::from("test message"),
            qos: QoS::ExactlyOnce as u8,
            retain: false,
            dup: false,
            packet_id: Some(1),
        };

        processor.process(message);
        assert_eq!(*counter.lock().unwrap(), 1);
    }
}
