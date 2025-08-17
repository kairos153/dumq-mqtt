//! Protocol flags for MQTT messages
//! 
//! This module defines the core protocol flags used in MQTT message handling:
//! 
//! - **`RetainFlag`**: Controls message retention on the broker
//! - **`DupFlag`**: Indicates duplicate message delivery attempts

/// Retain flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetainFlag {
    NotRetained = 0,
    Retained = 1,
}

/// Duplicate flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DupFlag {
    NotDuplicate = 0,
    Duplicate = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retain_flag() {
        assert_eq!(RetainFlag::NotRetained as u8, 0);
        assert_eq!(RetainFlag::Retained as u8, 1);
    }

    #[test]
    fn test_dup_flag() {
        assert_eq!(DupFlag::NotDuplicate as u8, 0);
        assert_eq!(DupFlag::Duplicate as u8, 1);
    }
}
