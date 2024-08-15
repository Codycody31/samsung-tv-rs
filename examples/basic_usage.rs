use samsung_tv_rs::{Commands, SamsungTV};

#[tokio::main]
async fn main() {
    let tv = SamsungTV::new("192.168.1.100", 8001, "v2");

    match tv.send_command(Commands::KEY_VOLUP).await {
        Ok(_) => println!("Volume up command sent successfully!"),
        Err(e) => eprintln!("Failed to send command: {}", e),
    }
}
