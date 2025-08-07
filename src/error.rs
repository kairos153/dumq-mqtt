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
}

 