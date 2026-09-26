use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

/// WhatsApp Status (Stories) resource.
#[derive(Clone)]
pub struct StatusResource {
    pub(crate) transport: Arc<Transport>,
}

impl StatusResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List all active statuses received from contacts.
    pub async fn list(&self, session_id: &str) -> Result<Vec<StatusRecord>, OpenWAError> {
        let path = format!("/api/sessions/{}/status", encode_path_segment(session_id));
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Get a specific status by status ID.
    pub async fn get(
        &self,
        session_id: &str,
        status_id: &str,
    ) -> Result<StatusRecord, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/status/{}",
            encode_path_segment(session_id),
            encode_path_segment(status_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Delete a previously posted status by ID.
    pub async fn delete(
        &self,
        session_id: &str,
        status_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/status/{}",
            encode_path_segment(session_id),
            encode_path_segment(status_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }

    /// Download media associated with a status update.
    pub async fn get_media(
        &self,
        session_id: &str,
        status_id: &str,
    ) -> Result<Vec<u8>, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/status/{}/media",
            encode_path_segment(session_id),
            encode_path_segment(status_id)
        );
        self.transport
            .execute_raw(Method::GET, &path, None, None)
            .await
    }

    /// Post a text status update.
    pub async fn send_text(
        &self,
        session_id: &str,
        req: SendTextStatusRequest,
    ) -> Result<StatusResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/status/send-text",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Post an image status update.
    pub async fn send_image(
        &self,
        session_id: &str,
        req: SendMediaStatusRequest,
    ) -> Result<StatusResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/status/send-image",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Post a video status update.
    pub async fn send_video(
        &self,
        session_id: &str,
        req: SendMediaStatusRequest,
    ) -> Result<StatusResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/status/send-video",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Post a voice note status update (Ogg/Opus).
    pub async fn send_voice(
        &self,
        session_id: &str,
        req: SendVoiceStatusRequest,
    ) -> Result<StatusResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/status/send-voice",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }
}
