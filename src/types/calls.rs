use serde::{Deserialize, Serialize};

/// Request to create a shareable WhatsApp call link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallLinkRequest {
    #[serde(rename = "type", alias = "callType")]
    pub call_type: String, // "audio" or "video"
    /// Start time in epoch MILLISECONDS.
    #[serde(rename = "startTime")]
    pub start_time: i64,
}

impl CallLinkRequest {
    pub fn new(call_type: impl Into<String>, start_time: i64) -> Self {
        Self {
            call_type: call_type.into(),
            start_time,
        }
    }
}

/// Response containing the created call link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallLinkResponse {
    pub link: String,
}
