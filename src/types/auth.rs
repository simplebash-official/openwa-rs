use serde::{Deserialize, Serialize};

/// Role privilege hierarchy for an API key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyRole {
    Viewer,
    Operator,
    Admin,
}

/// Validation result of an API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthValidateResponse {
    pub valid: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(
        default,
        rename = "engineType",
        skip_serializing_if = "Option::is_none"
    )]
    pub engine_type: Option<String>,
}

/// Stored API key record (never returns plain key, only prefix).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyRecord {
    pub id: String,
    pub name: String,
    pub role: ApiKeyRole,
    #[serde(rename = "keyPrefix")]
    pub key_prefix: String,
    #[serde(default, rename = "allowedSessions")]
    pub allowed_sessions: Option<Vec<String>>,
    #[serde(default, rename = "allowedIps")]
    pub allowed_ips: Option<Vec<String>>,
    #[serde(default, rename = "allowedChats")]
    pub allowed_chats: Option<Vec<String>>,
    #[serde(default)]
    pub revoked: bool,
    #[serde(default, rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Request to create a new API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    #[serde(default = "default_operator")]
    pub role: ApiKeyRole,
    #[serde(
        default,
        rename = "allowedSessions",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_sessions: Option<Vec<String>>,
    #[serde(
        default,
        rename = "allowedIps",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_ips: Option<Vec<String>>,
    #[serde(
        default,
        rename = "allowedChats",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_chats: Option<Vec<String>>,
    #[serde(default, rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

fn default_operator() -> ApiKeyRole {
    ApiKeyRole::Operator
}

/// Response returned when an API key is minted (the ONLY time the plain key is returned).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyCreatedResponse {
    pub id: String,
    pub name: String,
    pub role: ApiKeyRole,
    pub key: String,
    #[serde(rename = "keyPrefix")]
    pub key_prefix: String,
}

/// Request to update an existing API key.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateApiKeyRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<ApiKeyRole>,
    #[serde(
        default,
        rename = "allowedSessions",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_sessions: Option<Vec<String>>,
    #[serde(
        default,
        rename = "allowedIps",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_ips: Option<Vec<String>>,
    #[serde(
        default,
        rename = "allowedChats",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_chats: Option<Vec<String>>,
    #[serde(default, rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}
