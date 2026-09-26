use serde::{Deserialize, Serialize};

/// Audit log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: String,
    pub action: String,
    #[serde(default, rename = "apiKeyId")]
    pub api_key_id: Option<String>,
    #[serde(default, rename = "sessionId")]
    pub session_id: Option<String>,
    #[serde(default)]
    pub details: Option<serde_json::Value>,
    pub timestamp: String,
}

/// Query parameters for listing audit logs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditLogsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(default, rename = "apiKeyId", skip_serializing_if = "Option::is_none")]
    pub api_key_id: Option<String>,
    #[serde(default, rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}
