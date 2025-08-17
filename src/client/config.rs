use std::time::Duration;

/// MQTT client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub server_addr: String,
    pub connect_timeout: Duration,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub keep_alive_interval: Duration,
    pub max_packet_size: usize,
    pub protocol_version: u8,
}

impl ClientConfig {
    /// Create a new client configuration with default values
    pub fn new(server_addr: impl Into<String>) -> Self {
        Self {
            server_addr: server_addr.into(),
            connect_timeout: Duration::from_secs(30),
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            keep_alive_interval: Duration::from_secs(60),
            max_packet_size: 1024 * 1024, // 1MB
            protocol_version: 4, // MQTT 3.1.1
        }
    }

    /// Set connection timeout
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set read timeout
    pub fn read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = timeout;
        self
    }

    /// Set write timeout
    pub fn write_timeout(mut self, timeout: Duration) -> Self {
        self.write_timeout = timeout;
        self
    }

    /// Set keep-alive interval
    pub fn keep_alive_interval(mut self, interval: Duration) -> Self {
        self.keep_alive_interval = interval;
        self
    }

    /// Set maximum packet size
    pub fn max_packet_size(mut self, size: usize) -> Self {
        self.max_packet_size = size;
        self
    }

    /// Set protocol version
    pub fn protocol_version(mut self, version: u8) -> Self {
        self.protocol_version = version;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config_new() {
        let config = ClientConfig::new("localhost:1883");
        assert_eq!(config.server_addr, "localhost:1883");
        assert_eq!(config.connect_timeout, Duration::from_secs(30));
        assert_eq!(config.read_timeout, Duration::from_secs(30));
        assert_eq!(config.write_timeout, Duration::from_secs(30));
        assert_eq!(config.keep_alive_interval, Duration::from_secs(60));
        assert_eq!(config.max_packet_size, 1024 * 1024);
        assert_eq!(config.protocol_version, 4);
    }

    #[test]
    fn test_client_config_builder_pattern() {
        let config = ClientConfig::new("mqtt.example.com:8883")
            .connect_timeout(Duration::from_secs(60))
            .read_timeout(Duration::from_secs(45))
            .write_timeout(Duration::from_secs(45))
            .keep_alive_interval(Duration::from_secs(120))
            .max_packet_size(2 * 1024 * 1024)
            .protocol_version(5);

        assert_eq!(config.connect_timeout, Duration::from_secs(60));
        assert_eq!(config.read_timeout, Duration::from_secs(45));
        assert_eq!(config.write_timeout, Duration::from_secs(45));
        assert_eq!(config.keep_alive_interval, Duration::from_secs(120));
        assert_eq!(config.max_packet_size, 2 * 1024 * 1024);
        assert_eq!(config.protocol_version, 5);
    }

    #[test]
    fn test_client_config_clone() {
        let config1 = ClientConfig::new("localhost:1883");
        let config2 = config1.clone();
        assert_eq!(config1.server_addr, config2.server_addr);
        assert_eq!(config1.connect_timeout, config2.connect_timeout);
        assert_eq!(config1.max_packet_size, config2.max_packet_size);
    }
}
