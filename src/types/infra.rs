use serde::{Deserialize, Serialize};

/// Infrastructure health status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfraHealth {
    pub status: String,
    #[serde(default)]
    pub version: Option<String>,
}

/// Infrastructure runtime status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfraStatus {
    pub status: String,
    #[serde(default, rename = "nodeId")]
    pub node_id: Option<String>,
    #[serde(default)]
    pub memory: Option<serde_json::Value>,
}

/// Currently active WhatsApp engine details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentEngineResponse {
    pub engine: String, // "whatsapp-web.js" or "baileys"
    #[serde(default)]
    pub version: Option<String>,
}

/// Information about available engines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineInfo {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub available: bool,
}

/// Gateway update check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckResponse {
    #[serde(rename = "currentVersion")]
    pub current_version: String,
    #[serde(rename = "latestVersion")]
    pub latest_version: String,
    #[serde(rename = "updateAvailable")]
    pub update_available: bool,
}

/// Infrastructure configuration payload for PUT /api/infra/config (SaveConfigDto).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InfraConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redis: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<serde_json::Value>,
}

/// System configuration key-value pairs (alias for InfraConfig).
pub type SaveConfigRequest = InfraConfig;

/// Request payload to restart infrastructure containers.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RestartRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    #[serde(
        default,
        rename = "profilesToRemove",
        skip_serializing_if = "Option::is_none"
    )]
    pub profiles_to_remove: Option<Vec<String>>,
}

/// Restart response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestartResponse {
    pub message: String,
}

/// File storage count statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCount {
    pub count: u64,
    #[serde(default, rename = "totalBytes")]
    pub total_bytes: Option<u64>,
}

/// Request to import infrastructure data tables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDataRequest {
    pub tables: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(
        default,
        rename = "stopOrphans",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_orphans: Option<bool>,
}

/// Request to import storage archive from path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportStorageRequest {
    #[serde(rename = "filePath")]
    pub file_path: String,
}
