//! Remote control commands for Samsung TVs.
//!
//! This module provides command types and key codes for controlling Samsung TVs.

mod keys;

pub use keys::Key;

use serde::Serialize;

/// Type of command to send to the TV.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CommandType {
    /// Single key press (press and release).
    #[default]
    Click,
    /// Key down event (for holding keys).
    Press,
    /// Key up event (for releasing held keys).
    Release,
}

impl CommandType {
    /// Returns the Samsung protocol string for this command type.
    pub fn as_str(&self) -> &'static str {
        match self {
            CommandType::Click => "Click",
            CommandType::Press => "Press",
            CommandType::Release => "Release",
        }
    }
}

/// A remote control command to send to the TV.
#[derive(Debug, Clone)]
pub struct RemoteCommand {
    /// The key to send.
    pub key: Key,
    /// The type of command (click, press, release).
    pub cmd_type: CommandType,
}

impl RemoteCommand {
    /// Creates a new click command for the specified key.
    pub fn click(key: Key) -> Self {
        Self {
            key,
            cmd_type: CommandType::Click,
        }
    }

    /// Creates a new press (key down) command for the specified key.
    pub fn press(key: Key) -> Self {
        Self {
            key,
            cmd_type: CommandType::Press,
        }
    }

    /// Creates a new release (key up) command for the specified key.
    pub fn release(key: Key) -> Self {
        Self {
            key,
            cmd_type: CommandType::Release,
        }
    }

    /// Converts this command to a JSON payload for the WebSocket API.
    pub fn to_payload(&self) -> CommandPayload {
        CommandPayload {
            method: "ms.remote.control".to_string(),
            params: CommandParams {
                cmd: self.cmd_type.as_str().to_string(),
                data_of_cmd: self.key.as_str().to_string(),
                option: "false".to_string(),
                type_of_remote: "SendRemoteKey".to_string(),
            },
        }
    }
}

/// WebSocket command payload.
#[derive(Debug, Serialize)]
pub struct CommandPayload {
    method: String,
    params: CommandParams,
}

/// Parameters for a remote control command.
#[derive(Debug, Serialize)]
pub struct CommandParams {
    #[serde(rename = "Cmd")]
    cmd: String,
    #[serde(rename = "DataOfCmd")]
    data_of_cmd: String,
    #[serde(rename = "Option")]
    option: String,
    #[serde(rename = "TypeOfRemote")]
    type_of_remote: String,
}

// === Backward Compatibility ===

/// Legacy command enum for backward compatibility.
#[deprecated(since = "0.2.0", note = "Use Key enum instead")]
#[derive(Debug, Clone, Copy)]
pub enum Command {
    KeyVolUp,
    KeyVolDown,
    KeyMute,
    KeyPower,
    KeyHome,
}

#[allow(deprecated)]
impl Command {
    /// Returns the Samsung protocol string for this command.
    pub fn as_str(&self) -> &'static str {
        match self {
            Command::KeyVolUp => "KEY_VOLUP",
            Command::KeyVolDown => "KEY_VOLDOWN",
            Command::KeyMute => "KEY_MUTE",
            Command::KeyPower => "KEY_POWER",
            Command::KeyHome => "KEY_HOME",
        }
    }

    /// Converts this legacy command to the new Key type.
    pub fn to_key(&self) -> Key {
        match self {
            Command::KeyVolUp => Key::VolumeUp,
            Command::KeyVolDown => Key::VolumeDown,
            Command::KeyMute => Key::Mute,
            Command::KeyPower => Key::Power,
            Command::KeyHome => Key::Home,
        }
    }
}

/// Legacy commands constant for backward compatibility.
#[deprecated(since = "0.2.0", note = "Use Key enum directly instead")]
pub struct Commands;

#[allow(deprecated)]
impl Commands {
    pub const KEY_VOLUP: Command = Command::KeyVolUp;
    pub const KEY_VOLDOWN: Command = Command::KeyVolDown;
    pub const KEY_MUTE: Command = Command::KeyMute;
    pub const KEY_POWER: Command = Command::KeyPower;
    pub const KEY_HOME: Command = Command::KeyHome;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_type_str() {
        assert_eq!(CommandType::Click.as_str(), "Click");
        assert_eq!(CommandType::Press.as_str(), "Press");
        assert_eq!(CommandType::Release.as_str(), "Release");
    }

    #[test]
    fn test_remote_command_payload() {
        let cmd = RemoteCommand::click(Key::VolumeUp);
        let payload = cmd.to_payload();

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("ms.remote.control"));
        assert!(json.contains("KEY_VOLUP"));
        assert!(json.contains("Click"));
    }

    #[test]
    fn test_press_release_commands() {
        let press = RemoteCommand::press(Key::VolumeUp);
        assert_eq!(press.cmd_type, CommandType::Press);

        let release = RemoteCommand::release(Key::VolumeUp);
        assert_eq!(release.cmd_type, CommandType::Release);
    }

    #[allow(deprecated)]
    #[test]
    fn test_legacy_command_conversion() {
        let legacy = Command::KeyVolUp;
        assert_eq!(legacy.to_key(), Key::VolumeUp);
    }
}
