// TODO: Not sure if actually is enum, or it must be a string...

#[derive(Debug)]
pub enum Command {
    KeyVolUp,
    KeyVolDown,
    KeyMute,
    KeyPower,
    // Add other commands here
}

impl Command {
    pub fn as_str(&self) -> &'static str {
        match self {
            Command::KeyVolUp => "KEY_VOLUP",
            Command::KeyVolDown => "KEY_VOLDOWN",
            Command::KeyMute => "KEY_MUTE",
            Command::KeyPower => "KEY_POWER",
        }
    }
}

pub struct Commands;

impl Commands {
    pub const KEY_VOLUP: Command = Command::KeyVolUp;
    pub const KEY_VOLDOWN: Command = Command::KeyVolDown;
    pub const KEY_MUTE: Command = Command::KeyMute;
    pub const KEY_POWER: Command = Command::KeyPower;
    // Add other commands here
}
