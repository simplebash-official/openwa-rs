use serde::{Deserialize, Serialize};

/// Media conversion service status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaConversionStatus {
    pub enabled: bool,
    #[serde(rename = "ffmpegAvailable")]
    pub ffmpeg_available: bool,
}

/// Request to convert media (voice note or video).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaConvertRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
    #[serde(default, skip_serializing)]
    pub mimetype: Option<String>,
}

impl MediaConvertRequest {
    pub fn from_url(url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
            base64: None,
            mimetype: None,
        }
    }

    pub fn from_base64(base64: impl Into<String>) -> Self {
        Self {
            url: None,
            base64: Some(base64.into()),
            mimetype: None,
        }
    }
}

/// Converted media payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertedMedia {
    pub data: String, // base64
    pub mimetype: String,
    pub size: usize,
}
