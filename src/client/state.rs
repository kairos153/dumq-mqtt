/// MQTT client connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
}

impl ConnectionState {
    /// Check if the client is connected
    pub fn is_connected(&self) -> bool {
        matches!(self, ConnectionState::Connected)
    }

    /// Check if the client is disconnected
    pub fn is_disconnected(&self) -> bool {
        matches!(self, ConnectionState::Disconnected)
    }

    /// Check if the client is in a transitional state
    pub fn is_transitional(&self) -> bool {
        matches!(self, ConnectionState::Connecting | ConnectionState::Disconnecting)
    }

    /// Get a human-readable description of the state
    pub fn description(&self) -> &'static str {
        match self {
            ConnectionState::Disconnected => "Disconnected",
            ConnectionState::Connecting => "Connecting",
            ConnectionState::Connected => "Connected",
            ConnectionState::Disconnecting => "Disconnecting",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_state_values() {
        assert_eq!(ConnectionState::Disconnected as u8, 0);
        assert_eq!(ConnectionState::Connecting as u8, 1);
        assert_eq!(ConnectionState::Connected as u8, 2);
        assert_eq!(ConnectionState::Disconnecting as u8, 3);
    }

    #[test]
    fn test_connection_state_clone() {
        let state1 = ConnectionState::Connected;
        let state2 = state1.clone();
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_connection_state_partial_eq() {
        assert_ne!(ConnectionState::Disconnected, ConnectionState::Connected);
        assert_eq!(ConnectionState::Connecting, ConnectionState::Connecting);
        assert_ne!(ConnectionState::Connected, ConnectionState::Disconnecting);
    }

    #[test]
    fn test_connection_state_methods() {
        let disconnected = ConnectionState::Disconnected;
        let connecting = ConnectionState::Connecting;
        let connected = ConnectionState::Connected;
        let disconnecting = ConnectionState::Disconnecting;

        assert!(disconnected.is_disconnected());
        assert!(!disconnected.is_connected());
        assert!(!disconnected.is_transitional());

        assert!(connecting.is_transitional());
        assert!(!connecting.is_connected());
        assert!(!connecting.is_disconnected());

        assert!(connected.is_connected());
        assert!(!connected.is_disconnected());
        assert!(!connected.is_transitional());

        assert!(disconnecting.is_transitional());
        assert!(!disconnecting.is_connected());
        assert!(!disconnecting.is_disconnected());
    }

    #[test]
    fn test_connection_state_description() {
        assert_eq!(ConnectionState::Disconnected.description(), "Disconnected");
        assert_eq!(ConnectionState::Connecting.description(), "Connecting");
        assert_eq!(ConnectionState::Connected.description(), "Connected");
        assert_eq!(ConnectionState::Disconnecting.description(), "Disconnecting");
    }
}
