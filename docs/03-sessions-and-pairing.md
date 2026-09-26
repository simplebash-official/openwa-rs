# Chapter 3: Sessions & Phone Pairing

Sessions represent authenticated WhatsApp client instances on the OpenWA Gateway. A single gateway can host dozens or hundreds of independent WhatsApp sessions concurrently.

---

## 1. Session Lifecycle State Machine

A session moves through the following lifecycle states:

```
[STOPPED] ──► start() ──► [STARTING] ──► [SCAN_QR_CODE]
                              │                │
                              │ (paired)       │ (scan or pair-code)
                              ▼                ▼
                          [WORKING] ◄──────────┘
                              │
                    error / disconnect
                              ▼
                           [FAILED]
```

- **`STOPPED`**: Session exists in the database but the WhatsApp engine is offline.
- **`STARTING`**: Engine is initializing (spawning Chromium or establishing socket).
- **`SCAN_QR_CODE`**: Awaiting QR scan or pairing code verification.
- **`WORKING`**: Authenticated and connected to WhatsApp servers; ready to send and receive messages.
- **`FAILED`**: Session encountered an unrecoverable engine crash or auth revocation.

---

## 2. Creating a Session

```rust
use openwa::types::session::CreateSessionRequest;
use openwa::OpenWAClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenWAClient::new("http://localhost:3000", "opw_key")?;

    let req = CreateSessionRequest::new("marketing-bot")
        .with_start(true); // Automatically starts the session engine

    let session = client.sessions().create(req).await?;
    println!("Created session: ID={}, Status={:?}", session.id, session.status);

    Ok(())
}
```

### Specifying Engine and Proxy

```rust
use openwa::types::session::CreateSessionRequest;

let req = CreateSessionRequest::new("eu-support-agent")
    .with_engine("BAILEYS") // "WEBJS" or "BAILEYS"
    .with_proxy("http://user:pass@192.168.1.100:8080")
    .with_start(true);
```

---

## 3. Phone Pairing Workflows

You can link a phone to a session using either visual QR Code scanning or an 8-digit Pairing Code.

### Method A: Pairing via QR Code

```rust
let qr = client.sessions().get_qr_code("marketing-bot").await?;

println!("Scan this QR code in WhatsApp > Linked Devices:");
println!("{}", qr.qr_code);

if let Some(png_data_url) = qr.qr_code_url {
    println!("PNG Data URL: {}", png_data_url);
}
```

### Method B: Pairing via 8-Digit Pairing Code

Pairing codes allow headless linking without camera interaction:

```rust
// Phone number in international format without '+' (e.g. "1234567890")
let pairing = client
    .sessions()
    .get_pairing_code("marketing-bot", "1234567890")
    .await?;

println!("Enter this code on your phone in WhatsApp > Linked Devices > Link with phone number:");
println!("Pairing Code: {}", pairing.pairing_code);
```

---

## 4. Session Operations

### Listing Sessions

```rust
use openwa::types::session::ListSessionsQuery;

let sessions = client.sessions().list(Some(ListSessionsQuery {
    limit: Some(20),
    offset: Some(0),
    name: None,
})).await?;

for s in sessions {
    println!("- {} (status: {:?})", s.name, s.status);
}
```

### Starting, Stopping & Restarting

```rust
// Start an existing stopped session
client.sessions().start("marketing-bot").await?;

// Stop a session cleanly
client.sessions().stop("marketing-bot").await?;

// Restart a session
client.sessions().restart("marketing-bot").await?;

// Logout and remove local WhatsApp credentials
client.sessions().logout("marketing-bot").await?;

// Delete session permanently from gateway
client.sessions().delete("marketing-bot").await?;
```

---

## 5. Setting Presence & Availability

Update the session's overall online status:

```rust
use openwa::types::session::SetOwnPresenceRequest;

// Set online
client.sessions().set_presence("marketing-bot", SetOwnPresenceRequest::new(true)).await?;

// Set offline
client.sessions().set_presence("marketing-bot", SetOwnPresenceRequest::new(false)).await?;
```
