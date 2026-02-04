//! Error types for Samsung TV operations.

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for Samsung TV operations.
pub type Result<T> = std::result::Result<T, SamsungTvError>;

/// Errors that can occur when interacting with a Samsung TV.
#[derive(Error, Debug)]
pub enum SamsungTvError {
    /// Failed to establish a connection to the TV.
    #[error("failed to connect to TV at {host}:{port}: {source}")]
    ConnectionFailed {
        host: String,
        port: u16,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// TLS/SSL error during secure connection.
    #[error("TLS error: {0}")]
    TlsError(#[from] native_tls::Error),

    /// TV requires authentication but no token was provided.
    #[error("authentication required - connect without token first to receive one from the TV")]
    AuthenticationRequired,

    /// Authentication with the provided token failed.
    #[error("authentication failed - token may be invalid or expired")]
    AuthenticationFailed,

    /// WebSocket protocol error.
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    /// Command execution failed.
    #[error("command '{command}' failed: {reason}")]
    CommandFailed { command: String, reason: String },

    /// Operation timed out.
    #[error("operation timed out")]
    Timeout,

    /// Invalid response received from TV.
    #[error("invalid response from TV: {message}")]
    InvalidResponse { message: String },

    /// Failed to load token from file.
    #[error("failed to load token from {path}: {source}")]
    TokenLoadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to save token to file.
    #[error("failed to save token to {path}: {source}")]
    TokenSaveFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// URL parsing error.
    #[error("invalid URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// HTTP request error.
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Connection is not established.
    #[error("not connected to TV")]
    NotConnected,

    /// App not found on the TV.
    #[error("app '{app_id}' not found")]
    AppNotFound { app_id: String },
}
