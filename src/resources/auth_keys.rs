use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

/// Management of API keys and credential validation (Requires ADMIN role).
#[derive(Clone)]
pub struct AuthKeysResource {
    pub(crate) transport: Arc<Transport>,
}

impl AuthKeysResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List all API keys.
    pub async fn list(&self) -> Result<Vec<ApiKeyRecord>, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/auth/api-keys", None, None)
            .await
    }

    /// Get a specific API key by ID.
    pub async fn get(&self, id: &str) -> Result<ApiKeyRecord, OpenWAError> {
        let path = format!("/api/auth/api-keys/{}", encode_path_segment(id));
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Create a new API key.
    pub async fn create(
        &self,
        req: CreateApiKeyRequest,
    ) -> Result<ApiKeyCreatedResponse, OpenWAError> {
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, "/api/auth/api-keys", None, Some(body))
            .await
    }

    /// Update an existing API key.
    pub async fn update(
        &self,
        id: &str,
        req: UpdateApiKeyRequest,
    ) -> Result<ApiKeyRecord, OpenWAError> {
        let path = format!("/api/auth/api-keys/{}", encode_path_segment(id));
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Permanently delete an API key.
    pub async fn delete(&self, id: &str) -> Result<SuccessResult, OpenWAError> {
        let path = format!("/api/auth/api-keys/{}", encode_path_segment(id));
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }

    /// Revoke an API key immediately without deleting record.
    pub async fn revoke(&self, id: &str) -> Result<SuccessResult, OpenWAError> {
        let path = format!("/api/auth/api-keys/{}/revoke", encode_path_segment(id));
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Validate the currently configured API key and retrieve role privileges.
    pub async fn validate(&self) -> Result<AuthValidateResponse, OpenWAError> {
        self.transport
            .execute(Method::POST, "/api/auth/validate", None, None)
            .await
    }
}
