# Chapter 2: Architecture & Design

This document details the architectural choices, security guarantees, and network protocols implemented in `openwa-rs`.

---

## 1. High-Level Architecture

`openwa-rs` is designed in layered abstractions to maintain separation of concerns:

```
┌────────────────────────────────────────────────────────┐
│                   OpenWAClient                         │
│  (Top-level client providing scoped resource domains)  │
├────────────────────────────────────────────────────────┤
│  sessions()  │  messages()  │  chats()  │  groups()    │
│  webhooks()  │  status()    │  admin()  │  events()    │
├────────────────────────────────────────────────────────┤
│                    Transport                           │
│  - Method Idempotency Fencing                          │
│  - Send Pacing Rate Limit Detection                    │
│  - Exponential Backoff & Jitter                        │
│  - WhatsApp Path Segment Encoding                      │
├────────────────────────────────────────────────────────┤
│           Reqwest Client (Rustls + Tokio)              │
│  - Policy::none() Redirect Protection                  │
│  - HTTP/1.1 & HTTP/2 Keep-Alive Connection Pool        │
└────────────────────────────────────────────────────────┘
```

---

## 2. WhatsApp Identifier Path Encoding

WhatsApp identifiers (JIDs) contain special characters that clash with standard URI percent-encoding rules:
- `@` — separates phone numbers or group IDs from the domain (e.g., `123@c.us`, `123-456@g.us`).
- `:` — used for multi-device device identifiers (e.g., `12345:1@c.us`).
- `+` — sometimes present in legacy phone numbers.

Standard URI encoders frequently turn `@` into `%40` and `:` into `%3A`. Many WhatsApp web socket bridges reject `%40` as invalid chat targets.

Conversely, path traversal characters (`/`, `\`, `#`, `?`) must be strictly escaped to prevent path injection attacks.

`openwa-rs` implements custom path segment encoding via [`encode_path_segment`](../src/transport.rs):

```rust
// Characters preserved verbatim:
// '@', ':', '+', '-', '.', '_'

// Characters safely escaped:
// '/', '?', '#', spaces, non-ASCII
```

---

## 3. Credential Protection & Redirect Fencing

Security standard RFC 7230 notes that standard HTTP clients automatically follow `301` / `302` redirects and may leak authorization headers (such as `X-API-Key`) to third-party domains if an unencrypted or external redirect occurs.

`openwa-rs` enforces a strict no-redirect policy:

```rust
let client = reqwest::Client::builder()
    .redirect(reqwest::redirect::Policy::none())
    .build()?;
```

If the gateway server responds with a redirect status (`301`, `302`, `307`, `308`), the client returns a structured [`OpenWAError::Api`] error rather than leaking credentials to an untrusted endpoint.

---

## 4. Multi-Engine Abstraction

The OpenWA Gateway supports two distinct WhatsApp engines:

1. **`WEBJS` (whatsapp-web.js)**: Runs an internal headless Chromium browser instance controlling WhatsApp Web. Ideal for full fidelity, sticker packs, and legacy web behaviors.
2. **`BAILEYS` (@whiskeysockets/baileys)**: Direct WebSocket binary multi-device connection without browser overhead. Extremely lightweight, low memory footprint, and fast startup.

`openwa-rs` models both engines transparently under unified domain models. Specific differences (such as `ClickButtonRequest` which is supported on Baileys) are clearly demarcated in the API documentation.

---

## 5. Global Validation Pipe Strictness

OpenWA runs with NestJS strict global validation pipes:
```typescript
whitelist: true,
forbidNonWhitelisted: true,
transform: true
```

> [!WARNING]
> Because `forbidNonWhitelisted: true` is enabled on the server, any superfluous field in a JSON payload will cause an immediate **HTTP 400 Bad Request** error.
>
> All `openwa-rs` request DTOs strictly adhere to the exact server validation schemas. Fields that are not accepted by the backend endpoint are prevented at compile time or excluded during serialization using `#[serde(skip_serializing)]`.
