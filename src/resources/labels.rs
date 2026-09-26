use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

/// WhatsApp Business Labels resource.
#[derive(Clone)]
pub struct LabelsResource {
    pub(crate) transport: Arc<Transport>,
}

impl LabelsResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List all labels for a session.
    pub async fn list(&self, session_id: &str) -> Result<Vec<LabelRecord>, OpenWAError> {
        let path = format!("/api/sessions/{}/labels", encode_path_segment(session_id));
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Get a specific label by ID.
    pub async fn get(&self, session_id: &str, label_id: &str) -> Result<LabelRecord, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/labels/{}",
            encode_path_segment(session_id),
            encode_path_segment(label_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Create or update a label.
    pub async fn upsert(
        &self,
        session_id: &str,
        label_id: &str,
        req: UpsertLabelRequest,
    ) -> Result<LabelRecord, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/labels/{}",
            encode_path_segment(session_id),
            encode_path_segment(label_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Delete a label.
    pub async fn delete(
        &self,
        session_id: &str,
        label_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/labels/{}",
            encode_path_segment(session_id),
            encode_path_segment(label_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }

    /// Get all chats associated with a given label.
    pub async fn get_chats(
        &self,
        session_id: &str,
        label_id: &str,
    ) -> Result<Vec<ChatSummary>, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/labels/{}/chats",
            encode_path_segment(session_id),
            encode_path_segment(label_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Get all labels assigned to a chat.
    pub async fn get_chat_labels(
        &self,
        session_id: &str,
        chat_id: &str,
    ) -> Result<Vec<LabelRecord>, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/labels/chat/{}",
            encode_path_segment(session_id),
            encode_path_segment(chat_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Add a label to a chat.
    pub async fn add_label_to_chat(
        &self,
        session_id: &str,
        chat_id: &str,
        req: AddLabelRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/labels/chat/{}",
            encode_path_segment(session_id),
            encode_path_segment(chat_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Remove a label from a chat.
    pub async fn remove_label_from_chat(
        &self,
        session_id: &str,
        chat_id: &str,
        label_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/labels/chat/{}/{}",
            encode_path_segment(session_id),
            encode_path_segment(chat_id),
            encode_path_segment(label_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }
}
