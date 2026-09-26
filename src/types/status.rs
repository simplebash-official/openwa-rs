use serde::{Deserialize, Serialize};

/// Contact details associated with a status update.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusContact {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, rename = "pushName", skip_serializing_if = "Option::is_none")]
    pub push_name: Option<String>,
}

/// A contact status / story update record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusRecord {
    pub id: String,
    pub contact: StatusContact,
    #[serde(rename = "type")]
    pub status_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default, rename = "hasMedia")]
    pub has_media: bool,
    /// Epoch milliseconds.
    #[serde(rename = "postedAt")]
    pub posted_at: i64,
    /// Epoch milliseconds.
    #[serde(rename = "expiresAt")]
    pub expires_at: i64,
}

/// Response returned when posting a status update.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResult {
    #[serde(rename = "statusId")]
    pub status_id: String,
    /// ISO-8601 string.
    pub timestamp: String,
    /// ISO-8601 string.
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

/// Request to post a text status update.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTextStatusRequest {
    pub text: String,
    #[serde(
        default,
        rename = "backgroundColor",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<u8>,
}

/// Request to post an image or video status update.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMediaStatusRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

/// Request to post a voice status update (must be Ogg/Opus).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendVoiceStatusRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
}
