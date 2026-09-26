use serde::{Deserialize, Serialize};

/// High-level session statistics overview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatsOverview {
    pub total: usize,
    pub active: usize,
    pub ready: usize,
    pub disconnected: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<usize>,
}

/// Message volume statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStats {
    pub total: u64,
    pub sent: u64,
    pub received: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivered: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<u64>,
}

/// Specific session statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStats {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub messages: MessageStats,
}
