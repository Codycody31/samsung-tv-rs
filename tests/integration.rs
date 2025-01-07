use samsung_tv_rs::{Command, Commands, SamsungTV};

// TODO: Before all tests startup fake tv api

// #[tokio::test]
// async fn test_send_command() {
//     let tv = SamsungTV::new("192.168.68.64", 8001, "samsung-tv", "v2");
//     //
//     // match tv.get_info().await {
//     //     Ok(_) => println!("Got info successfully!"),
//     //     Err(e) => eprintln!("Failed to get info: {}", e),
//     // }
//
//     let result = tv.unwrap().send_command(Commands::KEY_VOLUP, 1).await;
//     assert!(result.is_ok());
// }

#[tokio::test]
async fn test_get_info() {
    let tv = SamsungTV::new("192.168.68.64", 8001, "Desktop Samsung TV Rust Remote", "v2");
    let responses = tv.await.unwrap().send_command(Command::KeyHome, 1).await;

    for response in responses {
        println!("Received response: {:?}", response);
    }
    //
    // let result = tv.get_info().await;
    // println!("{:#?}", result);
    // assert!(result.is_ok());
}