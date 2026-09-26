#[cfg(feature = "events")]
use openwa::events::WSServerMessage;
#[cfg(feature = "events")]
use openwa::OpenWAClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "events"))]
    {
        eprintln!("Please run this example with --features events:");
        eprintln!("  cargo run --example realtime_events --features events");
        return Ok(());
    }

    #[cfg(feature = "events")]
    {
        let base_url =
            env::var("OPENWA_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let api_key = env::var("OPENWA_API_KEY").expect("OPENWA_API_KEY must be set");

        let client = OpenWAClient::new(base_url, api_key)?;

        println!("Connecting to OpenWA Socket.IO /events stream...");
        let mut stream = client.events().await?;
        println!("Connected! Subscribing to all sessions (*)...");

        stream.subscribe("*", &["*"])?;

        println!("Listening for real-time events (Ctrl+C to quit)...");
        while let Some(msg) = stream.next_message().await {
            match msg {
                WSServerMessage::Event { payload, timestamp } => {
                    println!(
                        "[{}] Event: '{}' from session '{}'",
                        timestamp, payload.event, payload.session_id
                    );
                    println!("Data: {}", serde_json::to_string_pretty(&payload.data)?);
                }
                WSServerMessage::Subscribed {
                    session_id, events, ..
                } => {
                    println!("Subscription confirmed for '{}': {:?}", session_id, events);
                }
                WSServerMessage::Error { code, message, .. } => {
                    eprintln!("WebSocket server error ({}): {}", code, message);
                }
                WSServerMessage::Pong { timestamp, .. } => {
                    println!("Heartbeat pong received at {}", timestamp);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
