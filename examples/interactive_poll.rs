//! Example: Creating and voting on interactive polls using OpenWA
//!
//! Run with:
//! ```bash
//! OPENWA_API_KEY="your-key" cargo run --example interactive_poll
//! ```

use openwa::types::builders::PollBuilder;
use openwa::types::message::VotePollRequest;
use openwa::OpenWAClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("OPENWA_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let api_key = env::var("OPENWA_API_KEY").unwrap_or_else(|_| "opw_test_secret".into());
    let target_chat = env::var("OPENWA_CHAT_ID").unwrap_or_else(|_| "1234567890@c.us".into());

    let client = OpenWAClient::new(base_url, api_key)?;

    println!("Creating interactive WhatsApp poll using PollBuilder...");
    let poll_req = PollBuilder::new(&target_chat, "Which backend language is best?")
        .option("Rust 🦀")
        .option("TypeScript ⚡")
        .option("Go 🐹")
        .option("Python 🐍")
        .allow_multiple_answers(false)
        .build();

    let res = client.messages().send_poll("default", poll_req).await?;
    println!("Poll created! Message ID: {}", res.message_id);

    println!("Voting on poll for option 'Rust 🦀'...");
    let vote_req = VotePollRequest {
        chat_id: target_chat,
        poll_message_id: res.message_id,
        options: vec!["Rust 🦀".into()],
    };

    let vote_res = client.messages().vote_poll("default", vote_req).await?;
    println!("Vote cast! Success: {}", vote_res.success);

    Ok(())
}
