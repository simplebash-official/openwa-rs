use serde::{Deserialize, Serialize};

/// Generic success response returned by several OpenWA endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResult {
    pub success: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Participant operation result item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantResult {
    pub id: String,
    pub success: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Batch participant operation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantsResult {
    pub success: bool,
    #[serde(default)]
    pub results: Vec<ParticipantResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Chat or message kind classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatKind {
    Individual,
    Group,
    Channel,
    Status,
    Broadcast,
    Unknown,
}
