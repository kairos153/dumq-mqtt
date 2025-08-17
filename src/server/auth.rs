//! Authentication module

use std::collections::HashMap;

/// Authentication configuration
#[derive(Debug, Clone, Default)]
pub struct Authentication {
    pub users: HashMap<String, String>, // username -> password
}

impl Authentication {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_user(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.users.insert(username.into(), password.into());
        self
    }

    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        if let Some(expected_password) = self.users.get(username) {
            expected_password == password
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_new() {
        let auth = Authentication::new();
        assert!(auth.users.is_empty());
    }

    #[test]
    fn test_authentication_add_user() {
        let auth = Authentication::new()
            .add_user("user1", "pass1")
            .add_user("user2", "pass2");

        assert_eq!(auth.users.len(), 2);
        assert_eq!(auth.users.get("user1"), Some(&"pass1".to_string()));
        assert_eq!(auth.users.get("user2"), Some(&"pass2".to_string()));
    }

    #[test]
    fn test_authentication_authenticate() {
        let auth = Authentication::new()
            .add_user("user1", "pass1");

        assert!(auth.authenticate("user1", "pass1"));
        assert!(!auth.authenticate("user1", "wrong_pass"));
        assert!(!auth.authenticate("unknown_user", "pass1"));
    }

    #[test]
    fn test_authentication_clone() {
        let auth1 = Authentication::new().add_user("user1", "pass1");
        let auth2 = auth1.clone();
        assert_eq!(auth1.users.len(), auth2.users.len());
        assert_eq!(auth1.authenticate("user1", "pass1"), auth2.authenticate("user1", "pass1"));
    }
}
