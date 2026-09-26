use openwa::{OpenWAClient, SendTextRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("OPENWA_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let api_key = env::var("OPENWA_API_KEY").expect("OPENWA_API_KEY must be set");
    let session_id = env::var("OPENWA_SESSION_ID").unwrap_or_else(|_| "default".to_string());
    let recipient = env::var("RECIPIENT_PHONE").unwrap_or_else(|_| "1234567890@c.us".to_string());

    let client = OpenWAClient::new(base_url, api_key)?;

    println!(
        "Sending message to {} via session {}...",
        recipient, session_id
    );

    let res = client
        .messages()
        .send_text(
            &session_id,
            SendTextRequest::new(recipient, "Hello from the openwa Rust SDK! 🦀"),
        )
        .await?;

    println!("Message sent successfully!");
    println!("  Message ID: {}", res.message_id);
    println!("  Timestamp:  {}", res.timestamp);

    Ok(())
}
