//! Integration tests for Samsung TV RS.
//!
//! Note: These tests require a real Samsung TV on the network.
//! They are ignored by default - run with `cargo test -- --ignored` to execute.

use samsung_tv_rs::{Key, SamsungTvConfig};

/// Test discovery functionality - scans local network for Samsung TVs.
#[test]
fn test_discovery() {
    use samsung_tv_rs::discovery::{discover, DiscoveryOptions};
    use std::time::Duration;

    println!("\n=== Samsung TV Discovery ===\n");
    println!("Scanning local network for Samsung TVs...\n");

    let options = DiscoveryOptions::new()
        .timeout(Duration::from_secs(5))
        .fetch_info(true);

    let tvs = discover(options).expect("Discovery failed");

    if tvs.is_empty() {
        println!("No Samsung TVs found on the network.");
        println!("\nTips:");
        println!("  - Make sure your TV is powered on");
        println!("  - Ensure your computer is on the same network as the TV");
        println!("  - Some TVs may have network standby disabled");
    } else {
        println!("Found {} Samsung TV(s):\n", tvs.len());
        println!("{:-<60}", "");

        for (i, tv) in tvs.iter().enumerate() {
            println!("\nTV #{}", i + 1);
            println!("  IP Address:  {}", tv.ip);
            println!("  Port:        {}", tv.port);
            println!(
                "  Secure:      {}",
                if tv.supports_tls { "Yes (TLS)" } else { "No" }
            );

            if let Some(location) = &tv.location {
                println!("  Location:    {}", location);
            }

            if let Some(model) = &tv.model {
                println!("  Model:       {}", model);
            }

            if let Some(usn) = &tv.usn {
                println!("  USN:         {}", usn);
            }

            if let Some(info) = &tv.info {
                println!("\n  --- Detailed Info ---");
                println!("  Name:        {}", info.name);
                println!("  Type:        {}", info.device_type);
                println!("  Version:     {}", info.version);

                if let Some(model_name) = &info.device.modelName {
                    println!("  Model Name:  {}", model_name);
                }
                if let Some(firmware) = &info.device.firmwareVersion {
                    println!("  Firmware:    {}", firmware);
                }
                if let Some(os) = &info.device.OS {
                    println!("  OS:          {}", os);
                }
                if let Some(resolution) = &info.device.resolution {
                    println!("  Resolution:  {}", resolution);
                }
            }
        }

        println!("\n{:-<60}", "");
        println!("\nConnection examples:");
        for tv in &tvs {
            if tv.supports_tls {
                println!("  SamsungTvConfig::new(\"{}\").secure()", tv.ip);
            } else {
                println!("  SamsungTvConfig::new(\"{}\")", tv.ip);
            }
        }
    }

    println!("\n=== Discovery Complete ===\n");
}

// === Unit tests that don't require a TV ===

#[test]
fn test_config_builder() {
    let config = SamsungTvConfig::new("192.168.1.100")
        .secure()
        .name("Test")
        .token("abc123")
        .build();

    assert_eq!(config.host, "192.168.1.100");
    assert_eq!(config.port, 8002);
    assert!(config.use_tls);
    assert_eq!(config.token, Some("abc123".to_string()));
}

#[test]
fn test_key_codes() {
    assert_eq!(Key::Power.as_str(), "KEY_POWER");
    assert_eq!(Key::VolumeUp.as_str(), "KEY_VOLUP");
    assert_eq!(Key::Home.as_str(), "KEY_HOME");
    assert_eq!(Key::Enter.as_str(), "KEY_ENTER");
}

#[test]
fn test_remote_command_serialization() {
    use samsung_tv_rs::RemoteCommand;

    let cmd = RemoteCommand::click(Key::VolumeUp);
    let payload = cmd.to_payload();
    let json = serde_json::to_string(&payload).unwrap();

    assert!(json.contains("ms.remote.control"));
    assert!(json.contains("KEY_VOLUP"));
    assert!(json.contains("Click"));
}
