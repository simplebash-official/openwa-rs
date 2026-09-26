use crate::error::OpenWAError;
use crate::transport::Transport;
use reqwest::Method;
use std::sync::Arc;

/// Prometheus metrics scraping resource.
#[derive(Clone)]
pub struct MetricsResource {
    pub(crate) transport: Arc<Transport>,
}

impl MetricsResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// Scrape Prometheus metrics in standard exposition text format.
    pub async fn scrape(&self) -> Result<String, OpenWAError> {
        let bytes = self
            .transport
            .execute_raw(Method::GET, "/api/metrics", None, None)
            .await?;
        String::from_utf8(bytes).map_err(|e| OpenWAError::Decoding(e.to_string()))
    }
}
