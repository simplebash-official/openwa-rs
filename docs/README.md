# OpenWA Rust SDK Documentation

Welcome to the comprehensive documentation for **`openwa-rs`**, the official-grade, high-performance Rust client library for the [OpenWA WhatsApp API Gateway](https://github.com/simplebash-official/openwa-rs).

---

## 🧭 Documentation Map

| Chapter | Topic | Description |
|:---|:---|:---|
| **[01. Getting Started](01-getting-started.md)** | Quickstart & Setup | Installation, client initialization, environment configuration, sending your first message. |
| **[02. Architecture & Design](02-architecture-and-design.md)** | Core Architecture | WhatsApp JID path encoding, credential protection, connection pooling, and multi-engine support. |
| **[03. Sessions & Pairing](03-sessions-and-pairing.md)** | Session Lifecycle | Managing sessions, QR code scanning, 8-digit pairing codes, proxies, and multi-engine setup (`WEBJS` vs `BAILEYS`). |
| **[04. Messages & Media](04-messages-and-media.md)** | Messaging Guide | Text, formatting, mentions, images, videos, audio/PTT voice notes, interactive buttons, polls, reactions, and bulk batches. |
| **[05. Chats, Groups & Channels](05-chats-groups-channels.md)** | Conversation Management | Chat archiving, pinning, muting, typing presence, group admin controls, ephemeral timers, and newsletters/channels. |
| **[06. Contacts, Status & Labels](06-contacts-status-labels.md)** | CRM & Social | Number verification (`check_number`), contact profiles, WhatsApp stories/status updates, and WhatsApp Business labels. |
| **[07. Webhooks & Security](07-webhooks-and-security.md)** | Inbound Webhooks | Registering webhooks, constant-time HMAC-SHA256 signature verification, and Axum / Hyper turnkey extractors. |
| **[08. Real-Time Events](08-realtime-events.md)** | WebSocket Streaming | Socket.IO v4 streaming, wildcard event subscriptions, event parsing, and auto-reconnecting streams. |
| **[09. Error Handling & Retries](09-error-handling-retries.md)** | Resilience & Retries | Error categorization, `SEND_PACING_LIMITED` rate limiting vs HTTP 429, and idempotency-aware retries. |
| **[10. Administration & Operations](10-admin-and-operations.md)** | Admin & Ops | Scoped API key management, audit logs, plugins, integrations, system config, and Prometheus `/metrics`. |
| **[11. API Reference Matrix](11-api-reference-matrix.md)** | Full API Matrix | Exhaustive reference table mapping all 24 resources and 160 endpoints to Rust methods and DTOs. |

---

## ⚡ Quick Feature Lookup

- **Sending a message?** See [04. Messages & Media](04-messages-and-media.md).
- **Listening to inbound messages?** See [07. Webhooks & Security](07-webhooks-and-security.md) or [08. Real-Time Events](08-realtime-events.md).
- **Pairing a phone via QR or Pairing Code?** See [03. Sessions & Pairing](03-sessions-and-pairing.md).
- **Managing WhatsApp Groups?** See [05. Chats, Groups & Channels](05-chats-groups-channels.md).
- **Handling rate limits and timeouts?** See [09. Error Handling & Retries](09-error-handling-retries.md).

---

## 🛠 Compatibility & MSRV

- **Minimum Supported Rust Version (MSRV)**: `1.80.0`
- **Supported WhatsApp Engines**: `WEBJS` (WhatsApp Web) & `BAILEYS` (Multi-device socket)
- **Supported OpenWA Server**: `v2.x` and `v3.x`
