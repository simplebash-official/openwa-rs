use serde::{Deserialize, Serialize};

/// Subscribable event type constants matching OpenWA gateway specifications.
///
/// `message.failed` and `session.reconnect_loop` are webhook-only and intentionally absent;
/// see [`crate::webhook::WEBHOOK_EVENTS`].
pub const SUBSCRIBABLE_EVENTS: &[&str] = &[
    "message.received",
    "message.sent",
    "message.ack",
    "message.revoked",
    "message.reaction",
    "message.edited",
    "session.status",
    "session.qr",
    "session.authenticated",
    "session.disconnected",
    "session.restriction",
    "group.join",
    "group.leave",
    "group.update",
    "group.join_request",
    "call.received",
    "status.received",
    "presence.update",
    "call.accepted",
    "call.rejected",
    "call.missed",
];

/// Request sent by client to subscribe to real-time events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSSubscribeRequest {
    #[serde(rename = "type")]
    pub message_type: String, // "subscribe"
    #[serde(rename = "sessionId")]
    pub session_id: String, // Session ID or "*"
    pub events: Vec<String>, // Event names or ["*"]
    #[serde(default, rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl WSSubscribeRequest {
    pub fn new(session_id: impl Into<String>, events: Vec<String>) -> Self {
        Self {
            message_type: "subscribe".to_string(),
            session_id: session_id.into(),
            events,
            request_id: None,
        }
    }

    pub fn with_request_id(mut self, id: impl Into<String>) -> Self {
        self.request_id = Some(id.into());
        self
    }
}

/// Request sent by client to unsubscribe from events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSUnsubscribeRequest {
    #[serde(rename = "type")]
    pub message_type: String, // "unsubscribe"
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(default, rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl WSUnsubscribeRequest {
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            message_type: "unsubscribe".to_string(),
            session_id: session_id.into(),
            request_id: None,
        }
    }
}

/// Heartbeat ping frame.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSPingRequest {
    #[serde(rename = "type")]
    pub message_type: String, // "ping"
    #[serde(default, rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl Default for WSPingRequest {
    fn default() -> Self {
        Self {
            message_type: "ping".to_string(),
            request_id: None,
        }
    }
}

/// Confirmation of active subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSSubscribedResponse {
    #[serde(default, rename = "type")]
    pub message_type: Option<String>,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub events: Vec<String>,
    #[serde(default, rename = "requestId")]
    pub request_id: Option<String>,
    pub timestamp: String,
}

/// Confirmation of unsubscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSUnsubscribedResponse {
    #[serde(default, rename = "type")]
    pub message_type: Option<String>,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(default, rename = "requestId")]
    pub request_id: Option<String>,
    pub timestamp: String,
}

/// Heartbeat pong response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSPongResponse {
    #[serde(default, rename = "type")]
    pub message_type: Option<String>,
    #[serde(default, rename = "requestId")]
    pub request_id: Option<String>,
    pub timestamp: String,
}

/// Error frame returned by the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSErrorResponse {
    #[serde(default, rename = "type")]
    pub message_type: Option<String>,
    pub code: String,
    pub message: String,
    #[serde(default, rename = "requestId")]
    pub request_id: Option<String>,
    pub timestamp: String,
}

/// Event payload inner data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPayload {
    pub event: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub data: serde_json::Value,
}

/// Live event notification message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSEventMessage {
    #[serde(rename = "type")]
    pub message_type: String, // "event"
    pub payload: EventPayload,
    pub timestamp: String,
}

/// Sum type of all possible messages received from OpenWA event gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WSServerMessage {
    #[serde(rename = "subscribed")]
    Subscribed {
        #[serde(rename = "sessionId")]
        session_id: String,
        events: Vec<String>,
        #[serde(default, rename = "requestId")]
        request_id: Option<String>,
        timestamp: String,
    },
    #[serde(rename = "unsubscribed")]
    Unsubscribed {
        #[serde(rename = "sessionId")]
        session_id: String,
        #[serde(default, rename = "requestId")]
        request_id: Option<String>,
        timestamp: String,
    },
    #[serde(rename = "pong")]
    Pong {
        #[serde(default, rename = "requestId")]
        request_id: Option<String>,
        timestamp: String,
    },
    #[serde(rename = "event")]
    Event {
        payload: EventPayload,
        timestamp: String,
    },
    #[serde(rename = "error")]
    Error {
        code: String,
        message: String,
        #[serde(default, rename = "requestId")]
        request_id: Option<String>,
        timestamp: String,
    },
}
