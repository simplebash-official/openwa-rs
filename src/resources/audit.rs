use crate::error::OpenWAError;
use crate::transport::Transport;
use crate::types::{AuditLogEntry, AuditLogsQuery};
use reqwest::Method;
use std::sync::Arc;

/// Audit logging resource for administrative tracking (Requires ADMIN role).
#[derive(Clone)]
pub struct AuditResource {
    pub(crate) transport: Arc<Transport>,
}

impl AuditResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List audit logs with pagination and filters.
    pub async fn list(
        &self,
        query: Option<AuditLogsQuery>,
    ) -> Result<Vec<AuditLogEntry>, OpenWAError> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if let Some(ref q) = query {
            if let Some(limit) = q.limit {
                query_params.push(("limit", limit.to_string()));
            }
            if let Some(offset) = q.offset {
                query_params.push(("offset", offset.to_string()));
            }
            if let Some(ref k) = q.api_key_id {
                query_params.push(("apiKeyId", k.clone()));
            }
            if let Some(ref s) = q.session_id {
                query_params.push(("sessionId", s.clone()));
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
            .execute(Method::GET, "/api/audit", q_opt, None)
            .await
    }
}
