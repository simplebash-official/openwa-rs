use crate::error::OpenWAError;
use crate::transport::Transport;
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

/// Infrastructure and server lifecycle resource (Requires ADMIN role).
#[derive(Clone)]
pub struct InfraResource {
    pub(crate) transport: Arc<Transport>,
}

impl InfraResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// Get overall infrastructure status and memory usage.
    pub async fn get_status(&self) -> Result<InfraStatus, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/infra/status", None, None)
            .await
    }

    /// List all available WhatsApp engines.
    pub async fn get_engines(&self) -> Result<Vec<EngineInfo>, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/infra/engines", None, None)
            .await
    }

    /// Get currently active engine information.
    pub async fn get_current_engine(&self) -> Result<CurrentEngineResponse, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/infra/engines/current", None, None)
            .await
    }

    /// Check if a newer version of OpenWA Gateway is available.
    pub async fn check_updates(&self) -> Result<UpdateCheckResponse, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/infra/update-check", None, None)
            .await
    }

    /// Check infrastructure components health.
    pub async fn check_health(&self) -> Result<InfraHealth, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/infra/health", None, None)
            .await
    }

    /// Retrieve runtime configuration.
    pub async fn get_config(&self) -> Result<InfraConfig, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/infra/config", None, None)
            .await
    }

    /// Update runtime configuration.
    pub async fn update_config(&self, req: InfraConfig) -> Result<InfraConfig, OpenWAError> {
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, "/api/infra/config", None, Some(body))
            .await
    }

    /// Request a server process restart.
    pub async fn restart(&self) -> Result<RestartResponse, OpenWAError> {
        self.transport
            .execute(Method::POST, "/api/infra/restart", None, None)
            .await
    }

    /// Export all database migration tables as JSON payload.
    pub async fn export_data(&self) -> Result<Vec<u8>, OpenWAError> {
        self.transport
            .execute_raw(Method::GET, "/api/infra/export-data", None, None)
            .await
    }

    /// Restore database tables from previously exported data.
    pub async fn import_data(&self, req: ImportDataRequest) -> Result<SuccessResult, OpenWAError> {
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, "/api/infra/import-data", None, Some(body))
            .await
    }

    /// Get count and total bytes of stored media and auth files.
    pub async fn get_storage_file_count(&self) -> Result<StorageCount, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/infra/storage/files/count", None, None)
            .await
    }

    /// Export storage files as a compressed tar archive.
    pub async fn export_storage(&self) -> Result<Vec<u8>, OpenWAError> {
        self.transport
            .execute_raw(Method::GET, "/api/infra/storage/export", None, None)
            .await
    }

    /// Import storage archive from internal file path.
    pub async fn import_storage(
        &self,
        req: ImportStorageRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, "/api/infra/storage/import", None, Some(body))
            .await
    }
}
