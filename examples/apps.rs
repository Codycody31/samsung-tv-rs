//! App control example: List and launch apps on a Samsung TV.
//!
//! Usage:
//!   cargo run --example apps -- <TV_IP_ADDRESS> [APP_NAME] [URL]
//!
//! Examples:
//!   cargo run --example apps -- 192.168.1.100                              # List all apps
//!   cargo run --example apps -- 192.168.1.100 netflix                      # Launch Netflix
//!   cargo run --example apps -- 192.168.1.100 browser https://example.com  # Open URL

use samsung_tv::{app_ids, SamsungTV, SamsungTvConfig};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let tv_ip = args.get(1).map(|s| s.as_str()).unwrap_or("192.168.1.100");
    let app_name = args.get(2).map(|s| s.to_lowercase());

    println!("Connecting to Samsung TV at {}...", tv_ip);

    let config = SamsungTvConfig::new(tv_ip)
        .secure()
        .name("Rust App Controller")
        .token_file(PathBuf::from(
            dirs::home_dir()
                .unwrap_or_default()
                .join(".samsung_tv_token"),
        ))
        .timeout(Duration::from_secs(10))
        .build();

    let mut tv = SamsungTV::connect(config).await?;
    println!("Connected!\n");

    match app_name.as_deref() {
        Some("netflix") => {
            println!("Launching Netflix...");
            tv.launch_app(app_ids::NETFLIX).await?;
        }
        Some("youtube") => {
            println!("Launching YouTube...");
            tv.launch_app(app_ids::YOUTUBE).await?;
        }
        Some("prime") | Some("amazon") => {
            println!("Launching Prime Video...");
            tv.launch_app(app_ids::PRIME_VIDEO).await?;
        }
        Some("disney") | Some("disney+") => {
            println!("Launching Disney+...");
            tv.launch_app(app_ids::DISNEY_PLUS).await?;
        }
        Some("spotify") => {
            println!("Launching Spotify...");
            tv.launch_app(app_ids::SPOTIFY).await?;
        }
        Some("plex") => {
            println!("Launching Plex...");
            tv.launch_app(app_ids::PLEX).await?;
        }
        Some("browser") => {
            let url = args
                .get(3)
                .map(|s| s.as_str())
                .unwrap_or("https://www.google.com");
            println!("Opening {} in web browser...", url);
            tv.open_browser(url).await?;
        }
        Some(name) => {
            println!("Unknown app '{}'. Trying to list installed apps...\n", name);
            list_apps(&mut tv).await?;
        }
        None => {
            println!("No app specified. Listing installed apps...\n");
            list_apps(&mut tv).await?;
            println!("\n--- Available shortcuts ---");
            println!("  netflix   - Launch Netflix");
            println!("  youtube   - Launch YouTube");
            println!("  prime     - Launch Prime Video");
            println!("  disney    - Launch Disney+");
            println!("  spotify   - Launch Spotify");
            println!("  plex      - Launch Plex");
            println!("  browser   - Open web browser");
        }
    }

    tv.disconnect().await?;
    Ok(())
}

async fn list_apps(tv: &mut SamsungTV) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching installed apps (this may take a moment)...\n");

    match tv.list_apps().await {
        Ok(apps) => {
            if apps.is_empty() {
                println!("No apps found (or TV doesn't support app listing).");
            } else {
                println!("Installed apps ({}):", apps.len());
                println!("{:-<50}", "");
                for app in apps {
                    println!("  {} (ID: {})", app.name, app.app_id);
                }
            }
        }
        Err(e) => {
            println!("Could not list apps: {}", e);
            println!("Note: App listing may not be supported on all TV models.");
        }
    }

    Ok(())
}
