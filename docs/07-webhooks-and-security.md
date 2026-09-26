# Chapter 7: Webhooks & Security

Webhooks allow the OpenWA Gateway to push real-time WhatsApp events (inbound messages, message delivery ACKs, connection drops, and QR codes) to your HTTP service.

---

## 1. Registering Webhooks

Register a webhook endpoint via `client.webhooks()`.

```rust
use openwa::types::webhook::CreateWebhookRequest;
use std::collections::HashMap;

let mut headers = HashMap::new();
headers.insert("X-Custom-Auth".to_string(), "backend_secret".to_string());

let req = CreateWebhookRequest {
    url: "https://api.yourdomain.com/webhooks/whatsapp".into(),
    events: vec!["message.received".into(), "message.ack".into(), "session.status".into()],
    headers: Some(headers),
};

let webhook = client.webhooks().create("default", req).await?;
println!("Webhook registered with ID: {}", webhook.id);
```

---

## 2. Webhook Security: HMAC-SHA256 Signatures

To ensure webhook payloads originate strictly from your OpenWA instance and have not been tampered with or replayed in transit, OpenWA signs every HTTP request.

### The `X-OpenWA-Signature` Header

The gateway calculates an HMAC-SHA256 hash of the raw HTTP request body using your configured webhook secret, sending the signature in the header:

```http
X-OpenWA-Signature: sha256=a35f29910d56b4618e404b8686a3479be87a0e1c66860155cbefc2fa7c701460
```

### Constant-Time Verification

> [!CAUTION]
> Never compare HMAC signatures using standard `==` string equality operators. Standard string equality fails fast upon finding the first differing byte, creating microsecond timing discrepancies that attackers can exploit via statistical timing attacks.

`openwa-rs` implements cryptographic constant-time comparison using [`subtle::ConstantTimeEq`](https://docs.rs/subtle/latest/subtle/trait.ConstantTimeEq.html):

```rust
use openwa::webhook::verify_signature;

let raw_body: &[u8] = b"{\"event\":\"message.received\",\"sessionId\":\"default\"}";
let secret: &str = "my_webhook_secret_key";
let signature_header: &str = "sha256=a35f29910d56b4618e404b8686a3479be87a0e1c66860155cbefc2fa7c701460";

let is_valid = verify_signature(raw_body, secret, signature_header);
if !is_valid {
    eprintln!("Forgery or tampering detected!");
}
```

---

## 3. Webhook Server Integrations

### Option A: Turnkey Axum Webhook Extractor (`feature = "axum"`)

If you build your web service with [Axum](https://github.com/tokio-rs/axum), enable the `axum` feature flag in `Cargo.toml`:

```toml
[dependencies]
openwa = { version = "0.1", features = ["axum"] }
```

Use [`OpenWAWebhook`](../src/webhook.rs) as an extractor. It automatically extracts the raw bytes, performs constant-time signature verification, and deserializes the body into a typed [`WebhookDelivery<T>`]:

```rust
use axum::{http::StatusCode, routing::post, Extension, Router};
use openwa::webhook::{OpenWAWebhook, WebhookDelivery, WebhookSecret};
use serde_json::Value;

async fn whatsapp_webhook(
    OpenWAWebhook(delivery): OpenWAWebhook<Value>,
) -> StatusCode {
    println!("Received event '{}' for session '{}'", delivery.event, delivery.session_id);
    println!("Payload data: {}", delivery.data);

    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let secret = "my_webhook_secret_key";

    let app = Router::new()
        .route("/webhooks/whatsapp", post(whatsapp_webhook))
        .layer(Extension(WebhookSecret::new(secret)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    println!("Listening for webhooks on http://localhost:4000/webhooks/whatsapp");
    axum::serve(listener, app).await.unwrap();
}
```

### Option B: Standalone Hyper / Tokio Webhook Server

For lightweight microservices without web frameworks, see the full runnable example in [`examples/webhook_server.rs`](../examples/webhook_server.rs):

```bash
OPENWA_WEBHOOK_SECRET="my_secret" cargo run --example webhook_server
```

---

## 4. Standard Event Types

| Event | Data Payload Type | Description |
|:---|:---|:---|
| `message.received` | `MessageRecord` | An inbound message arrived from a user or group. |
| `message.ack` | `MessageAckData` | Delivery receipt update (`pending`, `sent`, `delivered`, `read`). |
| `message.reaction` | `ReactionRecord` | A user added or updated an emoji reaction on a message. |
| `session.qr` | `SessionQrData` | New QR code generated for phone pairing. |
| `session.status` | `SessionStatusData` | Session state changed (`STARTING`, `WORKING`, `FAILED`). |
| `session.authenticated` | `SessionAuthenticatedData` | Session successfully authenticated with WhatsApp. |
| `group.join` | `GroupMembershipChangeData` | A user joined or was added to a group. |
| `group.leave` | `GroupMembershipChangeData` | A user left or was removed from a group. |
