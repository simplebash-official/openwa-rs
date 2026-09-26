//! # OpenWA Rust Client Library
//!
//! Official-grade, idiomatic Rust client library for the [OpenWA WhatsApp API Gateway](https://github.com/mayurasandakalum/openwa-rs).
//!
//! Provides comprehensive support for all 160 REST endpoints across 24 tags, constant-time HMAC-SHA256
//! webhook signature verification, method idempotency retry fencing, and real-time Socket.IO events.
//!
//! ## Quick Start
//!
//! ```no_run
//! use openwa::{OpenWAClient, SendTextRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = OpenWAClient::new("http://localhost:3000", "my-api-key")?;
//!
//!     // Send a text message
//!     let res = client
//!         .messages()
//!         .send_text(
//!             "my-session-id",
//!             SendTextRequest::new("1234567890@c.us", "Hello from Rust!"),
//!         )
//!         .await?;
//!
//!     println!("Message sent with ID: {}", res.message_id);
//!     Ok(())
//! }

//! ```
//!
//! ## Webhook Signature Verification
//!
//! ```
//! use openwa::webhook::verify_signature;
//!
//! let payload = b"{\"event\":\"message.received\",\"sessionId\":\"sess-1\"}";
//! let secret = "my_webhook_secret";
//! // Header format is "sha256=<hex>"
//! let signature_header = "sha256=...";
//!
//! let is_valid = verify_signature(payload, secret, signature_header);
//! ```

pub mod client;
pub mod error;
pub mod events;
pub mod resources;
pub mod retry;
pub mod transport;
pub mod types;
pub mod webhook;

pub use client::{OpenWAClient, OpenWAClientBuilder};
pub use error::{OpenWAApiError, OpenWAError, OpenWARateLimitError};
pub use retry::RetryPolicy;
pub use types::*;
pub use webhook::{verify_signature, WebhookDelivery, WebhookHeaders};

#[cfg(feature = "events")]
pub use events::EventStream;
