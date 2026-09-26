# Chapter 10: Administration, Operations & Observability

Privileged administrative endpoints require the `ADMIN` role. They are grouped cleanly under `client.admin()`.

---

## 1. Scoped API Key Management

Access API key management via `client.admin().auth_keys()`.

```rust
use openwa::types::auth::CreateAuthKeyRequest;

// 1. Create a scoped API key for a microservice
let req = CreateAuthKeyRequest {
    name: "analytics-worker".into(),
    role: "OPERATOR".into(), // "ADMIN", "OPERATOR", or "VIEWER"
    allowed_sessions: Some(vec!["marketing-bot".into()]),
    expires_at: None,
};

let key = client.admin().auth_keys().create(req).await?;
println!("Generated API Key: {}", key.key); // Shown only once!

// 2. List all active API keys
let keys = client.admin().auth_keys().list().await?;
for k in keys {
    println!("Key: {} (Role: {}, Created: {})", k.name, k.role, k.created_at);
}

// 3. Revoke an API key
client.admin().auth_keys().delete(&key.id).await?;
```

---

## 2. Infrastructure & Configuration

Control server settings and manage restarts via `client.admin().infra()`.

```rust
use openwa::types::infra::{RestartRequest, SaveConfigRequest};

// Read current running gateway configuration
let config = client.admin().infra().get_config().await?;
println!("Config: {:?}", config);

// Update configuration
let update = SaveConfigRequest {
    config: serde_json::json!({
        "webhookUrl": "https://api.yourdomain.com/events",
        "maxMediaSizeMb": 64
    }),
};
client.admin().infra().save_config(update).await?;

// Trigger a graceful server restart
client.admin().infra().restart(RestartRequest {
    all_sessions: Some(true),
}).await?;
```

---

## 3. Observability & Health Probes

### Kubernetes / Docker Health Checks

Access health endpoints via `client.health()`:

```rust
// Gateway liveness probe (HTTP 200 if server process is running)
let live = client.health().liveness().await?;
println!("Liveness: {}", live.status);

// Gateway readiness probe (HTTP 200 if database and redis are healthy)
let ready = client.health().readiness().await?;
println!("Readiness: {}", ready.status);
```

### Prometheus Metrics

Scrape raw Prometheus metrics for Grafana dashboards via `client.admin().metrics()`:

```rust
let prometheus_text = client.admin().metrics().scrape().await?;
println!("Prometheus metrics snippet:\n{}", &prometheus_text[..200]);
```

### System Stats & Audit Logs

```rust
// System hardware and message throughput stats
let stats = client.admin().stats().get().await?;
println!("Uptime: {}s, Total Messages: {}", stats.uptime, stats.total_messages);

// Historical audit trail
let audit_entries = client.admin().audit().list(None).await?;
for entry in audit_entries {
    println!("[{}] User '{}' performed '{}'", entry.timestamp, entry.actor_id, entry.action);
}
```
