use openwa::webhook::{verify_signature, WebhookDelivery};
use openwa::MessageResponse;

#[test]
fn test_verify_signature_valid() {
    let payload = b"{\"event\":\"message.received\",\"sessionId\":\"s-123\"}";
    let secret = "test_webhook_secret_key";
    // echo -n '{"event":"message.received","sessionId":"s-123"}' | openssl dgst -sha256 -hmac 'test_webhook_secret_key'
    // Let's compute with openwa or hmac
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload);
    let signature = hex::encode(mac.finalize().into_bytes());

    let header_with_prefix = format!("sha256={}", signature);
    assert!(verify_signature(payload, secret, &header_with_prefix));
    assert!(verify_signature(payload, secret, &signature));
}

#[test]
fn test_verify_signature_tampered_payload() {
    let payload = b"{\"event\":\"message.received\",\"sessionId\":\"s-123\"}";
    let tampered = b"{\"event\":\"message.received\",\"sessionId\":\"s-456\"}";
    let secret = "test_webhook_secret_key";

    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload);
    let signature = hex::encode(mac.finalize().into_bytes());

    assert!(!verify_signature(
        tampered,
        secret,
        &format!("sha256={}", signature)
    ));
}

#[test]
fn test_verify_signature_wrong_secret() {
    let payload = b"{\"event\":\"message.received\",\"sessionId\":\"s-123\"}";
    let secret = "secret_one";
    let wrong_secret = "secret_two";

    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload);
    let signature = hex::encode(mac.finalize().into_bytes());

    assert!(!verify_signature(payload, wrong_secret, &signature));
}

#[test]
fn test_verify_signature_malformed_header() {
    let payload = b"test";
    let secret = "secret";
    assert!(!verify_signature(payload, secret, "not-valid-hex!"));
    assert!(!verify_signature(payload, secret, "sha256=xyz"));
}

#[test]
fn test_webhook_delivery_envelope_deserialization() {
    let json = r#"{
        "event": "message.sent",
        "sessionId": "sess-uuid-1",
        "timestamp": "2026-09-26T12:00:00Z",
        "data": {
            "messageId": "msg-xyz-99",
            "timestamp": 1758888000
        }
    }"#;

    let delivery: WebhookDelivery<MessageResponse> = serde_json::from_str(json).unwrap();
    assert_eq!(delivery.event, "message.sent");
    assert_eq!(delivery.session_id, "sess-uuid-1");
    assert_eq!(delivery.data.message_id, "msg-xyz-99");
    assert_eq!(delivery.data.timestamp, 1758888000);
}
