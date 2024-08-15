# samsung-tv-rs

A Rust library for interacting with Samsung Smart TVs via their API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
samsung-tv-rs = "0.1.0"
```

## Usage

```rust
use samsung_tv_rs::{Commands, SamsungTV};

#[tokio::main]
async fn main() {
    let tv = SamsungTV::new("192.168.1.100", 8001, "v2");

    match tv.get_info().await {
        Ok(_) => println!("Successfully called TV API"),
        Err(e) => eprintln!("Failed to get TV info: {}", e),
    }

    match tv.send_command(Commands::KEY_VOLUP).await {
        Ok(_) => println!("Volume up command sent successfully!"),
        Err(e) => eprintln!("Failed to send command: {}", e),
    }
}
```

## License

This project is licensed under the MIT License.
