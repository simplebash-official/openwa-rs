use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

/// Quick-reply and HSM Templates resource.
#[derive(Clone)]
pub struct TemplatesResource {
    pub(crate) transport: Arc<Transport>,
}

impl TemplatesResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List all templates defined for a session.
    pub async fn list(&self, session_id: &str) -> Result<Vec<TemplateRecord>, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/templates",
            encode_path_segment(session_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Get a specific template by ID.
    pub async fn get(
        &self,
        session_id: &str,
        template_id: &str,
    ) -> Result<TemplateRecord, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/templates/{}",
            encode_path_segment(session_id),
            encode_path_segment(template_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Create a new template.
    pub async fn create(
        &self,
        session_id: &str,
        req: CreateTemplateRequest,
    ) -> Result<TemplateRecord, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/templates",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Update an existing template.
    pub async fn update(
        &self,
        session_id: &str,
        template_id: &str,
        req: UpdateTemplateRequest,
    ) -> Result<TemplateRecord, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/templates/{}",
            encode_path_segment(session_id),
            encode_path_segment(template_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Delete a template.
    pub async fn delete(
        &self,
        session_id: &str,
        template_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/templates/{}",
            encode_path_segment(session_id),
            encode_path_segment(template_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }
}
