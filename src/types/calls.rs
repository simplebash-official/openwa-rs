use serde::{Deserialize, Serialize};

/// Request to create a shareable WhatsApp call link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallLinkRequest {
    #[serde(rename = "callType")]
    pub call_type: String, // "audio" or "video"
    /// Start time in epoch MILLISECONDS.
    #[serde(default, rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
}

/// Response containing the created call link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallLinkResponse {
    pub link: String,
}
