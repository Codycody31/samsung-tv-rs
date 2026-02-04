//! Samsung TV remote control key codes.
//!
//! This module contains all known key codes for Samsung Smart TV remotes.

use std::fmt;

/// All available remote control keys for Samsung TVs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Key {
    // === Power ===
    /// Power toggle
    Power,
    /// Power off only
    PowerOff,

    // === Volume ===
    /// Increase volume
    VolumeUp,
    /// Decrease volume
    VolumeDown,
    /// Toggle mute
    Mute,

    // === Channel ===
    /// Next channel
    ChannelUp,
    /// Previous channel
    ChannelDown,
    /// Show channel list
    ChannelList,
    /// Return to previous channel
    PreviousChannel,

    // === Navigation ===
    /// Navigate up
    Up,
    /// Navigate down
    Down,
    /// Navigate left
    Left,
    /// Navigate right
    Right,
    /// Confirm/Select
    Enter,
    /// Go back
    Return,
    /// Exit current menu
    Exit,
    /// Go to home screen
    Home,
    /// Open menu
    Menu,

    // === Numbers ===
    /// Number 0
    Num0,
    /// Number 1
    Num1,
    /// Number 2
    Num2,
    /// Number 3
    Num3,
    /// Number 4
    Num4,
    /// Number 5
    Num5,
    /// Number 6
    Num6,
    /// Number 7
    Num7,
    /// Number 8
    Num8,
    /// Number 9
    Num9,

    // === Media Playback ===
    /// Start playback
    Play,
    /// Pause playback
    Pause,
    /// Stop playback
    Stop,
    /// Rewind
    Rewind,
    /// Fast forward
    FastForward,
    /// Start recording
    Record,

    // === Color Buttons ===
    /// Red function button
    Red,
    /// Green function button
    Green,
    /// Yellow function button
    Yellow,
    /// Blue function button
    Blue,

    // === Input Sources ===
    /// Open source selection
    Source,
    /// HDMI (generic)
    Hdmi,
    /// HDMI port 1
    Hdmi1,
    /// HDMI port 2
    Hdmi2,
    /// HDMI port 3
    Hdmi3,
    /// HDMI port 4
    Hdmi4,
    /// Component input 1
    Component1,
    /// Component input 2
    Component2,
    /// AV input 1
    Av1,
    /// AV input 2
    Av2,
    /// AV input 3
    Av3,
    /// TV tuner
    Tv,
    /// Digital TV tuner
    Dtv,

    // === Smart Features ===
    /// Open Smart Hub
    SmartHub,
    /// Open apps menu
    Apps,
    /// Open web browser
    Browser,
    /// Open search
    Search,
    /// Activate voice control
    Voice,
    /// Ambient mode (Frame TVs)
    Ambient,

    // === Picture Settings ===
    /// Change picture size
    PictureSize,
    /// Change picture mode
    PictureMode,
    /// Toggle aspect ratio
    AspectRatio,

    // === Settings & Info ===
    /// Open settings menu
    Settings,
    /// Show program info
    Info,
    /// Open TV guide
    Guide,
    /// Open tools menu
    Tools,
    /// Set sleep timer
    Sleep,

    // === Teletext ===
    /// Toggle teletext
    Teletext,
    /// Teletext mix mode
    TeletextMix,

    // === Additional Media Controls ===
    /// Skip to next track/chapter
    Next,
    /// Skip to previous track/chapter
    Previous,

    // === Picture-in-Picture ===
    /// Toggle PIP
    Pip,
    /// Swap PIP windows
    PipSwap,
    /// Change PIP size
    PipSize,
    /// Toggle PIP on/off
    PipOnOff,

    // === 3D (Legacy) ===
    /// Toggle 3D mode
    ThreeD,

    // === Accessibility ===
    /// Toggle subtitles/closed captions
    Subtitle,
    /// Audio description
    AudioDescription,

    // === DVR Functions ===
    /// DVR menu
    DvrMenu,
    /// Live TV
    Live,

    // === Miscellaneous ===
    /// Open E-Manual
    EManual,
    /// Open factory menu (use with caution)
    FactoryMenu,
    /// Toggle auto program
    AutoProgram,
    /// Open contents menu
    Contents,
    /// Social media features
    Social,
    /// Open MTS (audio) menu
    Mts,
    /// Caption/subtitle options
    Caption,
    /// Open network settings
    Network,

    // === Extended Keys ===
    /// Page up in lists
    PageUp,
    /// Page down in lists
    PageDown,
    /// Enter multi-view
    MultiView,
    /// Extra button
    Extra,
    /// Open calendar
    Calendar,
    /// Open email
    Email,
    /// Help menu
    Help,
    /// Open my apps
    MyApps,

    // === Input Method ===
    /// Switch input language
    Language,
    /// Open on-screen keyboard
    Keyboard,

    // === Custom key code
    /// A custom key code not in the standard list
    Custom(&'static str),
}

impl Key {
    /// Returns the Samsung protocol string for this key.
    pub fn as_str(&self) -> &str {
        match self {
            // Power
            Key::Power => "KEY_POWER",
            Key::PowerOff => "KEY_POWEROFF",

            // Volume
            Key::VolumeUp => "KEY_VOLUP",
            Key::VolumeDown => "KEY_VOLDOWN",
            Key::Mute => "KEY_MUTE",

            // Channel
            Key::ChannelUp => "KEY_CHUP",
            Key::ChannelDown => "KEY_CHDOWN",
            Key::ChannelList => "KEY_CH_LIST",
            Key::PreviousChannel => "KEY_PRECH",

            // Navigation
            Key::Up => "KEY_UP",
            Key::Down => "KEY_DOWN",
            Key::Left => "KEY_LEFT",
            Key::Right => "KEY_RIGHT",
            Key::Enter => "KEY_ENTER",
            Key::Return => "KEY_RETURN",
            Key::Exit => "KEY_EXIT",
            Key::Home => "KEY_HOME",
            Key::Menu => "KEY_MENU",

            // Numbers
            Key::Num0 => "KEY_0",
            Key::Num1 => "KEY_1",
            Key::Num2 => "KEY_2",
            Key::Num3 => "KEY_3",
            Key::Num4 => "KEY_4",
            Key::Num5 => "KEY_5",
            Key::Num6 => "KEY_6",
            Key::Num7 => "KEY_7",
            Key::Num8 => "KEY_8",
            Key::Num9 => "KEY_9",

            // Media Playback
            Key::Play => "KEY_PLAY",
            Key::Pause => "KEY_PAUSE",
            Key::Stop => "KEY_STOP",
            Key::Rewind => "KEY_REWIND",
            Key::FastForward => "KEY_FF",
            Key::Record => "KEY_REC",

            // Color Buttons
            Key::Red => "KEY_RED",
            Key::Green => "KEY_GREEN",
            Key::Yellow => "KEY_YELLOW",
            Key::Blue => "KEY_BLUE",

            // Input Sources
            Key::Source => "KEY_SOURCE",
            Key::Hdmi => "KEY_HDMI",
            Key::Hdmi1 => "KEY_HDMI1",
            Key::Hdmi2 => "KEY_HDMI2",
            Key::Hdmi3 => "KEY_HDMI3",
            Key::Hdmi4 => "KEY_HDMI4",
            Key::Component1 => "KEY_COMPONENT1",
            Key::Component2 => "KEY_COMPONENT2",
            Key::Av1 => "KEY_AV1",
            Key::Av2 => "KEY_AV2",
            Key::Av3 => "KEY_AV3",
            Key::Tv => "KEY_TV",
            Key::Dtv => "KEY_DTV",

            // Smart Features
            Key::SmartHub => "KEY_SMART",
            Key::Apps => "KEY_APPS",
            Key::Browser => "KEY_CONVERGENCE",
            Key::Search => "KEY_SEARCH",
            Key::Voice => "KEY_VOICE",
            Key::Ambient => "KEY_AMBIENT",

            // Picture Settings
            Key::PictureSize => "KEY_PICTURE_SIZE",
            Key::PictureMode => "KEY_PMODE",
            Key::AspectRatio => "KEY_ASPECT",

            // Settings & Info
            Key::Settings => "KEY_MENU",
            Key::Info => "KEY_INFO",
            Key::Guide => "KEY_GUIDE",
            Key::Tools => "KEY_TOOLS",
            Key::Sleep => "KEY_SLEEP",

            // Teletext
            Key::Teletext => "KEY_TTX_MIX",
            Key::TeletextMix => "KEY_TTX_SUBFACE",

            // Additional Media
            Key::Next => "KEY_FF_",
            Key::Previous => "KEY_REWIND_",

            // PIP
            Key::Pip => "KEY_PIP_ONOFF",
            Key::PipSwap => "KEY_PIP_SWAP",
            Key::PipSize => "KEY_PIP_SIZE",
            Key::PipOnOff => "KEY_PIP_ONOFF",

            // 3D
            Key::ThreeD => "KEY_PANNEL_CHDOWN",

            // Accessibility
            Key::Subtitle => "KEY_SUB_TITLE",
            Key::AudioDescription => "KEY_AD",

            // DVR
            Key::DvrMenu => "KEY_DVR_MENU",
            Key::Live => "KEY_LIVE",

            // Miscellaneous
            Key::EManual => "KEY_E_MANUAL",
            Key::FactoryMenu => "KEY_FACTORY",
            Key::AutoProgram => "KEY_AUTO_PROGRAM",
            Key::Contents => "KEY_CONTENTS",
            Key::Social => "KEY_SOCIAL",
            Key::Mts => "KEY_MTS",
            Key::Caption => "KEY_CAPTION",
            Key::Network => "KEY_NETWORK",

            // Extended
            Key::PageUp => "KEY_PAGEUP",
            Key::PageDown => "KEY_PAGEDOWN",
            Key::MultiView => "KEY_MULTI_VIEW",
            Key::Extra => "KEY_EXTRA",
            Key::Calendar => "KEY_CALENDAR",
            Key::Email => "KEY_EMAIL",
            Key::Help => "KEY_HELP",
            Key::MyApps => "KEY_ESAVING",

            // Input Method
            Key::Language => "KEY_LANG",
            Key::Keyboard => "KEY_KEYBOARD",

            // Custom
            Key::Custom(s) => s,
        }
    }

    /// Creates a key from a raw Samsung protocol string.
    ///
    /// Returns `None` if the string doesn't match any known key.
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "KEY_POWER" => Key::Power,
            "KEY_POWEROFF" => Key::PowerOff,
            "KEY_VOLUP" => Key::VolumeUp,
            "KEY_VOLDOWN" => Key::VolumeDown,
            "KEY_MUTE" => Key::Mute,
            "KEY_CHUP" => Key::ChannelUp,
            "KEY_CHDOWN" => Key::ChannelDown,
            "KEY_CH_LIST" => Key::ChannelList,
            "KEY_PRECH" => Key::PreviousChannel,
            "KEY_UP" => Key::Up,
            "KEY_DOWN" => Key::Down,
            "KEY_LEFT" => Key::Left,
            "KEY_RIGHT" => Key::Right,
            "KEY_ENTER" => Key::Enter,
            "KEY_RETURN" => Key::Return,
            "KEY_EXIT" => Key::Exit,
            "KEY_HOME" => Key::Home,
            "KEY_MENU" => Key::Menu,
            "KEY_0" => Key::Num0,
            "KEY_1" => Key::Num1,
            "KEY_2" => Key::Num2,
            "KEY_3" => Key::Num3,
            "KEY_4" => Key::Num4,
            "KEY_5" => Key::Num5,
            "KEY_6" => Key::Num6,
            "KEY_7" => Key::Num7,
            "KEY_8" => Key::Num8,
            "KEY_9" => Key::Num9,
            "KEY_PLAY" => Key::Play,
            "KEY_PAUSE" => Key::Pause,
            "KEY_STOP" => Key::Stop,
            "KEY_REWIND" => Key::Rewind,
            "KEY_FF" => Key::FastForward,
            "KEY_REC" => Key::Record,
            "KEY_RED" => Key::Red,
            "KEY_GREEN" => Key::Green,
            "KEY_YELLOW" => Key::Yellow,
            "KEY_BLUE" => Key::Blue,
            "KEY_SOURCE" => Key::Source,
            "KEY_HDMI" => Key::Hdmi,
            "KEY_HDMI1" => Key::Hdmi1,
            "KEY_HDMI2" => Key::Hdmi2,
            "KEY_HDMI3" => Key::Hdmi3,
            "KEY_HDMI4" => Key::Hdmi4,
            "KEY_INFO" => Key::Info,
            "KEY_GUIDE" => Key::Guide,
            "KEY_TOOLS" => Key::Tools,
            "KEY_SMART" => Key::SmartHub,
            "KEY_APPS" => Key::Apps,
            _ => return None,
        })
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_to_string() {
        assert_eq!(Key::Power.as_str(), "KEY_POWER");
        assert_eq!(Key::VolumeUp.as_str(), "KEY_VOLUP");
        assert_eq!(Key::Enter.as_str(), "KEY_ENTER");
    }

    #[test]
    fn test_key_parse() {
        assert_eq!(Key::parse("KEY_POWER"), Some(Key::Power));
        assert_eq!(Key::parse("KEY_VOLUP"), Some(Key::VolumeUp));
        assert_eq!(Key::parse("UNKNOWN"), None);
    }

    #[test]
    fn test_key_display() {
        assert_eq!(format!("{}", Key::Home), "KEY_HOME");
    }

    #[test]
    fn test_custom_key() {
        let custom = Key::Custom("KEY_CUSTOM_123");
        assert_eq!(custom.as_str(), "KEY_CUSTOM_123");
    }
}
