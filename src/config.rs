//! Configuration builder for Samsung TV connections.

use std::path::PathBuf;
use std::time::Duration;

/// Default port for insecure WebSocket connections.
pub const DEFAULT_WS_PORT: u16 = 8001;

/// Default port for secure WebSocket connections (TLS).
pub const DEFAULT_WSS_PORT: u16 = 8002;

/// Configuration for connecting to a Samsung TV.
#[derive(Debug, Clone)]
pub struct SamsungTvConfig {
    /// TV hostname or IP address.
    pub host: String,
    /// WebSocket port (8001 for ws://, 8002 for wss://).
    pub port: u16,
    /// Client name displayed on the TV during pairing.
    pub name: String,
    /// Whether to use TLS (wss://).
    pub use_tls: bool,
    /// Authentication token from previous connection.
    pub token: Option<String>,
    /// Path to file for persisting the authentication token.
    pub token_file: Option<PathBuf>,
    /// Connection and operation timeout.
    pub timeout: Duration,
    /// Delay between key presses when sending multiple keys.
    pub key_delay: Duration,
    /// Whether to automatically reconnect on connection loss.
    pub auto_reconnect: bool,
}

impl SamsungTvConfig {
    /// Creates a new configuration with the specified host.
    ///
    /// Uses default values:
    /// - Port: 8001 (insecure)
    /// - Name: "Samsung TV RS"
    /// - TLS: disabled
    /// - Timeout: 5 seconds
    /// - Key delay: 300ms
    /// - Auto reconnect: enabled
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            port: DEFAULT_WS_PORT,
            name: "Samsung TV RS".to_string(),
            use_tls: false,
            token: None,
            token_file: None,
            timeout: Duration::from_secs(5),
            key_delay: Duration::from_millis(300),
            auto_reconnect: true,
        }
    }

    /// Sets the WebSocket port.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the client name displayed on the TV.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the authentication token.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Sets the path for token persistence.
    pub fn token_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.token_file = Some(path.into());
        self
    }

    /// Enables TLS and sets the port to 8002.
    ///
    /// This is required for newer Samsung TVs (2018+).
    pub fn secure(mut self) -> Self {
        self.use_tls = true;
        self.port = DEFAULT_WSS_PORT;
        self
    }

    /// Sets the connection timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the delay between consecutive key presses.
    pub fn key_delay(mut self, delay: Duration) -> Self {
        self.key_delay = delay;
        self
    }

    /// Enables or disables automatic reconnection.
    pub fn auto_reconnect(mut self, enabled: bool) -> Self {
        self.auto_reconnect = enabled;
        self
    }

    /// Finalizes the configuration.
    ///
    /// This is a no-op that allows for a consistent builder pattern.
    pub fn build(self) -> Self {
        self
    }

    /// Returns the WebSocket URL scheme based on TLS setting.
    pub(crate) fn scheme(&self) -> &'static str {
        if self.use_tls {
            "wss"
        } else {
            "ws"
        }
    }
}

impl Default for SamsungTvConfig {
    fn default() -> Self {
        Self::new("192.168.1.1")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SamsungTvConfig::new("192.168.1.100");
        assert_eq!(config.host, "192.168.1.100");
        assert_eq!(config.port, 8001);
        assert!(!config.use_tls);
        assert_eq!(config.scheme(), "ws");
    }

    #[test]
    fn test_secure_config() {
        let config = SamsungTvConfig::new("192.168.1.100").secure();
        assert_eq!(config.port, 8002);
        assert!(config.use_tls);
        assert_eq!(config.scheme(), "wss");
    }

    #[test]
    fn test_builder_chain() {
        let config = SamsungTvConfig::new("tv.local")
            .secure()
            .name("My Remote")
            .token("abc123")
            .timeout(Duration::from_secs(10))
            .build();

        assert_eq!(config.host, "tv.local");
        assert_eq!(config.name, "My Remote");
        assert_eq!(config.token, Some("abc123".to_string()));
        assert_eq!(config.timeout, Duration::from_secs(10));
    }
}
