//! Basic example: Connect to a Samsung TV and send commands.
//!
//! Usage:
//!   cargo run --example basic -- <TV_IP_ADDRESS>
//!
//! On first connection, you'll need to accept the pairing request on your TV.

use samsung_tv::{Key, SamsungTV, SamsungTvConfig};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get TV IP from command line
    let args: Vec<String> = env::args().collect();
    let tv_ip = args.get(1).map(|s| s.as_str()).unwrap_or("192.168.1.100");

    println!("Connecting to Samsung TV at {}...", tv_ip);

    // Configure the connection
    let config = SamsungTvConfig::new(tv_ip)
        .secure() // Use TLS (port 8002) - required for newer TVs
        .name("Rust Remote Example")
        .token_file(PathBuf::from(
            dirs::home_dir()
                .unwrap_or_default()
                .join(".samsung_tv_token"),
        ))
        .timeout(Duration::from_secs(10))
        .build();

    // Connect to the TV
    let mut tv = SamsungTV::connect(config).await?;

    println!("Connected!");

    // Show token if we received one
    if let Some(token) = tv.token() {
        println!("Authentication token: {}", token);
    }

    // Try to get TV info
    match tv.get_info().await {
        Ok(info) => {
            println!("TV Name: {}", info.name);
            println!("TV Model: {:?}", info.device.modelName);
        }
        Err(e) => {
            println!("Could not get TV info: {}", e);
        }
    }

    // Send some commands with delays
    println!("\nSending volume up...");
    tv.volume_up().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    println!("Sending volume down...");
    tv.volume_down().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    println!("Sending home key...");
    tv.send_key(Key::Home).await?;

    // Example of sending multiple keys
    println!("\nNavigating: down, down, enter...");
    tv.send_keys(&[Key::Down, Key::Down, Key::Enter]).await?;

    println!("\nDone!");

    // Disconnect gracefully
    tv.disconnect().await?;

    Ok(())
}
