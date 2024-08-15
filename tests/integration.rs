use samsung_tv_rs::{Commands, SamsungTV};

// TODO: Before all tests startup fake tv api

#[tokio::test]
async fn test_send_command() {
    let tv = SamsungTV::new("192.168.1.100", 8001, "v2");

    let result = tv.send_command(Commands::KEY_MUTE).await;
    assert!(result.is_ok());
}
