//! Token-based authentication for Samsung TVs.
//!
//! Samsung TVs (2018+) require token authentication for secure WebSocket connections.
//! The flow is:
//! 1. Connect without a token -> TV shows "Allow connection?" prompt
//! 2. User accepts on TV -> TV sends response containing a token
//! 3. Save the token for future use
//! 4. Future connections use the saved token -> No prompt needed

use std::fs;
use std::path::PathBuf;

use crate::error::{Result, SamsungTvError};

/// Manages authentication tokens for Samsung TV connections.
#[derive(Debug, Default)]
pub struct TokenManager {
    /// Current token in memory.
    token: Option<String>,
    /// Path to persist the token.
    token_file: Option<PathBuf>,
}

impl TokenManager {
    /// Creates a new token manager.
    pub fn new(token_file: Option<PathBuf>) -> Self {
        Self {
            token: None,
            token_file,
        }
    }

    /// Creates a token manager with an initial token.
    pub fn with_token(token: String, token_file: Option<PathBuf>) -> Self {
        Self {
            token: Some(token),
            token_file,
        }
    }

    /// Returns the current token, if any.
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// Sets the current token.
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    /// Clears the current token.
    pub fn clear_token(&mut self) {
        self.token = None;
    }

    /// Loads the token from the configured file path.
    ///
    /// Returns `Ok(Some(token))` if a token was loaded,
    /// `Ok(None)` if no file exists or no path is configured.
    pub fn load(&mut self) -> Result<Option<String>> {
        let Some(path) = &self.token_file else {
            return Ok(None);
        };

        if !path.exists() {
            return Ok(None);
        }

        let token = fs::read_to_string(path).map_err(|e| SamsungTvError::TokenLoadFailed {
            path: path.clone(),
            source: e,
        })?;

        let token = token.trim().to_string();
        if token.is_empty() {
            return Ok(None);
        }

        self.token = Some(token.clone());
        Ok(Some(token))
    }

    /// Saves the current token to the configured file path.
    ///
    /// Does nothing if no token is set or no path is configured.
    pub fn save(&self) -> Result<()> {
        let Some(token) = &self.token else {
            return Ok(());
        };

        let Some(path) = &self.token_file else {
            return Ok(());
        };

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| SamsungTvError::TokenSaveFailed {
                path: path.clone(),
                source: e,
            })?;
        }

        fs::write(path, token).map_err(|e| SamsungTvError::TokenSaveFailed {
            path: path.clone(),
            source: e,
        })?;

        Ok(())
    }

    /// Extracts a token from a Samsung TV WebSocket response.
    ///
    /// The TV sends a response like:
    /// ```json
    /// {
    ///   "event": "ms.channel.connect",
    ///   "data": {
    ///     "token": "12345678"
    ///   }
    /// }
    /// ```
    pub fn extract_from_response(response: &str) -> Option<String> {
        let json: serde_json::Value = serde_json::from_str(response).ok()?;

        // Check if this is a connect event
        let event = json.get("event")?.as_str()?;
        if event != "ms.channel.connect" {
            return None;
        }

        // Extract the token from data
        let data = json.get("data")?;
        let token = data.get("token")?.as_str()?;

        if token.is_empty() {
            return None;
        }

        Some(token.to_string())
    }

    /// Checks if the response indicates an authentication error.
    pub fn is_auth_error(response: &str) -> bool {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(response) {
            if let Some(event) = json.get("event").and_then(|v| v.as_str()) {
                return event == "ms.channel.unauthorized"
                    || event == "ms.error"
                    || event.contains("unauthorized");
            }
        }
        false
    }

    /// Checks if the response indicates a successful connection.
    pub fn is_connected(response: &str) -> bool {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(response) {
            if let Some(event) = json.get("event").and_then(|v| v.as_str()) {
                return event == "ms.channel.connect";
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_token_from_response() {
        let response = r#"{
            "event": "ms.channel.connect",
            "data": {
                "token": "12345678"
            }
        }"#;

        let token = TokenManager::extract_from_response(response);
        assert_eq!(token, Some("12345678".to_string()));
    }

    #[test]
    fn test_extract_token_wrong_event() {
        let response = r#"{
            "event": "ms.channel.clientConnect",
            "data": {}
        }"#;

        let token = TokenManager::extract_from_response(response);
        assert_eq!(token, None);
    }

    #[test]
    fn test_is_auth_error() {
        let error_response = r#"{"event": "ms.channel.unauthorized"}"#;
        assert!(TokenManager::is_auth_error(error_response));

        let success_response = r#"{"event": "ms.channel.connect"}"#;
        assert!(!TokenManager::is_auth_error(success_response));
    }

    #[test]
    fn test_is_connected() {
        let response = r#"{"event": "ms.channel.connect"}"#;
        assert!(TokenManager::is_connected(response));

        let other = r#"{"event": "ms.channel.ready"}"#;
        assert!(!TokenManager::is_connected(other));
    }

    #[test]
    fn test_token_manager_memory() {
        let mut manager = TokenManager::new(None);
        assert!(manager.token().is_none());

        manager.set_token("test_token".to_string());
        assert_eq!(manager.token(), Some("test_token"));

        manager.clear_token();
        assert!(manager.token().is_none());
    }
}
