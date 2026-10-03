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

#[test]
fn test_webhook_event_catalog() {
    use openwa::webhook::{WebhookEvent, WEBHOOK_EVENTS};
    use std::collections::HashSet;

    assert_eq!(WEBHOOK_EVENTS.len(), 23);
    assert_eq!(
        WEBHOOK_EVENTS.iter().collect::<HashSet<_>>().len(),
        WEBHOOK_EVENTS.len()
    );
    for name in WEBHOOK_EVENTS.iter().copied().chain(["*"]) {
        let ev: WebhookEvent = name.parse().unwrap();
        assert_eq!(ev.as_str(), name);
        assert_eq!(serde_json::to_string(&ev).unwrap(), format!("\"{name}\""));
        assert_eq!(String::from(ev), name);
    }
    assert!("nope".parse::<WebhookEvent>().is_err());
}

#[test]
fn test_ws_events_are_webhook_events_minus_webhook_only() {
    use openwa::events::SUBSCRIBABLE_EVENTS;
    use openwa::webhook::WEBHOOK_EVENTS;
    use std::collections::HashSet;

    let ws: HashSet<_> = SUBSCRIBABLE_EVENTS.iter().collect();
    let wh: HashSet<_> = WEBHOOK_EVENTS
        .iter()
        .filter(|e| **e != "message.failed" && **e != "session.reconnect_loop")
        .collect();
    assert_eq!(ws, wh);
}

#[test]
fn test_new_payloads_deserialize() {
    use openwa::webhook::*;
    use serde_json::json;

    let d: WebhookDelivery<SessionReconnectLoopData> = serde_json::from_value(json!({
        "event": "session.reconnect_loop", "timestamp": "t", "sessionId": "s",
        "data": {"sessionId": "s", "attempts": 5, "nextDelayMs": 30000}
    }))
    .unwrap();
    assert_eq!(d.event_kind(), Some(WebhookEvent::SessionReconnectLoop));
    assert_eq!(d.data.attempts, 5);

    let d: WebhookDelivery<MessageFailedData> = serde_json::from_value(json!({
        "event": "message.failed", "timestamp": "t", "sessionId": "s",
        "data": {"id": "a", "messageId": "b", "status": "failed", "ack": -1}
    }))
    .unwrap();
    assert_eq!(d.event_kind(), Some(WebhookEvent::MessageFailed));
    assert_eq!(d.data.ack, -1);

    serde_json::from_value::<MessageRevokedData>(json!({
        "id": "i", "chatId": "c", "from": "f", "to": "t", "type": "revoked", "body": "", "timestamp": 1
    })).unwrap();
    let r: MessageReactionData = serde_json::from_value(json!({
        "messageId": "m", "chatId": "c", "reaction": "", "senderId": "s"
    }))
    .unwrap();
    assert!(r.reactions.is_none());
    serde_json::from_value::<MessageEditedData>(json!({
        "messageId": "m", "chatId": "c", "body": "b", "senderId": "s", "from": "f", "to": "t",
        "fromMe": false, "isGroup": false, "type": "chat", "hasMedia": false, "timestamp": 1
    }))
    .unwrap();
    let p: PresenceUpdateData = serde_json::from_value(json!({
        "sessionId": "s", "chatId": "c",
        "participants": [{"id": "p", "state": "composing"}]
    }))
    .unwrap();
    assert!(p.participants[0].last_seen.is_none());
    let r: SessionRestrictionData = serde_json::from_value(json!({
        "sessionId": "s", "active": false, "kind": "k", "code": null, "expiresAt": null
    }))
    .unwrap();
    assert!(!r.active);
    let g: GroupUpdateData = serde_json::from_value(json!({
        "groupId": "g", "participantIds": [], "changes": {"subject": "x", "locked": true}, "timestamp": 1
    })).unwrap();
    assert_eq!(g.changes.unwrap().subject.as_deref(), Some("x"));
    serde_json::from_value::<GroupJoinRequestData>(json!({
        "groupId": "g", "participantIds": ["u"], "timestamp": 1
    }))
    .unwrap();
    serde_json::from_value::<CallReceivedData>(json!({
        "callId": "c", "from": "f", "isVideo": true, "isGroup": false, "timestamp": 1
    }))
    .unwrap();
    serde_json::from_value::<CallOutcomeData>(json!({
        "sessionId": "s", "callId": "c", "from": "f", "outcome": "missed",
        "isVideo": false, "isGroup": false, "timestamp": 1
    }))
    .unwrap();
    serde_json::from_value::<StatusReceivedData>(json!({
        "sessionId": "s", "statusId": "x", "contact": {"id": "c"}, "type": "image",
        "hasMedia": true, "mediaOmitted": false, "postedAt": 1, "expiresAt": 2
    }))
    .unwrap();
}
