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
