# samsung-tv

A Rust library for controlling Samsung Smart TVs via their WebSocket API.

## Features

- **Remote Control** - Send key presses (power, volume, navigation, media controls, etc.)
- **App Control** - Launch apps like Netflix, YouTube, Prime Video, and more
- **Auto-Discovery** - Find Samsung TVs on your local network via SSDP
- **TLS Support** - Secure connections for newer TVs (2018+)
- **Token Authentication** - Automatic token handling and persistence
- **Async** - Built on tokio for efficient async I/O

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
samsung-tv = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use samsung_tv::{SamsungTV, SamsungTvConfig, Key};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to TV (use .secure() for 2018+ TVs)
    let config = SamsungTvConfig::new("192.168.1.100")
        .secure()
        .name("My Rust Remote")
        .build();

    let mut tv = SamsungTV::connect(config).await?;

    // Send commands
    tv.volume_up().await?;
    tv.send_key(Key::Home).await?;

    // Launch an app
    tv.launch_app(samsung_tv::app_ids::NETFLIX).await?;

    tv.disconnect().await?;
    Ok(())
}
```

## Auto-Discovery

Find Samsung TVs on your network without knowing their IP:

```rust
use samsung_tv::discovery::{discover, DiscoveryOptions};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = DiscoveryOptions::new()
        .timeout(Duration::from_secs(5))
        .fetch_info(true);

    let tvs = discover(options)?;

    for tv in tvs {
        println!("Found TV at {}:{}", tv.ip, tv.port);
        if let Some(info) = &tv.info {
            println!("  Name: {}", info.name);
        }
    }
    Ok(())
}
```

## Authentication

Newer Samsung TVs (2018+) require token authentication:

1. On first connection, the TV displays an "Allow connection?" prompt
2. Accept the prompt on your TV
3. The library receives and stores the authentication token
4. Future connections use the saved token automatically

To persist tokens between sessions:

```rust
use std::path::PathBuf;

let config = SamsungTvConfig::new("192.168.1.100")
    .secure()
    .token_file(PathBuf::from("/home/user/.samsung_tv_token"))
    .build();
```

## Available Keys

The `Key` enum includes 100+ remote control keys:

| Category | Keys |
|----------|------|
| **Power** | `Power`, `PowerOff` |
| **Volume** | `VolumeUp`, `VolumeDown`, `Mute` |
| **Channel** | `ChannelUp`, `ChannelDown`, `ChannelList`, `PreviousChannel` |
| **Navigation** | `Up`, `Down`, `Left`, `Right`, `Enter`, `Return`, `Exit`, `Home`, `Menu` |
| **Numbers** | `Num0` - `Num9` |
| **Media** | `Play`, `Pause`, `Stop`, `Rewind`, `FastForward`, `Record` |
| **Colors** | `Red`, `Green`, `Yellow`, `Blue` |
| **Sources** | `Source`, `Hdmi`, `Hdmi1`-`Hdmi4`, `Tv`, `Dtv` |
| **Smart** | `SmartHub`, `Apps`, `Browser`, `Search`, `Voice` |

## App Control

Launch popular streaming apps:

```rust
use samsung_tv::app_ids;

// Launch by app ID constant
tv.launch_app(app_ids::NETFLIX).await?;
tv.launch_app(app_ids::YOUTUBE).await?;
tv.launch_app(app_ids::PRIME_VIDEO).await?;
tv.launch_app(app_ids::DISNEY_PLUS).await?;
tv.launch_app(app_ids::SPOTIFY).await?;
tv.launch_app(app_ids::PLEX).await?;

// Open a URL in the TV browser
tv.open_browser("https://example.com").await?;
```

## Convenience Methods

```rust
// Power
tv.power_off().await?;
tv.power_toggle().await?;

// Volume
tv.volume_up().await?;
tv.volume_down().await?;
tv.mute().await?;

// Channels
tv.channel_up().await?;
tv.channel_down().await?;

// Navigation
tv.up().await?;
tv.down().await?;
tv.left().await?;
tv.right().await?;
tv.enter().await?;
tv.back().await?;
tv.home().await?;
tv.menu().await?;

// Media
tv.play().await?;
tv.pause().await?;
tv.stop().await?;

// Multiple keys with delay
tv.send_keys(&[Key::Down, Key::Down, Key::Enter]).await?;

// Hold a key (e.g., for rapid volume change)
tv.hold_key(Key::VolumeUp, Duration::from_secs(2)).await?;
```

## Configuration Options

```rust
use std::time::Duration;

let config = SamsungTvConfig::new("192.168.1.100")
    .port(8002)                              // WebSocket port (default: 8001, secure: 8002)
    .secure()                                // Enable TLS (sets port to 8002)
    .name("My Remote")                       // Name shown on TV during pairing
    .token("abc123")                         // Use existing token
    .token_file(PathBuf::from("~/.token"))   // Persist token to file
    .timeout(Duration::from_secs(10))        // Connection timeout
    .key_delay(Duration::from_millis(300))   // Delay between key presses
    .auto_reconnect(true)                    // Auto-reconnect on disconnect
    .build();
```

## TV Information

```rust
let info = tv.get_info().await?;
println!("TV Name: {}", info.name);
println!("Model: {:?}", info.device.modelName);
println!("Firmware: {:?}", info.device.firmwareVersion);
```

## Examples

Run the included examples:

```bash
# Discover TVs on your network
cargo run --example discover

# Basic remote control
cargo run --example basic -- 192.168.1.100

# App launcher
cargo run --example apps -- 192.168.1.100 netflix
```

## Supported TVs

This library supports Samsung Smart TVs with the WebSocket remote control API:

- **2016+** models: Use port 8001 (ws://)
- **2018+** models: Use port 8002 (wss://) with `.secure()`

The TV must be on the same network and have "Remote Control" enabled in settings.

## License

This project is licensed under the MIT License.
