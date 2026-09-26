use serde::{Deserialize, Serialize};

/// Integration instance record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationInstance {
    pub id: String,
    #[serde(rename = "pluginId")]
    pub plugin_id: String,
    pub name: String,
    #[serde(default)]
    pub config: serde_json::Value,
    #[serde(default)]
    pub active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Request to create an integration instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIntegrationInstanceRequest {
    pub name: String,
    #[serde(default)]
    pub config: serde_json::Value,
    #[serde(default = "default_true")]
    pub active: bool,
}

fn default_true() -> bool {
    true
}

/// Request to update an integration instance.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateIntegrationInstanceRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// Response returned from regenerating instance secret.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegenerateSecretResponse {
    pub secret: String,
}

/// Response returned from redriving a failed integration event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedriveResponse {
    pub success: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
