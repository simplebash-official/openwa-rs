# Chapter 8: Real-Time WebSocket Events

For reactive, event-driven applications, `openwa-rs` implements an asynchronous Socket.IO v4 client protocol over WebSocket, allowing you to consume real-time gateway events with low latency.

---

## 1. Enabling the `events` Feature

In your `Cargo.toml`:

```toml
[dependencies]
openwa = { version = "0.1", features = ["events"] }
```

---

## 2. Basic EventStream Connection

Connect to the Socket.IO `/events` namespace and subscribe to events:

```rust
use openwa::events::WSServerMessage;
use openwa::OpenWAClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenWAClient::new("http://localhost:3000", "opw_key")?;

    // 1. Establish WebSocket connection
    let mut stream = client.events().await?;

    // 2. Subscribe to all events across all sessions
    stream.subscribe("*", &["*"])?;

    // 3. Process events as they arrive
    while let Some(msg) = stream.next_message().await {
        match msg {
            WSServerMessage::Event { payload, timestamp } => {
                println!("[{}] Event: '{}' on session '{}'", timestamp, payload.event, payload.session_id);
                println!("Data: {}", payload.data);
            }
            WSServerMessage::Subscribed { session_id, events, .. } => {
                println!("Subscribed to session '{}' events: {:?}", session_id, events);
            }
            WSServerMessage::Error { code, message, .. } => {
                eprintln!("Socket error [{}]: {}", code, message);
            }
            _ => {}
        }
    }

    Ok(())
}
```

---

## 3. Subscription Scopes & Topic Filters

The `stream.subscribe(session_id, events)` method accepts targeted filters:

```rust
// Subscribe to only incoming messages on a single specific session
stream.subscribe("marketing-bot", &["message.received"])?;

// Subscribe to QR codes and status changes for all sessions
stream.subscribe("*", &["session.qr", "session.status"])?;

// Unsubscribe
stream.unsubscribe("marketing-bot", &["message.received"])?;
```

> `message.failed` and `session.reconnect_loop` are **webhook-only**; they cannot be subscribed to on the socket. See [Webhooks](07-webhooks-and-security.md).

---

## 4. Resilient Auto-Reconnection: `ReconnectingEventStream`

For production background daemon services, network drops or gateway restarts should not crash your application.

Use [`ReconnectingEventStream`](../src/events/reconnect.rs), which automatically handles reconnection with exponential backoff and transparently resubscribes all previous topic filters:

```rust
use openwa::events::{ReconnectConfig, WSServerMessage};
use openwa::OpenWAClient;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenWAClient::new("http://localhost:3000", "opw_key")?;

    let config = ReconnectConfig {
        max_retries: 20,
        base_delay: Duration::from_millis(500),
        max_delay: Duration::from_secs(15),
    };

    let mut stream = client.reconnecting_events(Some(config)).await?;

    // Subscriptions registered here are remembered and replayed automatically
    stream.subscribe("*", &["message.received"])?;

    println!("Listening for messages with automatic reconnection...");
    while let Some(msg) = stream.next_message().await {
        if let WSServerMessage::Event { payload, .. } = msg {
            println!("Inbound: {} on {}", payload.event, payload.session_id);
        }
    }

    Ok(())
}
```
