//! App control for Samsung Smart TVs.
//!
//! This module provides functionality for listing and launching apps on Samsung TVs.

use serde::{Deserialize, Serialize};

/// Information about an installed app on the TV.
#[derive(Debug, Clone, Deserialize)]
pub struct App {
    /// Unique identifier for the app.
    #[serde(rename = "appId")]
    pub app_id: String,

    /// Display name of the app.
    pub name: String,

    /// Whether the app is currently visible/running.
    #[serde(default)]
    pub is_visible: bool,

    /// App icon URL.
    #[serde(default)]
    pub icon: Option<String>,

    /// App version.
    #[serde(default)]
    pub version: Option<String>,
}

/// Payload for launching an app.
#[derive(Debug, Serialize)]
pub(crate) struct AppLaunchPayload {
    pub method: &'static str,
    pub params: AppLaunchParams,
}

#[derive(Debug, Serialize)]
pub(crate) struct AppLaunchParams {
    pub event: &'static str,
    pub to: &'static str,
    pub data: AppLaunchData,
}

#[derive(Debug, Serialize)]
pub(crate) struct AppLaunchData {
    #[serde(rename = "appId")]
    pub app_id: String,
    pub action_type: &'static str,
    #[serde(rename = "metaTag", skip_serializing_if = "Option::is_none")]
    pub meta_tag: Option<String>,
}

impl AppLaunchPayload {
    /// Creates a payload to launch an app by ID.
    pub fn launch(app_id: &str) -> Self {
        Self {
            method: "ms.channel.emit",
            params: AppLaunchParams {
                event: "ed.apps.launch",
                to: "host",
                data: AppLaunchData {
                    app_id: app_id.to_string(),
                    action_type: "DEEP_LINK",
                    meta_tag: None,
                },
            },
        }
    }

    /// Creates a payload to launch an app with deep link metadata.
    pub fn launch_with_meta(app_id: &str, meta: &str) -> Self {
        Self {
            method: "ms.channel.emit",
            params: AppLaunchParams {
                event: "ed.apps.launch",
                to: "host",
                data: AppLaunchData {
                    app_id: app_id.to_string(),
                    action_type: "DEEP_LINK",
                    meta_tag: Some(meta.to_string()),
                },
            },
        }
    }
}

/// Payload for requesting the app list.
#[derive(Debug, Serialize)]
pub(crate) struct AppListPayload {
    pub method: &'static str,
    pub params: AppListParams,
}

#[derive(Debug, Serialize)]
pub(crate) struct AppListParams {
    pub event: &'static str,
    pub data: AppListData,
}

#[derive(Debug, Serialize)]
pub(crate) struct AppListData {}

impl AppListPayload {
    /// Creates a payload to request the installed apps list.
    pub fn new() -> Self {
        Self {
            method: "ms.channel.emit",
            params: AppListParams {
                event: "ed.installedApp.get",
                data: AppListData {},
            },
        }
    }
}

/// Tizen browser app ID — used to open URLs on the TV.
const TIZEN_BROWSER_APP_ID: &str = "org.tizen.browser";

/// Payload for opening the web browser.
///
/// Opening the browser is just launching the Tizen browser app with the
/// target URL passed as `metaTag` and `NATIVE_LAUNCH` as the action type.
pub(crate) type BrowserPayload = AppLaunchPayload;

impl AppLaunchPayload {
    /// Creates a payload to open a URL in the TV's web browser.
    pub fn open_browser(url: &str) -> Self {
        Self {
            method: "ms.channel.emit",
            params: AppLaunchParams {
                event: "ed.apps.launch",
                to: "host",
                data: AppLaunchData {
                    app_id: TIZEN_BROWSER_APP_ID.to_string(),
                    action_type: "NATIVE_LAUNCH",
                    meta_tag: Some(url.to_string()),
                },
            },
        }
    }
}

/// Common app IDs for popular streaming services.
pub mod app_ids {
    /// Netflix app ID
    pub const NETFLIX: &str = "11101200001";
    /// YouTube app ID
    pub const YOUTUBE: &str = "111299001912";
    /// Amazon Prime Video app ID
    pub const PRIME_VIDEO: &str = "3201512006785";
    /// Disney+ app ID
    pub const DISNEY_PLUS: &str = "3201901017640";
    /// Hulu app ID
    pub const HULU: &str = "3201601007625";
    /// Spotify app ID
    pub const SPOTIFY: &str = "3201606009684";
    /// Plex app ID
    pub const PLEX: &str = "3201512006963";
    /// Apple TV app ID
    pub const APPLE_TV: &str = "3201807016597";
    /// HBO Max app ID
    pub const HBO_MAX: &str = "3201601007230";
    /// Twitch app ID
    pub const TWITCH: &str = "3201504001965";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_launch_payload() {
        let payload = AppLaunchPayload::launch("Netflix");
        let json = serde_json::to_string(&payload).unwrap();

        assert!(json.contains("ms.channel.emit"));
        assert!(json.contains("ed.apps.launch"));
        assert!(json.contains("Netflix"));
        assert!(json.contains("DEEP_LINK"));
    }

    #[test]
    fn test_app_launch_with_meta() {
        let payload = AppLaunchPayload::launch_with_meta("YouTube", "video_id=abc123");
        let json = serde_json::to_string(&payload).unwrap();

        assert!(json.contains("YouTube"));
        assert!(json.contains("video_id=abc123"));
    }

    #[test]
    fn test_app_list_payload() {
        let payload = AppListPayload::new();
        let json = serde_json::to_string(&payload).unwrap();

        assert!(json.contains("ms.channel.emit"));
        assert!(json.contains("ed.installedApp.get"));
    }

    #[test]
    fn test_browser_payload() {
        let payload = AppLaunchPayload::open_browser("https://example.com");
        let json = serde_json::to_string(&payload).unwrap();

        assert!(json.contains("https://example.com"));
        assert!(json.contains("org.tizen.browser"));
        assert!(json.contains("NATIVE_LAUNCH"));
        assert!(json.contains("metaTag"));
    }

    #[test]
    fn test_app_launch_with_meta_uses_camel_case() {
        let payload = AppLaunchPayload::launch_with_meta("YouTube", "video_id=abc123");
        let json = serde_json::to_string(&payload).unwrap();

        assert!(json.contains("metaTag"));
        assert!(!json.contains("meta_tag"));
    }
}
