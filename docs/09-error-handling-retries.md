# Chapter 9: Error Handling & Retry Policies

Robust network services must anticipate timeouts, server reboots, HTTP backpressure, and WhatsApp anti-spam rate limiting. `openwa-rs` models errors with high granularity and implements safe retry fencing.

---

## 1. Error Taxonomy: `OpenWAError`

All methods return `Result<T, OpenWAError>`. The enum encompasses all failure modes:

```rust
use openwa::error::OpenWAError;

match result {
    Ok(data) => println!("Success: {:?}", data),
    Err(OpenWAError::Api(api_err)) => {
        // Structured error response from the OpenWA gateway
        eprintln!("API Error [HTTP {}]: {} ({})", api_err.status_code, api_err.message, api_err.code);
    }
    Err(OpenWAError::RateLimited(rl)) => {
        // 429 Too Many Requests or SEND_PACING_LIMITED
        eprintln!("Rate limited! Retry after: {:?}, Is send pacing: {}", rl.retry_after, rl.is_send_pacing);
    }
    Err(OpenWAError::Authentication(msg)) => {
        // 401 Unauthorized or 403 Forbidden
        eprintln!("Auth failure: {}", msg);
    }
    Err(OpenWAError::NotFound(msg)) => {
        // 404 Not Found (e.g., unknown session or message ID)
        eprintln!("Resource not found: {}", msg);
    }
    Err(OpenWAError::Timeout { timeout, context, .. }) => {
        eprintln!("Request timed out after {:?} during: {}", timeout, context);
    }
    Err(OpenWAError::Transport(e)) => {
        eprintln!("Network transport error: {}", e);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

---

## 2. WhatsApp Anti-Spam: `SEND_PACING_LIMITED` vs HTTP 429

WhatsApp enforces strict sending quotas on linked devices. If you send too rapidly, OpenWA returns an HTTP 429 with error code:
```json
{
  "code": "SEND_PACING_LIMITED",
  "message": "Send rate exceeds WhatsApp anti-spam limits",
  "status": 429
}
```

> [!CAUTION]
> When `SEND_PACING_LIMITED` occurs, retrying immediately or with naive exponential backoff risks permanent WhatsApp number bans.
>
> The `openwa-rs` transport layer automatically detects `SEND_PACING_LIMITED` and **refuses to retry it**, immediately bubbling it up to your application as `OpenWARateLimitError { is_send_pacing: true, .. }` so you can halt or delay your message queue.

---

## 3. Method Idempotency Fencing

Standard HTTP clients often retry any request that encounters a network glitch or a 503 error.

In WhatsApp messaging:
- If a `POST /messages/send-text` request timed out while waiting for a response, the gateway might have already sent the message to the customer's phone!
- Retrying a non-idempotent `POST` will result in duplicate messages being delivered to the customer.

`openwa-rs` enforces strict **idempotency fencing**:
- **Idempotent methods (`GET`, `PUT`, `DELETE`)**: Automatically retried on transient network disconnects, HTTP 502, 503, and standard 429 backpressure.
- **Non-idempotent methods (`POST`, `PATCH`)**: **Never automatically retried** unless an explicit idempotency key was accepted and verified.

---

## 4. Customizing the Retry Policy

Configure [`RetryPolicy`](../src/retry.rs) via `OpenWAClientBuilder`:

```rust
use openwa::client::OpenWAClientBuilder;
use openwa::retry::RetryPolicy;
use std::time::Duration;

let custom_retry = RetryPolicy {
    max_retries: 5,
    base_delay: Duration::from_millis(300),
    max_delay: Duration::from_secs(10),
    retryable_statuses: vec![429, 502, 503, 504],
    respect_retry_after: true, // Honors Retry-After HTTP headers from gateway
};

let client = OpenWAClientBuilder::new()
    .base_url("http://localhost:3000")
    .api_key("opw_key")
    .retry_policy(Some(custom_retry))
    .build()?;
```
