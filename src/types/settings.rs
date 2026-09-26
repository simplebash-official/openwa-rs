use serde::{Deserialize, Serialize};

/// System runtime settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemSettings {
    #[serde(default)]
    pub settings: serde_json::Value,
}
