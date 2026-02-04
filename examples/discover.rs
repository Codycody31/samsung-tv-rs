//! Discovery example: Find Samsung TVs on the local network.
//!
//! Usage:
//!   cargo run --example discover
//!
//! This will scan the local network for Samsung TVs using SSDP.

use samsung_tv_rs::discovery::{discover, DiscoveryOptions};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Scanning for Samsung TVs on the local network...\n");

    let options = DiscoveryOptions::new()
        .timeout(Duration::from_secs(5))
        .fetch_info(true); // Also fetch detailed TV info

    let tvs = discover(options)?;

    if tvs.is_empty() {
        println!("No Samsung TVs found.");
        println!("\nTips:");
        println!("  - Make sure your TV is powered on");
        println!("  - Ensure your computer is on the same network as the TV");
        println!("  - Some TVs may have network standby disabled");
        return Ok(());
    }

    println!("Found {} Samsung TV(s):\n", tvs.len());

    for (i, tv) in tvs.iter().enumerate() {
        println!("{}. {}", i + 1, tv.ip);
        println!("   Port: {} ({})", tv.port, if tv.supports_tls { "secure" } else { "insecure" });

        if let Some(name) = &tv.name {
            println!("   Name: {}", name);
        }

        if let Some(model) = &tv.model {
            println!("   Model: {}", model);
        }

        if let Some(info) = &tv.info {
            println!("   TV Name: {}", info.name);
            if let Some(model_name) = &info.device.modelName {
                println!("   Model Name: {}", model_name);
            }
            if let Some(firmware) = &info.device.firmwareVersion {
                println!("   Firmware: {}", firmware);
            }
        }

        println!();
    }

    // Print connection examples
    println!("--- Connection Examples ---\n");

    for tv in &tvs {
        if tv.supports_tls {
            println!(
                "let config = SamsungTvConfig::new(\"{}\").secure();",
                tv.ip
            );
        } else {
            println!("let config = SamsungTvConfig::new(\"{}\");", tv.ip);
        }
    }

    Ok(())
}
