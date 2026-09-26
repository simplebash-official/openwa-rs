use serde::{Deserialize, Serialize};

/// System health response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uptime: Option<f64>,
}

/// Liveness and readiness probe response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthProbeResponse {
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}
