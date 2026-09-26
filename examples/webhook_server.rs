use openwa::webhook::{verify_signature, WebhookDelivery};
use serde_json::Value;

/// Example demonstrating how to verify HMAC-SHA256 signatures and deserialize
/// webhook payloads received in any Rust web framework (Axum, Actix, Warp, etc.).
fn main() {
    let webhook_secret = "whsec_super_secret_webhook_key_123";

    // Simulated incoming HTTP request
    let raw_body = br#"{
        "event": "message.received",
        "timestamp": "2026-09-26T12:00:00Z",
        "sessionId": "sess-uuid-42",
        "idempotencyKey": "idemp_abc_789",
        "deliveryId": "deliv_123",
        "data": {
            "id": "true_1234567890@c.us_3EB0...",
            "body": "Hi, I need assistance with my order!",
            "from": "1234567890@c.us",
            "to": "0987654321@c.us"
        }
    }"#;

    // Simulate X-OpenWA-Signature header
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let mut mac = Hmac::<Sha256>::new_from_slice(webhook_secret.as_bytes()).unwrap();
    mac.update(raw_body);
    let signature_hex = hex::encode(mac.finalize().into_bytes());
    let incoming_header = format!("sha256={}", signature_hex);

    // 1. Verify HMAC-SHA256 signature in constant time
    if !verify_signature(raw_body, webhook_secret, &incoming_header) {
        eprintln!("Rejected: Webhook signature verification failed!");
        return;
    }
    println!("Signature verified successfully!");

    // 2. Deserialize strongly-typed WebhookDelivery envelope
    let delivery: WebhookDelivery<Value> = match serde_json::from_slice(raw_body) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to parse webhook JSON envelope: {}", e);
            return;
        }
    };

    println!("Received Webhook Event:");
    println!("  Event Name:       {}", delivery.event);
    println!("  Session ID:       {}", delivery.session_id);
    println!("  Idempotency Key:  {:?}", delivery.idempotency_key);
    println!("  Payload:          {}", delivery.data);
}
