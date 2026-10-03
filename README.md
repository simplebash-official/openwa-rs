# openwa-rs

[![CI](https://github.com/simplebash-official/openwa-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/simplebash-official/openwa-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/openwa.svg)](https://crates.io/crates/openwa)
[![Documentation](https://docs.rs/openwa/badge.svg)](https://docs.rs/openwa)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.80.0-informational.svg)](Cargo.toml)


Official-grade, high-performance, idiomatic Rust client library and SDK for the [OpenWA WhatsApp API Gateway](https://github.com/simplebash-official/openwa-rs).

---

## ✨ Features

- **Full OpenAPI Coverage**: Implements **all 160 REST endpoints across 24 distinct domains**, including WhatsApp Web.js and Baileys engines.
- **Strong Typed Safety**: Comprehensive Rust data models with serde derivations, zero implicit untyped JSON envelopes.
- **WhatsApp Path Encoding**: Preserves WhatsApp identifier characters (`@`, `:`, `+`) while escaping path traversal characters (`/`, `#`, `?`).
- **Credential Protection**: Hardened redirect policy (`Policy::none()`) prevents token and header leakage across redirects.
- **Idempotency-Aware Retries**: Automatically retries safe HTTP methods (`GET`, `PUT`, `DELETE`) on `503` / `429` backpressure, while strictly fencing non-idempotent `POST` requests and detecting `SEND_PACING_LIMITED` refusal codes.
- **Local File & Streaming Media**: Asynchronously reads and Base64-encodes local files (`from_file`) and streams large downloads directly to disk (`download_media_to_file`).
- **Fluent Interactive Builders**: Ergonomic builder APIs for native WhatsApp Polls (`PollBuilder`), Text mentions (`TextRequestBuilder`), and bulk campaigns (`BulkMessageBuilder`).
- **Constant-Time Webhook Verification**: Cryptographic HMAC-SHA256 signature validation with `subtle::ConstantTimeEq` and turnkey **Axum** extractor (`OpenWAWebhook`).
- **Resilient Real-Time Events**: Socket.IO v4 client with wildcard subscriptions and auto-reconnecting stream with exponential backoff (`ReconnectingEventStream`).
- **Async Pagination Streams**: Built-in lazy paginators (`Paginator`) and batch collectors (`fetch_all_pages`).
- **Ergonomic Namespace Layout**: Clean separation between standard operator resources (`messages()`, `sessions()`, `chats()`, etc.) and privileged administration (`admin()`).

---

## 📚 Documentation & Guides

Comprehensive, production-ready guides are available in the [`docs/`](docs/) directory:

- 🧭 **[Documentation Hub & Index](docs/README.md)**
- 🚀 **[01. Getting Started](docs/01-getting-started.md)** — Installation, client configuration, sending first message.
- 🏛️ **[02. Architecture & Design](docs/02-architecture-and-design.md)** — JID encoding, redirect fences, multi-engine support.
- 📱 **[03. Sessions & Phone Pairing](docs/03-sessions-and-pairing.md)** — QR codes, 8-digit pairing codes, proxies, lifecycle.
- 💬 **[04. Messages & Media](docs/04-messages-and-media.md)** — Text formatting, media from disk/URL, audio PTT, polls, reactions, bulk.
- 👥 **[05. Chats, Groups & Channels](docs/05-chats-groups-channels.md)** — Unread markers, typing indicators, group admin, ephemeral timers.
- 🏷️ **[06. Contacts, Stories & Labels](docs/06-contacts-status-labels.md)** — Number verification, WhatsApp stories, business labels.
- 🔐 **[07. Webhooks & Security](docs/07-webhooks-and-security.md)** — HMAC-SHA256 verification, Axum and Hyper servers.
- ⚡ **[08. Real-Time WebSocket Events](docs/08-realtime-events.md)** — Socket.IO streaming, topic filters, auto-reconnection.
- 🛡️ **[09. Error Handling & Retries](docs/09-error-handling-retries.md)** — Rate limits, send pacing detection, idempotency fencing.
- ⚙️ **[10. Administration & Operations](docs/10-admin-and-operations.md)** — Scoped API keys, system config, Prometheus metrics.
- 📋 **[11. API Reference Matrix](docs/11-api-reference-matrix.md)** — Complete lookup table for all 24 resources & 160 endpoints.

---

## 📦 Installation

Add `openwa` to your `Cargo.toml`:

```toml
[dependencies]
openwa = "0.1"
tokio = { version = "1.38", features = ["full"] }
```

To enable real-time Socket.IO WebSocket event streaming:

```toml
[dependencies]
openwa = { version = "0.1", features = ["events"] }
```

---

## 🚀 Quick Start

### Sending a Text Message

```rust
use openwa::{OpenWAClient, SendTextRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenWAClient::new("http://localhost:3000", "your-api-key")?;

    let res = client
        .messages()
        .send_text(
            "default",
            SendTextRequest::new("1234567890@c.us", "Hello from Rust! 🦀"),
        )
        .await?;

    println!("Sent message with ID: {}", res.message_id);
    Ok(())
}
```

### Session Creation and QR Pairing

```rust
use openwa::{OpenWAClient, CreateSessionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenWAClient::new("http://localhost:3000", "your-api-key")?;

    // Create a new WhatsApp session
    let session = client
        .sessions()
        .create(CreateSessionRequest::new("marketing-bot"))
        .await?;

    println!("Session created: {} ({:?})", session.name, session.status);

    // Fetch the pairing QR code
    let qr = client.sessions().get_qr_code(&session.id).await?;
    println!("Scan this QR code in WhatsApp: {}", qr.qr_code);

    Ok(())
}
```

---

## 🔐 Webhook Signature Verification

OpenWA signs all outbound webhook deliveries with an HMAC-SHA256 signature in the `X-OpenWA-Signature` header (`sha256=<hex>`).

```rust
use openwa::webhook::{verify_signature, WebhookDelivery};
use serde_json::Value;

fn handle_incoming_webhook(raw_body: &[u8], signature_header: &str, secret: &str) {
    // 1. Verify in constant time to prevent timing attacks
    if !verify_signature(raw_body, secret, signature_header) {
        eprintln!("Invalid webhook signature!");
        return;
    }

    // 2. Deserialize strongly typed envelope
    let delivery: WebhookDelivery<Value> = serde_json::from_slice(raw_body).unwrap();
    println!("Received event '{}' for session '{}'", delivery.event, delivery.session_id);
}
```

---

> All 23 webhook events (plus `*`) are covered by `openwa::WebhookEvent` / `WEBHOOK_EVENTS`, with typed payload structs in `openwa::webhook`. `message.failed` and `session.reconnect_loop` are webhook-only.

## ⚡ Real-Time Events (WebSocket)

Connect to the OpenWA Socket.IO `/events` stream to receive live events in real time:

```rust
use openwa::events::WSServerMessage;
use openwa::OpenWAClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenWAClient::new("http://localhost:3000", "your-api-key")?;

    // Connect to WebSocket /events stream
    let mut stream = client.events().await?;

    // Subscribe to all events across all sessions
    stream.subscribe("*", &["*"])?;

    while let Some(msg) = stream.next_message().await {
        match msg {
            WSServerMessage::Event { payload, timestamp } => {
                println!("[{}] Event: {} on session {}", timestamp, payload.event, payload.session_id);
                println!("Data: {}", payload.data);
            }
            WSServerMessage::Subscribed { session_id, events, .. } => {
                println!("Subscribed to '{}' -> {:?}", session_id, events);
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

## 🛠 Complete API Domain Directory

| Resource | Method | Description |
|:---|:---|:---|
| **Sessions** | `client.sessions()` | List, create, start, stop, restart, QR code pairing, engine status, proxy, and session config |
| **Messages** | `client.messages()` | Send text, media, audio, document, location, contact, buttons, lists, polls, reaction, edit, delete, pin, star, and reply |
| **Contacts** | `client.contacts()` | Check WhatsApp number registration, fetch profiles, profile photos, contact cards, sync |
| **Groups** | `client.groups()` | Create group, invite links, join via code, manage admins, participants, change subject/description, leave |
| **Chats** | `client.chats()` | List chats, unread counters, mark read/unread, archive, pin, mute, and send presence typing/recording |
| **Webhooks** | `client.webhooks()` | CRUD webhooks, secret rotation, ping delivery test, and inspection of failed outbox deliveries |
| **Labels** | `client.labels()` | Manage WhatsApp Business labels and associate them with specific customer chats |
| **Channels** | `client.channels()` | Newsletters / Channels: search, subscribe, unsubscribe, mute, transfer ownership, manage admins |
| **Catalog** | `client.catalog()` | Business product catalog items, collections, pagination, and sending product cards |
| **Status** | `client.status()` | Read contact stories/statuses, download status media, post text, image, video, and audio stories |
| **Search** | `client.search()` | Global search across indexed messages with sender, chat, date range, and direction filters |
| **Templates** | `client.templates()` | Quick reply HSM templates management |
| **Profile** | `client.profile()` | Set session push name, update about text, upload avatar, and remove avatar |
| **Calls** | `client.calls()` | Generate WhatsApp voice/video call links and reject incoming calls |
| **Media** | `client.media()` | Media health diagnostics, voice note transcoding (Opus/Ogg), and MP4 conversion |
| **Health** | `client.health()` | Gateway status, Kubernetes/Docker liveness and readiness probes |
| **Admin** | `client.admin()` | Privileged administrative controls (API keys, stats, plugins, integrations, infra, settings, audit, metrics) |

---

## ⚙️ Advanced Configuration & Custom Retries

```rust
use openwa::retry::RetryPolicy;
use openwa::OpenWAClient;
use std::time::Duration;

let retry_policy = RetryPolicy {
    max_retries: 5,
    base_delay: Duration::from_millis(250),
    max_delay: Duration::from_secs(10),
    retryable_statuses: vec![429, 502, 503, 504],
    respect_retry_after: true,
};

let client = OpenWAClient::builder()
    .base_url("http://localhost:3000")
    .api_key("opw_live_secret_key")
    .timeout(Duration::from_secs(45))
    .retry_policy(Some(retry_policy))
    .build()?;
```

---

## 🏃 Running Examples & Tests

Run the test suite:

```bash
cargo test --all-targets --all-features
```

Run an example:

```bash
OPENWA_API_KEY="your-key" cargo run --example send_text
```

---

## 📄 License

Dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
