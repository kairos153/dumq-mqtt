use thiserror::Error;

/// MQTT library error types
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Invalid packet: {0}")]
    InvalidPacket(String),
    
    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(u8),
    
    #[error("Invalid QoS level: {0}")]
    InvalidQoS(u8),
    
    #[error("Invalid topic: {0}")]
    InvalidTopic(String),
    
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Authorization failed: {0}")]
    Authorization(String),
    
    #[error("Server error: {0}")]
    Server(String),
    
    #[error("Client error: {0}")]
    Client(String),
    
    #[error("Timeout")]
    Timeout,
    
    #[error("Disconnected")]
    Disconnected,
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

/// Result type for MQTT operations
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let connection_error = Error::Connection("Failed to connect".to_string());
        assert_eq!(connection_error.to_string(), "Connection error: Failed to connect");
        
        let protocol_error = Error::Protocol("Invalid packet".to_string());
        assert_eq!(protocol_error.to_string(), "Protocol error: Invalid packet");
        
        let timeout_error = Error::Timeout;
        assert_eq!(timeout_error.to_string(), "Timeout");
        
        let disconnected_error = Error::Disconnected;
        assert_eq!(disconnected_error.to_string(), "Disconnected");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection refused");
        let mqtt_error: Error = io_error.into();
        
        match mqtt_error {
            Error::Io(_) => (), // Expected
            _ => panic!("Expected Io error"),
        }
    }

    #[test]
    fn test_unsupported_version_error() {
        let error = Error::UnsupportedVersion(99);
        assert_eq!(error.to_string(), "Unsupported protocol version: 99");
    }

    #[test]
    fn test_invalid_qos_error() {
        let error = Error::InvalidQoS(5);
        assert_eq!(error.to_string(), "Invalid QoS level: 5");
    }

    #[test]
    fn test_all_error_variants() {
        // Test all error variants
        let errors = vec![
            Error::Connection("Connection failed".to_string()),
            Error::Protocol("Protocol violation".to_string()),
            Error::InvalidPacket("Malformed packet".to_string()),
            Error::UnsupportedVersion(3),
            Error::InvalidQoS(3),
            Error::InvalidTopic("Invalid topic format".to_string()),
            Error::Authentication("Invalid credentials".to_string()),
            Error::Authorization("Access denied".to_string()),
            Error::Server("Internal server error".to_string()),
            Error::Client("Client configuration error".to_string()),
            Error::Timeout,
            Error::Disconnected,
            Error::Serialization("Failed to serialize".to_string()),
            Error::Deserialization("Failed to deserialize".to_string()),
        ];

        assert_eq!(errors.len(), 14); // Total number of error variants
    }

    #[test]
    fn test_error_with_empty_strings() {
        let connection_error = Error::Connection("".to_string());
        assert_eq!(connection_error.to_string(), "Connection error: ");
        
        let protocol_error = Error::Protocol("".to_string());
        assert_eq!(protocol_error.to_string(), "Protocol error: ");
        
        let invalid_packet_error = Error::InvalidPacket("".to_string());
        assert_eq!(invalid_packet_error.to_string(), "Invalid packet: ");
    }

    #[test]
    fn test_error_with_special_characters() {
        let connection_error = Error::Connection("Error with special chars: !@#$%^&*()".to_string());
        assert_eq!(connection_error.to_string(), "Connection error: Error with special chars: !@#$%^&*()");
        
        let topic_error = Error::InvalidTopic("Topic with spaces and symbols: test/topic/+".to_string());
        assert_eq!(topic_error.to_string(), "Invalid topic: Topic with spaces and symbols: test/topic/+");
    }

    #[test]
    fn test_error_with_unicode() {
        let connection_error = Error::Connection("한국어 에러 메시지".to_string());
        assert_eq!(connection_error.to_string(), "Connection error: 한국어 에러 메시지");
        
        let protocol_error = Error::Protocol("Unicode protocol error: 🚀".to_string());
        assert_eq!(protocol_error.to_string(), "Protocol error: Unicode protocol error: 🚀");
    }

    #[test]
    fn test_error_edge_cases() {
        // Test with very long error messages
        let long_message = "A".repeat(1000);
        let connection_error = Error::Connection(long_message.clone());
        assert_eq!(connection_error.to_string(), format!("Connection error: {}", long_message));
        
        // Test with maximum u8 values
        let max_version_error = Error::UnsupportedVersion(u8::MAX);
        assert_eq!(max_version_error.to_string(), format!("Unsupported protocol version: {}", u8::MAX));
        
        let max_qos_error = Error::InvalidQoS(u8::MAX);
        assert_eq!(max_qos_error.to_string(), format!("Invalid QoS level: {}", u8::MAX));
    }

    #[test]
    fn test_error_debug_format() {
        let connection_error = Error::Connection("Debug test".to_string());
        let debug_output = format!("{:?}", connection_error);
        assert!(debug_output.contains("Connection"));
        assert!(debug_output.contains("Debug test"));
        
        let timeout_error = Error::Timeout;
        let debug_output = format!("{:?}", timeout_error);
        assert!(debug_output.contains("Timeout"));
    }

    #[test]
    fn test_error_clone() {
        let original_error = Error::Connection("Original message".to_string());
        let cloned_error = match &original_error {
            Error::Connection(msg) => Error::Connection(msg.clone()),
            _ => panic!("Expected Connection error"),
        };
        
        assert_eq!(original_error.to_string(), cloned_error.to_string());
    }

    #[test]
    fn test_result_type() {
        // Test successful result
        let success_result: Result<String> = Ok("Success".to_string());
        assert!(success_result.is_ok());
        assert_eq!(success_result.unwrap(), "Success");
        
        // Test error result
        let error_result: Result<String> = Err(Error::Timeout);
        assert!(error_result.is_err());
        match error_result {
            Err(Error::Timeout) => (), // Expected
            _ => panic!("Expected Timeout error"),
        }
    }

    #[test]
    fn test_error_conversion_chains() {
        // Test chaining of error conversions
        let io_error = std::io::Error::new(std::io::ErrorKind::TimedOut, "Operation timed out");
        let mqtt_error: Error = io_error.into();
        
        // Convert back to string and verify
        let error_string = mqtt_error.to_string();
        assert!(error_string.contains("IO error"));
    }

    #[test]
    fn test_error_message_formatting() {
        // Test that error messages are properly formatted
        let errors = vec![
            (Error::Connection("test".to_string()), "Connection error: test"),
            (Error::Protocol("test".to_string()), "Protocol error: test"),
            (Error::InvalidPacket("test".to_string()), "Invalid packet: test"),
            (Error::UnsupportedVersion(1), "Unsupported protocol version: 1"),
            (Error::InvalidQoS(1), "Invalid QoS level: 1"),
            (Error::InvalidTopic("test".to_string()), "Invalid topic: test"),
            (Error::Authentication("test".to_string()), "Authentication failed: test"),
            (Error::Authorization("test".to_string()), "Authorization failed: test"),
            (Error::Server("test".to_string()), "Server error: test"),
            (Error::Client("test".to_string()), "Client error: test"),
            (Error::Serialization("test".to_string()), "Serialization error: test"),
            (Error::Deserialization("test".to_string()), "Deserialization error: test"),
        ];

        for (error, expected) in errors {
            assert_eq!(error.to_string(), expected);
        }
    }
}

 