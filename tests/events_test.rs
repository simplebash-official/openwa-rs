use openwa::events::*;

#[test]
fn test_subscribe_request_serialization() {
    let req = WSSubscribeRequest::new(
        "session-1",
        vec!["message.received".to_string(), "session.status".to_string()],
    )
    .with_request_id("req-42");

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("\"type\":\"subscribe\""));
    assert!(json.contains("\"sessionId\":\"session-1\""));
    assert!(json.contains("\"requestId\":\"req-42\""));
}

#[test]
fn test_server_message_deserialization_event() {
    let raw = r#"{
        "type": "event",
        "payload": {
            "event": "message.received",
            "sessionId": "s-1",
            "data": {
                "id": "true_123@c.us_ABC",
                "body": "Test incoming"
            }
        },
        "timestamp": "2026-09-26T12:00:00.000Z"
    }"#;

    let msg: WSServerMessage = serde_json::from_str(raw).unwrap();
    match msg {
        WSServerMessage::Event { payload, timestamp } => {
            assert_eq!(payload.event, "message.received");
            assert_eq!(payload.session_id, "s-1");
            assert_eq!(payload.data["body"], "Test incoming");
            assert_eq!(timestamp, "2026-09-26T12:00:00.000Z");
        }
        other => panic!("Expected Event variant, got: {:?}", other),
    }
}

#[test]
fn test_server_message_deserialization_subscribed_ack() {
    let raw = r#"{
        "type": "subscribed",
        "sessionId": "*",
        "events": ["*"],
        "requestId": "r-1",
        "timestamp": "2026-09-26T12:00:00.000Z"
    }"#;

    let msg: WSServerMessage = serde_json::from_str(raw).unwrap();
    match msg {
        WSServerMessage::Subscribed {
            session_id,
            events,
            request_id,
            ..
        } => {
            assert_eq!(session_id, "*");
            assert_eq!(events, vec!["*"]);
            assert_eq!(request_id.as_deref(), Some("r-1"));
        }
        other => panic!("Expected Subscribed variant, got: {:?}", other),
    }
}

#[test]
fn test_server_message_deserialization_error() {
    let raw = r#"{
        "type": "error",
        "code": "FORBIDDEN_SESSION",
        "message": "API key cannot subscribe to session *",
        "timestamp": "2026-09-26T12:00:00.000Z"
    }"#;

    let msg: WSServerMessage = serde_json::from_str(raw).unwrap();
    match msg {
        WSServerMessage::Error { code, message, .. } => {
            assert_eq!(code, "FORBIDDEN_SESSION");
            assert_eq!(message, "API key cannot subscribe to session *");
        }
        other => panic!("Expected Error variant, got: {:?}", other),
    }
}
