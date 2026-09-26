use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

/// Plugin management and configuration resource (Requires ADMIN role).
#[derive(Clone)]
pub struct PluginsResource {
    pub(crate) transport: Arc<Transport>,
}

impl PluginsResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List all installed plugins.
    pub async fn list(&self) -> Result<Vec<PluginRecord>, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/plugins", None, None)
            .await
    }

    /// Install a plugin from the registry/npm.
    pub async fn install(&self, req: InstallPluginRequest) -> Result<PluginRecord, OpenWAError> {
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, "/api/plugins/install", None, Some(body))
            .await
    }

    /// Install a plugin from a direct URL or tarball.
    pub async fn install_from_url(
        &self,
        req: InstallPluginUrlRequest,
    ) -> Result<PluginRecord, OpenWAError> {
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, "/api/plugins/install-url", None, Some(body))
            .await
    }

    /// Browse plugin catalog registry.
    pub async fn catalog(&self) -> Result<Vec<PluginCatalogItem>, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/plugins/catalog", None, None)
            .await
    }

    /// Get details of a single plugin.
    pub async fn get(&self, id: &str) -> Result<PluginRecord, OpenWAError> {
        let path = format!("/api/plugins/{}", encode_path_segment(id));
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Uninstall a plugin.
    pub async fn uninstall(&self, id: &str) -> Result<SuccessResult, OpenWAError> {
        let path = format!("/api/plugins/{}", encode_path_segment(id));
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }

    /// Enable a plugin.
    pub async fn enable(&self, id: &str) -> Result<PluginRecord, OpenWAError> {
        let path = format!("/api/plugins/{}/enable", encode_path_segment(id));
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Disable a plugin.
    pub async fn disable(&self, id: &str) -> Result<PluginRecord, OpenWAError> {
        let path = format!("/api/plugins/{}/disable", encode_path_segment(id));
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Update global configuration for a plugin.
    pub async fn update_config(
        &self,
        id: &str,
        config: serde_json::Value,
    ) -> Result<PluginRecord, OpenWAError> {
        let path = format!("/api/plugins/{}/config", encode_path_segment(id));
        self.transport
            .execute(Method::PUT, &path, None, Some(config))
            .await
    }

    /// Retrieve JSON schema / UI schema for configuring this plugin.
    pub async fn get_config_ui(&self, id: &str) -> Result<PluginConfigUi, OpenWAError> {
        let path = format!("/api/plugins/{}/config-ui", encode_path_segment(id));
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Update session-specific configuration for a plugin.
    pub async fn update_session_config(
        &self,
        id: &str,
        session_id: &str,
        config: serde_json::Value,
    ) -> Result<PluginRecord, OpenWAError> {
        let path = format!(
            "/api/plugins/{}/config/{}",
            encode_path_segment(id),
            encode_path_segment(session_id)
        );
        self.transport
            .execute(Method::PUT, &path, None, Some(config))
            .await
    }

    /// Update which sessions this plugin is assigned to.
    pub async fn update_sessions(
        &self,
        id: &str,
        req: UpdatePluginSessionsRequest,
    ) -> Result<PluginRecord, OpenWAError> {
        let path = format!("/api/plugins/{}/sessions", encode_path_segment(id));
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Update a plugin to its latest version.
    pub async fn update(&self, id: &str) -> Result<PluginRecord, OpenWAError> {
        let path = format!("/api/plugins/{}/update", encode_path_segment(id));
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Run a diagnostic health check on a plugin.
    pub async fn check_health(&self, id: &str) -> Result<PluginHealthResponse, OpenWAError> {
        let path = format!("/api/plugins/{}/health", encode_path_segment(id));
        self.transport.execute(Method::GET, &path, None, None).await
    }
}
