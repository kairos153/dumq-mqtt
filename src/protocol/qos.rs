//! Quality of Service (QoS) levels for MQTT protocol
//! 
//! MQTT defines three QoS levels that determine the reliability of message delivery:
//! 
//! - **`QoS::AtMostOnce` (0)**: Fire and forget - no acknowledgment
//! - **`QoS::AtLeastOnce` (1)**: At least once delivery with acknowledgment
//! - **`QoS::ExactlyOnce` (2)**: Exactly once delivery with two-phase acknowledgment

/// Quality of Service levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

impl QoS {
    /// Create QoS from u8 value
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(QoS::AtMostOnce),
            1 => Some(QoS::AtLeastOnce),
            2 => Some(QoS::ExactlyOnce),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qos_from_u8() {
        assert_eq!(QoS::from_u8(0), Some(QoS::AtMostOnce));
        assert_eq!(QoS::from_u8(1), Some(QoS::AtLeastOnce));
        assert_eq!(QoS::from_u8(2), Some(QoS::ExactlyOnce));
        assert_eq!(QoS::from_u8(3), None);
        assert_eq!(QoS::from_u8(255), None);
    }

    #[test]
    fn test_qos_values() {
        assert_eq!(QoS::AtMostOnce as u8, 0);
        assert_eq!(QoS::AtLeastOnce as u8, 1);
        assert_eq!(QoS::ExactlyOnce as u8, 2);
    }

    #[test]
    fn test_qos_clone() {
        let qos = QoS::AtLeastOnce;
        let cloned_qos = qos;
        assert_eq!(qos, cloned_qos);
    }
}
