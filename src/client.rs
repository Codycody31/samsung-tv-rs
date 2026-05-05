//! Main Samsung TV client.
//!
//! This module provides the primary API for controlling Samsung Smart TVs.

use std::collections::HashMap;
use std::time::Duration;

use serde::Deserialize;
use tokio::time::sleep;
use tracing::{debug, info};

use crate::apps::{App, AppLaunchPayload, AppListPayload, BrowserPayload};
use crate::auth::TokenManager;
use crate::commands::{Key, RemoteCommand};
use crate::config::SamsungTvConfig;
use crate::connection::Connection;
use crate::error::Result;

/// Device information from the TV's REST API.
#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
pub struct DeviceInfo {
    pub FrameTVSupport: Option<String>,
    pub GamePadSupport: Option<String>,
    pub ImeSyncedSupport: Option<String>,
    pub OS: Option<String>,
    pub TokenAuthSupport: Option<String>,
    pub VoiceSupport: Option<String>,
    pub countryCode: Option<String>,
    pub description: Option<String>,
    pub developerIP: Option<String>,
    pub developerMode: Option<String>,
    pub duid: Option<String>,
    pub firmwareVersion: Option<String>,
    pub id: Option<String>,
    pub ip: Option<String>,
    pub model: Option<String>,
    pub modelName: Option<String>,
    pub name: Option<String>,
    pub networkType: Option<String>,
    pub resolution: Option<String>,
    pub smartHubAgreement: Option<String>,
    pub ssid: Option<String>,
    #[serde(rename = "type")]
    pub device_type: Option<String>,
    pub udn: Option<String>,
    pub wifiMac: Option<String>,
}

/// TV information response.
#[derive(Debug, Clone, Deserialize)]
pub struct TvInfo {
    pub device: DeviceInfo,
    pub id: String,
    #[serde(rename = "isSupport", deserialize_with = "deserialize_support_info")]
    pub is_support: HashMap<String, String>,
    pub name: String,
    pub remote: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub uri: String,
    pub version: String,
}

fn deserialize_support_info<'de, D>(
    deserializer: D,
) -> std::result::Result<HashMap<String, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    let map: HashMap<String, String> =
        serde_json::from_str(&s).map_err(serde::de::Error::custom)?;
    Ok(map)
}

/// Main client for controlling a Samsung Smart TV.
///
/// # Example
///
/// ```no_run
/// use samsung_tv::{SamsungTV, SamsungTvConfig, Key};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = SamsungTvConfig::new("192.168.1.100")
///         .secure()
///         .name("My Remote")
///         .build();
///
///     let mut tv = SamsungTV::connect(config).await?;
///
///     // Send commands
///     tv.volume_up().await?;
///     tv.send_key(Key::Home).await?;
///
///     Ok(())
/// }
/// ```
pub struct SamsungTV {
    connection: Connection,
    config: SamsungTvConfig,
    token_manager: TokenManager,
    http_client: reqwest::Client,
}

impl SamsungTV {
    /// Connects to a Samsung TV with the given configuration.
    ///
    /// If authentication is required and no token is provided, the TV will show
    /// a confirmation dialog. Accept it on the TV to receive a token.
    pub async fn connect(config: SamsungTvConfig) -> Result<Self> {
        // Try to load token from file if configured
        let mut token_manager = TokenManager::new(config.token_file.clone());

        let config = if config.token.is_none() {
            if let Ok(Some(saved_token)) = token_manager.load() {
                info!("Loaded saved token from file");
                SamsungTvConfig {
                    token: Some(saved_token),
                    ..config
                }
            } else {
                config
            }
        } else {
            config
        };

        let (connection, received_token) = Connection::connect(&config).await?;

        // Save new token if received
        if let Some(token) = received_token {
            info!("Received authentication token from TV");
            token_manager.set_token(token.clone());
            if let Err(e) = token_manager.save() {
                debug!("Failed to save token: {}", e);
            }
        }

        Ok(Self {
            connection,
            config,
            token_manager,
            http_client: reqwest::Client::new(),
        })
    }

    /// Disconnects from the TV gracefully.
    pub async fn disconnect(&mut self) -> Result<()> {
        self.connection.close().await
    }

    /// Returns whether the TV connection is still active.
    pub fn is_connected(&self) -> bool {
        self.connection.is_connected()
    }

    /// Returns the current authentication token, if any.
    pub fn token(&self) -> Option<&str> {
        self.token_manager.token()
    }

    /// Gets information about the TV via its REST API.
    ///
    /// This doesn't require a WebSocket connection.
    /// Note: The HTTP API is always on port 8001, regardless of WebSocket port.
    pub async fn get_info(&self) -> Result<TvInfo> {
        // HTTP API is always on port 8001 (not 8002 which is WebSocket-only)
        let url = format!("http://{}:8001/api/v2/", self.config.host);

        let response = self
            .http_client
            .get(&url)
            .timeout(self.config.timeout)
            .send()
            .await?
            .error_for_status()?
            .json::<TvInfo>()
            .await?;

        Ok(response)
    }

    // === Remote Control ===

    /// Sends a single key press to the TV.
    pub async fn send_key(&mut self, key: Key) -> Result<()> {
        let cmd = RemoteCommand::click(key);
        let payload = serde_json::to_string(&cmd.to_payload())?;

        debug!("Sending key: {}", key);
        self.connection.send(&payload).await?;

        // Wait for response
        let _ = self.connection.receive_timeout(self.config.timeout).await;

        Ok(())
    }

    /// Sends multiple key presses with a delay between each.
    pub async fn send_keys(&mut self, keys: &[Key]) -> Result<()> {
        for key in keys {
            self.send_key(*key).await?;
            sleep(self.config.key_delay).await;
        }
        Ok(())
    }

    /// Holds a key for the specified duration.
    ///
    /// Useful for volume/channel scrolling.
    pub async fn hold_key(&mut self, key: Key, duration: Duration) -> Result<()> {
        // Send press
        let press = RemoteCommand::press(key);
        self.connection
            .send(&serde_json::to_string(&press.to_payload())?)
            .await?;

        // Hold for duration
        sleep(duration).await;

        // Send release
        let release = RemoteCommand::release(key);
        self.connection
            .send(&serde_json::to_string(&release.to_payload())?)
            .await?;

        Ok(())
    }

    // === Convenience Methods ===

    /// Turns off the TV.
    pub async fn power_off(&mut self) -> Result<()> {
        self.send_key(Key::PowerOff).await
    }

    /// Toggles power state.
    pub async fn power_toggle(&mut self) -> Result<()> {
        self.send_key(Key::Power).await
    }

    /// Increases volume by one step.
    pub async fn volume_up(&mut self) -> Result<()> {
        self.send_key(Key::VolumeUp).await
    }

    /// Decreases volume by one step.
    pub async fn volume_down(&mut self) -> Result<()> {
        self.send_key(Key::VolumeDown).await
    }

    /// Toggles mute.
    pub async fn mute(&mut self) -> Result<()> {
        self.send_key(Key::Mute).await
    }

    /// Goes to the next channel.
    pub async fn channel_up(&mut self) -> Result<()> {
        self.send_key(Key::ChannelUp).await
    }

    /// Goes to the previous channel.
    pub async fn channel_down(&mut self) -> Result<()> {
        self.send_key(Key::ChannelDown).await
    }

    /// Opens the home screen.
    pub async fn home(&mut self) -> Result<()> {
        self.send_key(Key::Home).await
    }

    /// Goes back.
    pub async fn back(&mut self) -> Result<()> {
        self.send_key(Key::Return).await
    }

    /// Opens the menu.
    pub async fn menu(&mut self) -> Result<()> {
        self.send_key(Key::Menu).await
    }

    /// Opens the source selection screen.
    pub async fn source(&mut self) -> Result<()> {
        self.send_key(Key::Source).await
    }

    // === Navigation ===

    /// Navigates up.
    pub async fn up(&mut self) -> Result<()> {
        self.send_key(Key::Up).await
    }

    /// Navigates down.
    pub async fn down(&mut self) -> Result<()> {
        self.send_key(Key::Down).await
    }

    /// Navigates left.
    pub async fn left(&mut self) -> Result<()> {
        self.send_key(Key::Left).await
    }

    /// Navigates right.
    pub async fn right(&mut self) -> Result<()> {
        self.send_key(Key::Right).await
    }

    /// Selects the current item.
    pub async fn enter(&mut self) -> Result<()> {
        self.send_key(Key::Enter).await
    }

    // === Media Playback ===

    /// Starts playback.
    pub async fn play(&mut self) -> Result<()> {
        self.send_key(Key::Play).await
    }

    /// Pauses playback.
    pub async fn pause(&mut self) -> Result<()> {
        self.send_key(Key::Pause).await
    }

    /// Stops playback.
    pub async fn stop(&mut self) -> Result<()> {
        self.send_key(Key::Stop).await
    }

    // === App Control ===

    /// Lists installed apps on the TV.
    ///
    /// Note: This may not work on all TV models.
    pub async fn list_apps(&mut self) -> Result<Vec<App>> {
        let payload = AppListPayload::new();
        self.connection
            .send(&serde_json::to_string(&payload)?)
            .await?;

        // Wait for response with apps list
        if let Some(response) = self.connection.receive_timeout(self.config.timeout).await? {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response) {
                if let Some(data) = json.get("data") {
                    if let Some(apps) = data.get("data") {
                        if let Ok(apps) = serde_json::from_value::<Vec<App>>(apps.clone()) {
                            return Ok(apps);
                        }
                    }
                }
            }
        }

        Ok(Vec::new())
    }

    /// Launches an app by its ID.
    ///
    /// Use `app_ids` module constants for common apps, or find the ID
    /// using `list_apps()`.
    pub async fn launch_app(&mut self, app_id: &str) -> Result<()> {
        let payload = AppLaunchPayload::launch(app_id);
        self.connection
            .send(&serde_json::to_string(&payload)?)
            .await?;

        // Wait for acknowledgment
        let _ = self.connection.receive_timeout(self.config.timeout).await;

        Ok(())
    }

    /// Launches an app with deep link metadata.
    ///
    /// Some apps support deep linking to specific content.
    pub async fn launch_app_with_meta(&mut self, app_id: &str, meta: &str) -> Result<()> {
        let payload = AppLaunchPayload::launch_with_meta(app_id, meta);
        self.connection
            .send(&serde_json::to_string(&payload)?)
            .await?;

        let _ = self.connection.receive_timeout(self.config.timeout).await;

        Ok(())
    }

    /// Opens a URL in the TV's web browser.
    ///
    /// Sends `KEY_EXIT` first to bail out of any currently running app —
    /// without this, launching the browser is a no-op on most Tizen versions
    /// when a streaming app is in the foreground.
    pub async fn open_browser(&mut self, url: &str) -> Result<()> {
        self.send_key(Key::Exit).await?;
        sleep(Duration::from_millis(500)).await;

        let payload = BrowserPayload::open_browser(url);
        self.connection
            .send(&serde_json::to_string(&payload)?)
            .await?;

        let _ = self.connection.receive_timeout(self.config.timeout).await;

        Ok(())
    }

    // === Text Input ===

    /// Sends text input to the TV.
    ///
    /// Use this when a text field is focused on the TV.
    pub async fn send_text(&mut self, text: &str) -> Result<()> {
        let payload = serde_json::json!({
            "method": "ms.remote.control",
            "params": {
                "Cmd": base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    text
                ),
                "TypeOfRemote": "SendInputString",
                "DataOfCmd": "base64"
            }
        });

        self.connection
            .send(&serde_json::to_string(&payload)?)
            .await?;

        let _ = self.connection.receive_timeout(self.config.timeout).await;

        Ok(())
    }
}

// === Backward Compatibility ===

#[allow(deprecated)]
use crate::commands::Command;

impl SamsungTV {
    /// Creates a new Samsung TV connection.
    #[deprecated(
        since = "0.2.0",
        note = "Use SamsungTV::connect() with SamsungTvConfig instead"
    )]
    pub async fn new(
        host: &str,
        port: u16,
        name: &str,
        _api_version: &str,
    ) -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = SamsungTvConfig::new(host).port(port).name(name).build();

        Self::connect(config)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Sends a command using the legacy API.
    #[deprecated(since = "0.2.0", note = "Use send_key() instead")]
    #[allow(deprecated)]
    pub async fn send_command(
        &mut self,
        command: Command,
        repeat: usize,
    ) -> std::result::Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let key = command.to_key();
        let mut responses = Vec::new();

        for _ in 0..repeat {
            self.send_key(key)
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

            responses.push(String::new());
            sleep(self.config.key_delay).await;
        }

        Ok(responses)
    }
}

impl Drop for SamsungTV {
    fn drop(&mut self) {
        // Note: We can't call async close() in drop, but the underlying
        // WebSocket will be closed when the connection is dropped.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = SamsungTvConfig::new("192.168.1.100").build();
        assert_eq!(config.host, "192.168.1.100");
        assert_eq!(config.port, 8001);
        assert!(!config.use_tls);
    }
}
