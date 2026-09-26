use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

/// Operational metrics and usage statistics (Requires ADMIN role).
#[derive(Clone)]
pub struct StatsResource {
    pub(crate) transport: Arc<Transport>,
}

impl StatsResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// Overall statistics of sessions, engines, and gateway uptime.
    pub async fn overview(&self) -> Result<SessionStatsOverview, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/stats/overview", None, None)
            .await
    }

    /// Global message throughput counts and queues.
    pub async fn message_stats(&self) -> Result<MessageStats, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/stats/messages", None, None)
            .await
    }

    /// Session-specific statistics and counters.
    pub async fn session_stats(&self, session_id: &str) -> Result<SessionStats, OpenWAError> {
        let path = format!("/api/stats/sessions/{}", encode_path_segment(session_id));
        self.transport.execute(Method::GET, &path, None, None).await
    }
}
