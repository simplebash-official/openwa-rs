use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::{ConvertedMedia, MediaConversionStatus, MediaConvertRequest};
use reqwest::Method;
use std::sync::Arc;

/// Media conversion and utilities resource (ffmpeg voice/video conversion).
#[derive(Clone)]
pub struct MediaResource {
    pub(crate) transport: Arc<Transport>,
}

impl MediaResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// Check media conversion engine capability and health.
    pub async fn conversion_status(
        &self,
        session_id: &str,
    ) -> Result<MediaConversionStatus, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/media/convert",
            encode_path_segment(session_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Convert an audio file to WhatsApp voice note format (Ogg/Opus).
    pub async fn convert_voice(
        &self,
        session_id: &str,
        req: MediaConvertRequest,
    ) -> Result<ConvertedMedia, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/media/convert/voice",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Convert a video file to WhatsApp compatible MP4 format.
    pub async fn convert_video(
        &self,
        session_id: &str,
        req: MediaConvertRequest,
    ) -> Result<ConvertedMedia, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/media/convert/video",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }
}
