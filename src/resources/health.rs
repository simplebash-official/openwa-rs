use crate::error::OpenWAError;
use crate::transport::Transport;
use crate::types::{HealthProbeResponse, HealthResponse};
use reqwest::Method;
use std::sync::Arc;

/// Service health and probe resource.
#[derive(Clone)]
pub struct HealthResource {
    pub(crate) transport: Arc<Transport>,
}

impl HealthResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// Comprehensive system health check.
    pub async fn check(&self) -> Result<HealthResponse, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/health", None, None)
            .await
    }

    /// Kubernetes / Docker liveness probe.
    pub async fn liveness(&self) -> Result<HealthProbeResponse, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/health/live", None, None)
            .await
    }

    /// Kubernetes / Docker readiness probe.
    pub async fn readiness(&self) -> Result<HealthProbeResponse, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/health/ready", None, None)
            .await
    }
}
