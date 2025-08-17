//! Server configuration module

use super::auth::Authentication;

/// MQTT server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub max_connections: usize,
    pub max_packet_size: usize,
    pub protocol_version: u8,
    pub allow_anonymous: bool,
    pub authentication: Option<Authentication>,
}

impl ServerConfig {
    pub fn new(bind_addr: impl Into<String>) -> Self {
        Self {
            bind_addr: bind_addr.into(),
            max_connections: 1000,
            max_packet_size: 1024 * 1024, // 1MB
            protocol_version: 4, // MQTT 3.1.1
            allow_anonymous: true,
            authentication: None,
        }
    }

    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    pub fn max_packet_size(mut self, size: usize) -> Self {
        self.max_packet_size = size;
        self
    }

    pub fn protocol_version(mut self, version: u8) -> Self {
        self.protocol_version = version;
        self
    }

    pub fn allow_anonymous(mut self, allow: bool) -> Self {
        self.allow_anonymous = allow;
        self
    }

    pub fn authentication(mut self, auth: Authentication) -> Self {
        self.authentication = Some(auth);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_new() {
        let config = ServerConfig::new("127.0.0.1:1883");
        assert_eq!(config.bind_addr, "127.0.0.1:1883");
        assert_eq!(config.max_connections, 1000);
        assert_eq!(config.max_packet_size, 1024 * 1024);
        assert_eq!(config.protocol_version, 4);
        assert_eq!(config.allow_anonymous, true);
        assert!(config.authentication.is_none());
    }

    #[test]
    fn test_server_config_builder_pattern() {
        let config = ServerConfig::new("localhost:1883")
            .max_connections(500)
            .max_packet_size(512 * 1024)
            .protocol_version(5)
            .allow_anonymous(false);

        assert_eq!(config.max_connections, 500);
        assert_eq!(config.max_packet_size, 512 * 1024);
        assert_eq!(config.protocol_version, 5);
        assert_eq!(config.allow_anonymous, false);
    }

    #[test]
    fn test_server_config_clone() {
        let config1 = ServerConfig::new("127.0.0.1:1883");
        let config2 = config1.clone();
        assert_eq!(config1.bind_addr, config2.bind_addr);
        assert_eq!(config1.max_connections, config2.max_connections);
    }
}
