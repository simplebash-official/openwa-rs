use serde::{Deserialize, Serialize};

/// Installed plugin record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRecord {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    pub enabled: bool,
    #[serde(default)]
    pub status: Option<String>,
}

/// Plugin catalog listing item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCatalogItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    #[serde(default)]
    pub author: Option<String>,
}

/// Request to install a plugin by package name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallPluginRequest {
    pub package: String,
}

/// Request to install a plugin from a remote URL or tarball.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallPluginUrlRequest {
    pub url: String,
}

/// Plugin configuration schema UI representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfigUi {
    pub schema: serde_json::Value,
    #[serde(default)]
    pub uischema: Option<serde_json::Value>,
}

/// Plugin health diagnostic result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginHealthResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to configure plugin sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePluginSessionsRequest {
    pub sessions: Vec<String>,
}

/// Request to update plugin configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfigRequest {
    pub config: serde_json::Value,
}

impl PluginConfigRequest {
    pub fn new(config: serde_json::Value) -> Self {
        Self { config }
    }
}
