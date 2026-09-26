use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

#[derive(Clone)]
pub struct WebhooksResource {
    pub(crate) transport: Arc<Transport>,
}

impl WebhooksResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List all webhooks for a specific session. (Requires OPERATOR role).
    pub async fn list(&self, session_id: &str) -> Result<Vec<WebhookRecord>, OpenWAError> {
        let path = format!("/api/sessions/{}/webhooks", encode_path_segment(session_id));
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Get a single webhook by ID. (Requires OPERATOR role).
    pub async fn get(
        &self,
        session_id: &str,
        webhook_id: &str,
    ) -> Result<WebhookRecord, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/webhooks/{}",
            encode_path_segment(session_id),
            encode_path_segment(webhook_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Create a new webhook for a session. (Requires OPERATOR role).
    pub async fn create(
        &self,
        session_id: &str,
        req: CreateWebhookRequest,
    ) -> Result<WebhookRecord, OpenWAError> {
        let path = format!("/api/sessions/{}/webhooks", encode_path_segment(session_id));
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Update an existing webhook. (Requires OPERATOR role).
    pub async fn update(
        &self,
        session_id: &str,
        webhook_id: &str,
        req: UpdateWebhookRequest,
    ) -> Result<WebhookRecord, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/webhooks/{}",
            encode_path_segment(session_id),
            encode_path_segment(webhook_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Delete a webhook. (Requires OPERATOR role).
    pub async fn delete(&self, session_id: &str, webhook_id: &str) -> Result<(), OpenWAError> {
        let path = format!(
            "/api/sessions/{}/webhooks/{}",
            encode_path_segment(session_id),
            encode_path_segment(webhook_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }

    /// Send a test event to the webhook. (Requires OPERATOR role).
    pub async fn test(
        &self,
        session_id: &str,
        webhook_id: &str,
    ) -> Result<WebhookTestResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/webhooks/{}/test",
            encode_path_segment(session_id),
            encode_path_segment(webhook_id)
        );
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// List all webhooks across every session visible to the key. (Requires OPERATOR role).
    pub async fn list_all(&self) -> Result<Vec<WebhookRecord>, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/webhooks", None, None)
            .await
    }

    /// List attempted and failed deliveries for diagnostic troubleshooting. (Requires ADMIN role).
    pub async fn delivery_failures(
        &self,
        query: Option<DeliveryFailureQuery>,
    ) -> Result<Vec<WebhookDeliveryFailure>, OpenWAError> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if let Some(ref q) = query {
            if let Some(limit) = q.limit {
                query_params.push(("limit", limit.to_string()));
            }
            if let Some(offset) = q.offset {
                query_params.push(("offset", offset.to_string()));
            }
            if let Some(ref sid) = q.session_id {
                query_params.push(("sessionId", sid.clone()));
            }
            if let Some(ref wid) = q.webhook_id {
                query_params.push(("webhookId", wid.clone()));
            }
        }
        let q_refs: Vec<(&str, &str)> =
            query_params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let q_opt = if q_refs.is_empty() {
            None
        } else {
            Some(q_refs.as_slice())
        };

        self.transport
            .execute(Method::GET, "/api/webhooks/delivery-failures", q_opt, None)
            .await
    }
}
