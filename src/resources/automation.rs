use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

/// Session automation rules (auto-replies, keyword triggers).
#[derive(Clone)]
pub struct AutomationResource {
    pub(crate) transport: Arc<Transport>,
}

impl AutomationResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List all automation rules configured for a session.
    pub async fn list(&self, session_id: &str) -> Result<Vec<AutomationRule>, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/automation-rules",
            encode_path_segment(session_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Get a specific automation rule by ID.
    pub async fn get(
        &self,
        session_id: &str,
        rule_id: &str,
    ) -> Result<AutomationRule, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/automation-rules/{}",
            encode_path_segment(session_id),
            encode_path_segment(rule_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Create a new automation rule for a session.
    pub async fn create(
        &self,
        session_id: &str,
        req: CreateAutomationRuleRequest,
    ) -> Result<AutomationRule, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/automation-rules",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Update an existing automation rule.
    pub async fn update(
        &self,
        session_id: &str,
        rule_id: &str,
        req: UpdateAutomationRuleRequest,
    ) -> Result<AutomationRule, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/automation-rules/{}",
            encode_path_segment(session_id),
            encode_path_segment(rule_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Delete an automation rule.
    pub async fn delete(
        &self,
        session_id: &str,
        rule_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/automation-rules/{}",
            encode_path_segment(session_id),
            encode_path_segment(rule_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }
}
