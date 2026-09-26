use serde::{Deserialize, Serialize};

/// Session lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Created,
    Initializing,
    QrReady,
    Authenticating,
    Ready,
    Disconnected,
    ActionRequired,
    Failed,
}

/// Restriction placed on the account by WhatsApp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRestriction {
    pub kind: String,
    pub code: String,
    #[serde(default, rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Information about an OpenWA session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResponse {
    pub id: String,
    pub name: String,
    pub status: SessionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, rename = "pushName", skip_serializing_if = "Option::is_none")]
    pub push_name: Option<String>,
    #[serde(
        default,
        rename = "connectedAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub connected_at: Option<String>,
    #[serde(
        default,
        rename = "lastActive",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_active: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(default, rename = "lastError", skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restriction: Option<AccountRestriction>,
    #[serde(default, rename = "engineLoaded")]
    pub engine_loaded: bool,
}

/// Runtime engine configuration of a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    #[serde(rename = "autoRejectCalls")]
    pub auto_reject_calls: bool,
    #[serde(rename = "maxReconnectAttempts")]
    pub max_reconnect_attempts: Option<i32>,
    #[serde(rename = "reconnectBaseDelay")]
    pub reconnect_base_delay: i32,
}

/// Request payload to update session runtime configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSessionConfigRequest {
    #[serde(
        default,
        rename = "autoRejectCalls",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_reject_calls: Option<bool>,
    #[serde(
        default,
        rename = "maxReconnectAttempts",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_reconnect_attempts: Option<i32>,
    #[serde(
        default,
        rename = "reconnectBaseDelay",
        skip_serializing_if = "Option::is_none"
    )]
    pub reconnect_base_delay: Option<i32>,
}

/// Masked proxy configuration for a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionProxy {
    pub enabled: bool,
    #[serde(default, rename = "proxyType", skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
    #[serde(default, rename = "proxyHost", skip_serializing_if = "Option::is_none")]
    pub proxy_host: Option<String>,
    #[serde(rename = "hasCredentials")]
    pub has_credentials: bool,
}

/// Request payload to update session proxy settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSessionProxyRequest {
    /// Proxy URL (e.g. `http://user:pass@host:port`), or null to clear.
    #[serde(rename = "proxyUrl")]
    pub proxy_url: Option<String>,
}

/// Request payload to create a new session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub name: String,
    #[serde(default, rename = "proxyUrl", skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(default, rename = "proxyType", skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
}

impl CreateSessionRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            proxy_url: None,
            proxy_type: None,
        }
    }
}

/// Query parameters for listing sessions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListSessionsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Response containing a base64-encoded QR code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeResponse {
    #[serde(rename = "qrCode")]
    pub qr_code: String,
}

/// Request payload for phone-based pairing code authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingCodeRequest {
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
}

/// Response containing an 8-character pairing code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingCodeResponse {
    pub code: String,
}

/// Request payload to set online presence for the account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetOnlinePresenceRequest {
    pub presence: String, // "available" or "unavailable"
}
