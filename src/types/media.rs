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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
}

/// Converted media payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertedMedia {
    pub data: String, // base64
    pub mimetype: String,
    pub size: usize,
}
