//! Example: Group administration using OpenWA
//!
//! Run with:
//! ```bash
//! OPENWA_API_KEY="your-key" cargo run --example group_admin
//! ```

use openwa::types::group::{CreateGroupRequest, UpdateGroupSettingsRequest};
use openwa::OpenWAClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("OPENWA_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let api_key = env::var("OPENWA_API_KEY").unwrap_or_else(|_| "opw_test_secret".into());

    let client = OpenWAClient::new(base_url, api_key)?;

    println!("1. Creating a new WhatsApp group...");
    let req = CreateGroupRequest::new("Rust Devs Community", vec!["1234567890@c.us".to_string()]);
    let group = client.groups().create("default", req).await?;
    println!("   Group created! ID: {}", group.id);

    println!("2. Updating group settings (announcements only & 24h ephemeral messages)...");
    let settings = UpdateGroupSettingsRequest {
        announce: Some(true),
        locked: Some(true),
        ephemeral_seconds: Some(86400),
        member_add_mode: None,
    };
    client
        .groups()
        .update_settings("default", &group.id, settings)
        .await?;
    println!("   Settings applied!");

    println!("3. Generating group invite link...");
    let link = client.groups().invite_code("default", &group.id).await?;
    println!("   Invite code: {}", link.code);
    println!("   Invite URL:  {}", link.link.as_deref().unwrap_or("-"));

    Ok(())
}
