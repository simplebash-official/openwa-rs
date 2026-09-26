use serde::{Deserialize, Serialize};

/// Ingress URL descriptor for an integration instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressUrl {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Integration instance record (matches OpenWA InstanceView).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationInstance {
    pub id: String,
    #[serde(rename = "pluginId")]
    pub plugin_id: String,
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(default, rename = "sessionScope")]
    pub session_scope: Option<String>,
    pub secret: String,
    #[serde(default, rename = "verifyToken")]
    pub verify_token: Option<String>,
    #[serde(default)]
    pub config: Option<serde_json::Value>,
    pub enabled: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(default, rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(default, rename = "ingressUrls")]
    pub ingress_urls: Option<Vec<IngressUrl>>,
}

/// Request to create an integration instance (matches OpenWA CreateInstanceDto).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIntegrationInstanceRequest {
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(default, rename = "sessionScope", skip_serializing_if = "Option::is_none")]
    pub session_scope: Option<String>,
    #[serde(default, rename = "verifyToken", skip_serializing_if = "Option::is_none")]
    pub verify_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

impl CreateIntegrationInstanceRequest {
    pub fn new(instance_id: impl Into<String>) -> Self {
        Self {
            instance_id: instance_id.into(),
            session_scope: None,
            verify_token: None,
            secret: None,
            config: None,
        }
    }
}

/// Request to update an integration instance (matches OpenWA UpdateInstanceDto).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateIntegrationInstanceRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, rename = "sessionScope", skip_serializing_if = "Option::is_none")]
    pub session_scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

/// Response returned from regenerating instance secret.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegenerateSecretResponse {
    pub secret: String,
}

/// Response returned from redriving dead-lettered integration deliveries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedriveResponse {
    #[serde(default)]
    pub redriven: usize,
    #[serde(default)]
    pub remaining: usize,
    #[serde(default, rename = "batchSize")]
    pub batch_size: usize,
}
