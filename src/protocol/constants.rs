//! MQTT protocol constants
//! 
//! This module defines the core constants used throughout the MQTT protocol
//! including protocol names, versions, limits, and default values.

use std::time::Duration;

/// MQTT protocol constants
pub const MQTT_PROTOCOL_NAME_V3_1: &str = "MQIsdp";
pub const MQTT_PROTOCOL_NAME_V3_1_1: &str = "MQTT";
pub const MQTT_PROTOCOL_NAME_V5_0: &str = "MQTT";

pub const MQTT_PROTOCOL_VERSION_V3_1: u8 = 3;
pub const MQTT_PROTOCOL_VERSION_V3_1_1: u8 = 4;
pub const MQTT_PROTOCOL_VERSION_V5_0: u8 = 5;

/// Maximum packet size
pub const MAX_PACKET_SIZE: usize = 268_435_455; // 256MB

/// Maximum topic length
pub const MAX_TOPIC_LENGTH: usize = 65535;

/// Maximum client ID length
pub const MAX_CLIENT_ID_LENGTH: usize = 23;

/// Default keep alive interval (60 seconds)
pub const DEFAULT_KEEP_ALIVE: u16 = 60;

/// Maximum keep alive interval (18 hours)
pub const MAX_KEEP_ALIVE: u16 = 65535;

/// Default connection timeout
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

/// Default read timeout
pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(30);

/// Default write timeout
pub const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(30);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_constants() {
        assert_eq!(MQTT_PROTOCOL_NAME_V3_1, "MQIsdp");
        assert_eq!(MQTT_PROTOCOL_NAME_V3_1_1, "MQTT");
        assert_eq!(MQTT_PROTOCOL_NAME_V5_0, "MQTT");
        
        assert_eq!(MQTT_PROTOCOL_VERSION_V3_1, 3);
        assert_eq!(MQTT_PROTOCOL_VERSION_V3_1_1, 4);
        assert_eq!(MQTT_PROTOCOL_VERSION_V5_0, 5);
        
        assert_eq!(MAX_PACKET_SIZE, 268_435_455);
        assert_eq!(MAX_TOPIC_LENGTH, 65535);
        assert_eq!(MAX_CLIENT_ID_LENGTH, 23);
        assert_eq!(DEFAULT_KEEP_ALIVE, 60);
        assert_eq!(MAX_KEEP_ALIVE, 65535);
        
        assert_eq!(DEFAULT_CONNECT_TIMEOUT, Duration::from_secs(30));
        assert_eq!(DEFAULT_READ_TIMEOUT, Duration::from_secs(30));
        assert_eq!(DEFAULT_WRITE_TIMEOUT, Duration::from_secs(30));
    }
}
