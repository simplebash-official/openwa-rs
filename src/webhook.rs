use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use subtle::ConstantTimeEq;

type HmacSha256 = Hmac<Sha256>;

/// Verifies the HMAC-SHA256 signature from the `X-OpenWA-Signature` header against the raw request body.
///
/// Returns true if the signature is valid, false otherwise.
/// The comparison is performed in constant time to prevent timing attacks.
pub fn verify_signature(raw_body: &[u8], secret: &str, signature_header: &str) -> bool {
    if signature_header.is_empty() || secret.is_empty() {
        return false;
    }

    let signature_hex = signature_header
        .strip_prefix("sha256=")
        .unwrap_or(signature_header);

    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(raw_body);
    let expected_bytes = mac.finalize().into_bytes();
    let expected_hex = hex::encode(expected_bytes);

    if signature_hex.len() != expected_hex.len() {
        return false;
    }

    signature_hex
        .as_bytes()
        .ct_eq(expected_hex.as_bytes())
        .into()
}

/// Standard envelope for all webhook deliveries from OpenWA.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDelivery<T = serde_json::Value> {
    pub event: String,
    pub timestamp: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(
        default,
        rename = "idempotencyKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub idempotency_key: Option<String>,
    #[serde(
        default,
        rename = "deliveryId",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_id: Option<String>,
    pub data: T,
}

/// Every event a webhook may subscribe to (the `events` list of a webhook), plus the `*` wildcard.
///
/// Unlike the WebSocket list (`events::SUBSCRIBABLE_EVENTS`), this includes the
/// webhook-only events `message.failed` and `session.reconnect_loop`.
pub const WEBHOOK_EVENTS: &[&str] = &[
    "message.received",
    "message.sent",
    "message.ack",
    "message.failed",
    "message.revoked",
    "message.reaction",
    "message.edited",
    "session.status",
    "session.qr",
    "session.authenticated",
    "session.disconnected",
    "session.reconnect_loop",
    "session.restriction",
    "presence.update",
    "call.accepted",
    "call.rejected",
    "call.missed",
    "group.join",
    "group.leave",
    "group.update",
    "group.join_request",
    "call.received",
    "status.received",
];

/// Typed webhook event name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WebhookEvent {
    #[serde(rename = "message.received")]
    MessageReceived,
    #[serde(rename = "message.sent")]
    MessageSent,
    #[serde(rename = "message.ack")]
    MessageAck,
    #[serde(rename = "message.failed")]
    MessageFailed,
    #[serde(rename = "message.revoked")]
    MessageRevoked,
    #[serde(rename = "message.reaction")]
    MessageReaction,
    #[serde(rename = "message.edited")]
    MessageEdited,
    #[serde(rename = "session.status")]
    SessionStatus,
    #[serde(rename = "session.qr")]
    SessionQr,
    #[serde(rename = "session.authenticated")]
    SessionAuthenticated,
    #[serde(rename = "session.disconnected")]
    SessionDisconnected,
    #[serde(rename = "session.reconnect_loop")]
    SessionReconnectLoop,
    #[serde(rename = "session.restriction")]
    SessionRestriction,
    #[serde(rename = "presence.update")]
    PresenceUpdate,
    #[serde(rename = "call.accepted")]
    CallAccepted,
    #[serde(rename = "call.rejected")]
    CallRejected,
    #[serde(rename = "call.missed")]
    CallMissed,
    #[serde(rename = "group.join")]
    GroupJoin,
    #[serde(rename = "group.leave")]
    GroupLeave,
    #[serde(rename = "group.update")]
    GroupUpdate,
    #[serde(rename = "group.join_request")]
    GroupJoinRequest,
    #[serde(rename = "call.received")]
    CallReceived,
    #[serde(rename = "status.received")]
    StatusReceived,
    /// The `*` wildcard (all events).
    #[serde(rename = "*")]
    All,
}

impl WebhookEvent {
    /// Wire name of the event, e.g. `"message.failed"`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MessageReceived => "message.received",
            Self::MessageSent => "message.sent",
            Self::MessageAck => "message.ack",
            Self::MessageFailed => "message.failed",
            Self::MessageRevoked => "message.revoked",
            Self::MessageReaction => "message.reaction",
            Self::MessageEdited => "message.edited",
            Self::SessionStatus => "session.status",
            Self::SessionQr => "session.qr",
            Self::SessionAuthenticated => "session.authenticated",
            Self::SessionDisconnected => "session.disconnected",
            Self::SessionReconnectLoop => "session.reconnect_loop",
            Self::SessionRestriction => "session.restriction",
            Self::PresenceUpdate => "presence.update",
            Self::CallAccepted => "call.accepted",
            Self::CallRejected => "call.rejected",
            Self::CallMissed => "call.missed",
            Self::GroupJoin => "group.join",
            Self::GroupLeave => "group.leave",
            Self::GroupUpdate => "group.update",
            Self::GroupJoinRequest => "group.join_request",
            Self::CallReceived => "call.received",
            Self::StatusReceived => "status.received",
            Self::All => "*",
        }
    }
}

impl std::fmt::Display for WebhookEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for WebhookEvent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .map_err(|_| format!("unknown webhook event: {s}"))
    }
}

impl From<WebhookEvent> for String {
    fn from(e: WebhookEvent) -> Self {
        e.as_str().to_string()
    }
}

impl<T> WebhookDelivery<T> {
    /// Parses [`Self::event`] into a [`WebhookEvent`]; `None` for names this SDK does not know yet.
    pub fn event_kind(&self) -> Option<WebhookEvent> {
        self.event.parse().ok()
    }
}

/// System headers attached to webhook deliveries.
#[derive(Debug, Clone, Default)]
pub struct WebhookHeaders {
    pub signature: Option<String>,
    pub event: Option<String>,
    pub idempotency_key: Option<String>,
    pub delivery_id: Option<String>,
    pub retry_count: Option<u32>,
}

impl WebhookHeaders {
    pub fn from_headers<F>(mut get_header: F) -> Self
    where
        F: FnMut(&str) -> Option<String>,
    {
        Self {
            signature: get_header("x-openwa-signature")
                .or_else(|| get_header("X-OpenWA-Signature")),
            event: get_header("x-openwa-event").or_else(|| get_header("X-OpenWA-Event")),
            idempotency_key: get_header("x-openwa-idempotency-key")
                .or_else(|| get_header("X-OpenWA-Idempotency-Key")),
            delivery_id: get_header("x-openwa-delivery-id")
                .or_else(|| get_header("X-OpenWA-Delivery-Id")),
            retry_count: get_header("x-openwa-retry-count")
                .or_else(|| get_header("X-OpenWA-Retry-Count"))
                .and_then(|s| s.parse().ok()),
        }
    }
}

/// `message.ack` delivery event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAckData {
    pub id: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub status: String, // "pending", "sent", "delivered", "read", "failed"
    pub ack: i32,
}

/// `session.qr` delivery event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionQrData {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// PNG data URL (data:image/png;base64,...).
    pub qr: String,
}

/// `session.status` delivery event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatusData {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub status: crate::types::SessionStatus,
}

/// `session.authenticated` delivery event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAuthenticatedData {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub phone: String,
    #[serde(default, rename = "pushName", skip_serializing_if = "Option::is_none")]
    pub push_name: Option<String>,
}

/// `session.disconnected` delivery event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionDisconnectedData {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub reason: String,
}

/// `group.join` and `group.leave` delivery event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMembershipChangeData {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(default, rename = "actorId", skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    #[serde(rename = "participantIds")]
    pub participant_ids: Vec<String>,
    pub timestamp: i64,
}

/// `group.update` delivery event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupUpdateData {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(default, rename = "actorId", skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    #[serde(default, rename = "participantIds")]
    pub participant_ids: Vec<String>,
    /// Only the fields that changed: `subject`, `description`, `announce`, `locked`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<GroupChanges>,
    pub timestamp: i64,
}

/// Changed group metadata carried by `group.update`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub announce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// `group.join_request` delivery event data payload.
///
/// `participant_ids` are the users asking to join.
pub type GroupJoinRequestData = GroupMembershipChangeData;

/// `message.failed` carries the same payload as `message.ack` with `status == "failed"`.
pub type MessageFailedData = MessageAckData;

/// `session.reconnect_loop` delivery event data payload.
///
/// Dispatched on every 5th consecutive reconnect attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReconnectLoopData {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub attempts: u32,
    #[serde(rename = "nextDelayMs")]
    pub next_delay_ms: u64,
}

/// `session.restriction` delivery event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRestrictionData {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// `false` when a restriction is lifted.
    pub active: bool,
    pub kind: String,
    #[serde(default)]
    pub code: Option<String>,
    /// ISO timestamp or `null`.
    #[serde(default, rename = "expiresAt")]
    pub expires_at: Option<String>,
}

/// `message.revoked` delivery event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRevokedData {
    pub id: String,
    /// Id of the original deleted message; reconcile on this, falling back to `id`.
    #[serde(default, rename = "revokedId", skip_serializing_if = "Option::is_none")]
    pub revoked_id: Option<String>,
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(default)]
    pub body: String,
    pub timestamp: i64,
}

/// `message.reaction` delivery event data payload. `reaction` is empty when removed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReactionData {
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub reaction: String,
    #[serde(rename = "senderId")]
    pub sender_id: String,
    /// Post-apply `{ senderId: emoji }` snapshot; absent means unknown, not empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<std::collections::HashMap<String, String>>,
}

/// `message.edited` delivery event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEditedData {
    /// Original message id.
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub body: String,
    #[serde(rename = "senderId")]
    pub sender_id: String,
    pub from: String,
    pub to: String,
    #[serde(rename = "fromMe")]
    pub from_me: bool,
    #[serde(rename = "isGroup")]
    pub is_group: bool,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(rename = "hasMedia")]
    pub has_media: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(
        default,
        rename = "mentionedIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub mentioned_ids: Option<Vec<String>>,
    /// Edit time, epoch seconds.
    pub timestamp: i64,
}

/// One participant entry of `presence.update`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceParticipant {
    pub id: String,
    /// `available`, `unavailable`, `composing`, `recording` or `paused`.
    pub state: String,
    /// Epoch seconds; absent when the contact hides it.
    #[serde(default, rename = "lastSeen", skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<i64>,
}

/// `presence.update` delivery event data payload (Baileys only).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceUpdateData {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub participants: Vec<PresenceParticipant>,
    #[serde(
        default,
        rename = "groupOnlineCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_online_count: Option<u32>,
}

/// `call.received` delivery event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallReceivedData {
    #[serde(rename = "callId")]
    pub call_id: String,
    pub from: String,
    #[serde(rename = "isVideo")]
    pub is_video: bool,
    #[serde(rename = "isGroup")]
    pub is_group: bool,
    pub timestamp: i64,
}

/// `call.accepted`, `call.rejected` and `call.missed` delivery event data payload (Baileys only).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallOutcomeData {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "callId")]
    pub call_id: String,
    pub from: String,
    pub outcome: String,
    #[serde(rename = "isVideo")]
    pub is_video: bool,
    #[serde(rename = "isGroup")]
    pub is_group: bool,
    pub timestamp: i64,
}

/// Contact block of `status.received`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusContact {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, rename = "pushName", skip_serializing_if = "Option::is_none")]
    pub push_name: Option<String>,
}

/// `status.received` delivery event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusReceivedData {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "statusId")]
    pub status_id: String,
    pub contact: StatusContact,
    #[serde(rename = "type")]
    pub status_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(rename = "hasMedia")]
    pub has_media: bool,
    #[serde(default, rename = "mediaOmitted")]
    pub media_omitted: bool,
    #[serde(
        default,
        rename = "omitReason",
        skip_serializing_if = "Option::is_none"
    )]
    pub omit_reason: Option<String>,
    /// Epoch milliseconds.
    #[serde(rename = "postedAt")]
    pub posted_at: i64,
    /// Epoch milliseconds.
    #[serde(rename = "expiresAt")]
    pub expires_at: i64,
}

#[cfg(feature = "axum")]
pub mod axum_support {
    use super::*;
    use ::axum::{
        async_trait,
        body::Bytes,
        extract::{FromRequest, Request},
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use serde::de::DeserializeOwned;

    /// Extension wrapper for providing the webhook secret to the Axum extractor.
    #[derive(Debug, Clone)]
    pub struct WebhookSecret(pub String);

    impl WebhookSecret {
        pub fn new(secret: impl Into<String>) -> Self {
            Self(secret.into())
        }
    }

    /// Strongly-typed Axum extractor for OpenWA webhooks.
    ///
    /// Automatically verifies the HMAC-SHA256 signature if [`WebhookSecret`] is present
    /// in the request extensions, and deserializes the body into [`WebhookDelivery<T>`].
    #[derive(Debug, Clone)]
    pub struct OpenWAWebhook<T = serde_json::Value>(pub WebhookDelivery<T>);

    #[async_trait]
    impl<S, T> FromRequest<S> for OpenWAWebhook<T>
    where
        S: Send + Sync,
        T: DeserializeOwned + Send,
    {
        type Rejection = Response;

        async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
            let secret = req.extensions().get::<WebhookSecret>().map(|s| s.0.clone());
            let signature = req
                .headers()
                .get("x-openwa-signature")
                .or_else(|| req.headers().get("X-OpenWA-Signature"))
                .and_then(|v| v.to_str().ok())
                .map(String::from);

            let bytes = Bytes::from_request(req, state)
                .await
                .map_err(IntoResponse::into_response)?;

            if let Some(secret_str) = secret {
                let sig = signature.as_deref().unwrap_or_default();
                if !verify_signature(&bytes, &secret_str, sig) {
                    return Err(
                        (StatusCode::UNAUTHORIZED, "Invalid OpenWA webhook signature")
                            .into_response(),
                    );
                }
            }

            let delivery: WebhookDelivery<T> = serde_json::from_slice(&bytes).map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Malformed webhook payload: {}", e),
                )
                    .into_response()
            })?;

            Ok(OpenWAWebhook(delivery))
        }
    }
}

#[cfg(feature = "axum")]
pub use axum_support::{OpenWAWebhook, WebhookSecret};
