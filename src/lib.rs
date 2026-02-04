//! Samsung TV RS - A Rust library for controlling Samsung Smart TVs.
#![allow(clippy::result_large_err)]
//!
//! This library provides an async API for interacting with Samsung Smart TVs
//! via their WebSocket remote control protocol.
//!
//! # Features
//!
//! - **Remote Control**: Send key presses (power, volume, navigation, etc.)
//! - **App Control**: Launch apps like Netflix, YouTube, etc.
//! - **TLS Support**: Secure connections for newer TVs (2018+)
//! - **Token Authentication**: Automatic token handling and persistence
//! - **Async**: Built on tokio for efficient async I/O
//!
//! # Quick Start
//!
//! ```no_run
//! use samsung_tv::{SamsungTV, SamsungTvConfig, Key};
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Configure connection
//!     let config = SamsungTvConfig::new("192.168.1.100")
//!         .secure()  // Use TLS (port 8002)
//!         .name("My Remote")
//!         .token_file(PathBuf::from("/home/user/.samsung_token"))
//!         .build();
//!
//!     // Connect to TV
//!     let mut tv = SamsungTV::connect(config).await?;
//!
//!     // Send commands
//!     tv.volume_up().await?;
//!     tv.send_key(Key::Home).await?;
//!
//!     // Launch an app
//!     tv.launch_app(samsung_tv::app_ids::NETFLIX).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Authentication
//!
//! Newer Samsung TVs (2018+) require authentication:
//!
//! 1. On first connection, the TV will display an "Allow connection?" prompt
//! 2. Accept the prompt on the TV
//! 3. The library receives and saves the authentication token
//! 4. Future connections use the saved token automatically
//!
//! Use `token_file()` in the config to persist tokens between sessions.

mod apps;
mod auth;
mod client;
mod commands;
mod config;
mod connection;
pub mod discovery;
mod error;

// Primary exports
pub use client::{DeviceInfo, SamsungTV, TvInfo};
pub use commands::{CommandType, Key, RemoteCommand};
pub use config::{SamsungTvConfig, DEFAULT_WSS_PORT, DEFAULT_WS_PORT};
pub use error::{Result, SamsungTvError};

// App-related exports
pub use apps::{app_ids, App};

// Re-export for token management
pub use auth::TokenManager;

// === Backward Compatibility ===

#[allow(deprecated)]
pub use commands::{Command, Commands};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_api_available() {
        // Ensure main types are accessible
        let _ = SamsungTvConfig::new("192.168.1.1");
        let _ = Key::Power;
        let _ = RemoteCommand::click(Key::VolumeUp);
    }

    #[test]
    fn test_key_convenience() {
        // Keys should be usable directly
        let key = Key::VolumeUp;
        assert_eq!(key.as_str(), "KEY_VOLUP");
    }

    #[allow(deprecated)]
    #[test]
    fn test_backward_compat() {
        // Legacy API should still work
        let _ = Commands::KEY_POWER;
        assert_eq!(Command::KeyPower.as_str(), "KEY_POWER");
    }
}
