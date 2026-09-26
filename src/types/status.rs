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

/// Media payload for status updates (URL or base64 with mimetype).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusMediaInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
}

impl StatusMediaInput {
    pub fn from_url(url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
            base64: None,
            mimetype: None,
        }
    }

    pub fn from_base64(base64: impl Into<String>, mimetype: impl Into<String>) -> Self {
        Self {
            url: None,
            base64: Some(base64.into()),
            mimetype: Some(mimetype.into()),
        }
    }

    /// Construct a status media payload from a local file asynchronously.
    pub async fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, std::io::Error> {
        let (b64, mime, _) = crate::types::media::read_file_as_base64(path).await?;
        Ok(Self {
            url: None,
            base64: Some(b64),
            mimetype: Some(mime),
        })
    }
}

/// Request to post an image status update.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendImageStatusRequest {
    pub image: StatusMediaInput,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
}

/// Request to post a video status update.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendVideoStatusRequest {
    pub video: StatusMediaInput,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
}

/// Request to post an image or video status update (generic helper).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMediaStatusRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<StatusMediaInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<StatusMediaInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
}

/// Request to post a voice status update (must be Ogg/Opus).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendVoiceStatusRequest {
    pub audio: StatusMediaInput,
    #[serde(
        default,
        rename = "backgroundColor",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
}
