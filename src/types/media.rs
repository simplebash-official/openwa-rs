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

    /// Construct a conversion request by reading a local file asynchronously.
    pub async fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, std::io::Error> {
        let (b64, _, _) = read_file_as_base64(path).await?;
        Ok(Self {
            url: None,
            base64: Some(b64),
            mimetype: None,
        })
    }
}

/// Converted media payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertedMedia {
    pub data: String, // base64
    pub mimetype: String,
    pub size: usize,
}

/// Guess the standard MIME type from a file path based on its extension.
pub fn guess_mime_type(path: &std::path::Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("bmp") => "image/bmp",
        Some("mp4") => "video/mp4",
        Some("3gp") => "video/3gpp",
        Some("mov") => "video/quicktime",
        Some("mkv") => "video/x-matroska",
        Some("webm") => "video/webm",
        Some("mp3") => "audio/mpeg",
        Some("ogg") | Some("opus") => "audio/ogg; codecs=opus",
        Some("wav") => "audio/wav",
        Some("m4a") => "audio/m4a",
        Some("aac") => "audio/aac",
        Some("pdf") => "application/pdf",
        Some("doc") => "application/msword",
        Some("docx") => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        Some("xls") => "application/vnd.ms-excel",
        Some("xlsx") => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        Some("ppt") => "application/vnd.ms-powerpoint",
        Some("pptx") => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        Some("txt") => "text/plain",
        Some("csv") => "text/csv",
        Some("json") => "application/json",
        Some("zip") => "application/zip",
        _ => "application/octet-stream",
    }
}

/// Read a file from disk asynchronously and return `(base64_data, mime_type, filename)`.
pub async fn read_file_as_base64(
    path: impl AsRef<std::path::Path>,
) -> Result<(String, String, String), std::io::Error> {
    use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
    use base64::Engine;

    let path_ref = path.as_ref();
    let bytes = tokio::fs::read(path_ref).await?;
    let b64 = BASE64_STANDARD.encode(&bytes);
    let mime = guess_mime_type(path_ref).to_string();
    let filename = path_ref
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file")
        .to_string();
    Ok((b64, mime, filename))
}
