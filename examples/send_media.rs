//! Example: Sending media and audio files using OpenWA
//!
//! Run with:
//! ```bash
//! OPENWA_API_KEY="your-key" cargo run --example send_media
//! ```

use openwa::types::message::{SendAudioRequest, SendMediaRequest};
use openwa::OpenWAClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("OPENWA_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let api_key = env::var("OPENWA_API_KEY").unwrap_or_else(|_| "opw_test_secret".into());
    let target_chat = env::var("OPENWA_CHAT_ID").unwrap_or_else(|_| "1234567890@c.us".into());

    let client = OpenWAClient::new(base_url, api_key)?;

    println!("1. Sending image from public URL...");
    let img_req = SendMediaRequest::from_url(&target_chat, "https://picsum.photos/400/300")
        .with_caption("Sent from openwa-rs! 🦀");

    match client.messages().send_image("default", img_req).await {
        Ok(res) => println!("   Image sent! Message ID: {}", res.message_id),
        Err(e) => eprintln!("   Failed to send image: {}", e),
    }

    println!("2. Sending audio voice note (PTT)...");
    let audio_req = SendAudioRequest::voice_note(
        &target_chat,
        "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3",
    );

    match client.messages().send_audio("default", audio_req).await {
        Ok(res) => println!("   Voice note sent! Message ID: {}", res.message_id),
        Err(e) => eprintln!("   Failed to send voice note: {}", e),
    }

    Ok(())
}
