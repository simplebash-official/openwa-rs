# Chapter 1: Getting Started

This guide walks you through setting up `openwa-rs` in your Rust project and sending your first WhatsApp message in under 5 minutes.

---

## 1. Prerequisites

1. A running instance of the [OpenWA Gateway](https://github.com/simplebash-official/openwa-rs) (via Docker, Kubernetes, or Node.js).
2. An active OpenWA API key (set in your server's `OPENWA_API_KEY` environment variable).
3. Rust 1.80.0 or higher.

---

## 2. Adding the Dependency

Add `openwa` and `tokio` to your `Cargo.toml`:

```toml
[dependencies]
openwa = "0.1"
tokio = { version = "1.38", features = ["full"] }
```

### Available Feature Flags

| Feature | Description | Default |
|---|---|---|
| `events` | Enables real-time WebSocket Socket.IO v4 client (`EventStream`, `ReconnectingEventStream`). | Disabled |
| `axum` | Enables turnkey request extractors for Axum web applications (`OpenWAWebhook`, `WebhookSecret`). | Disabled |

To enable all features:
```toml
[dependencies]
openwa = { version = "0.1", features = ["events", "axum"] }
```

---

## 3. Client Initialization

The primary entry point is [`OpenWAClient`](https://docs.rs/openwa/latest/openwa/struct.OpenWAClient.html).

### Simple Initialization

```rust
use openwa::OpenWAClient;

let client = OpenWAClient::new("http://localhost:3000", "your_secret_api_key")?;
```

### Advanced Builder Initialization

For production deployments, use [`OpenWAClientBuilder`](https://docs.rs/openwa/latest/openwa/struct.OpenWAClientBuilder.html) to configure custom timeouts and retry policies:

```rust
use openwa::client::OpenWAClientBuilder;
use openwa::retry::RetryPolicy;
use std::time::Duration;

let retry_policy = RetryPolicy {
    max_retries: 3,
    base_delay: Duration::from_millis(250),
    max_delay: Duration::from_secs(5),
    retryable_statuses: vec![429, 502, 503, 504],
    respect_retry_after: true,
};

let client = OpenWAClientBuilder::new()
    .base_url("https://wa.yourdomain.com")
    .api_key("opw_live_secret_key")
    .timeout(Duration::from_secs(30))
    .retry_policy(Some(retry_policy))
    .build()?;
```

---

## 4. Sending Your First Message

Here is a minimal, complete application that sends a text message:

```rust
use openwa::{OpenWAClient, SendTextRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenWAClient::new("http://localhost:3000", "your_api_key")?;

    // Session identifier (e.g., "default" or session UUID)
    let session_id = "default";
    
    // Target WhatsApp user JID
    let recipient_jid = "1234567890@c.us";

    let request = SendTextRequest::new(recipient_jid, "Hello from Rust! 🦀");

    let response = client.messages().send_text(session_id, request).await?;

    println!("Message sent successfully!");
    println!("Message ID: {}", response.message_id);
    println!("Timestamp:  {}", response.timestamp);

    Ok(())
}
```

> [!NOTE]
> WhatsApp user identifiers must include the appropriate domain suffix:
> - Individual contacts: `<country_code><number>@c.us` (e.g. `1234567890@c.us`)
> - Groups: `<group_id>@g.us` (e.g. `123456-7890@g.us`)
> - Broadcast lists: `status@broadcast`
> - Newsletters / Channels: `<channel_id>@newsletter`

---

## Next Steps

- Learn about the underlying architecture in [02. Architecture & Design](02-architecture-and-design.md).
- Learn how to create sessions and pair phones in [03. Sessions & Pairing](03-sessions-and-pairing.md).
