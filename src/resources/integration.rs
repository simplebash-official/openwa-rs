use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

/// Third-party integrations and webhook ingress routing (Requires ADMIN role).
#[derive(Clone)]
pub struct IntegrationResource {
    pub(crate) transport: Arc<Transport>,
}

impl IntegrationResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List all instances for a specific integration plugin.
    pub async fn list(&self, plugin_id: &str) -> Result<Vec<IntegrationInstance>, OpenWAError> {
        let path = format!(
            "/api/integration/plugins/{}/instances",
            encode_path_segment(plugin_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Get a single integration instance.
    pub async fn get(
        &self,
        plugin_id: &str,
        instance_id: &str,
    ) -> Result<IntegrationInstance, OpenWAError> {
        let path = format!(
            "/api/integration/plugins/{}/instances/{}",
            encode_path_segment(plugin_id),
            encode_path_segment(instance_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Create a new instance of an integration plugin.
    pub async fn create(
        &self,
        plugin_id: &str,
        req: CreateIntegrationInstanceRequest,
    ) -> Result<IntegrationInstance, OpenWAError> {
        let path = format!(
            "/api/integration/plugins/{}/instances",
            encode_path_segment(plugin_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Update an existing integration instance.
    pub async fn update(
        &self,
        plugin_id: &str,
        instance_id: &str,
        req: UpdateIntegrationInstanceRequest,
    ) -> Result<IntegrationInstance, OpenWAError> {
        let path = format!(
            "/api/integration/plugins/{}/instances/{}",
            encode_path_segment(plugin_id),
            encode_path_segment(instance_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PATCH, &path, None, Some(body))
            .await
    }

    /// Delete an integration instance.
    pub async fn delete(
        &self,
        plugin_id: &str,
        instance_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/integration/plugins/{}/instances/{}",
            encode_path_segment(plugin_id),
            encode_path_segment(instance_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }

    /// Regenerate ingress authentication secret for an instance.
    pub async fn regenerate_secret(
        &self,
        plugin_id: &str,
        instance_id: &str,
    ) -> Result<RegenerateSecretResponse, OpenWAError> {
        let path = format!(
            "/api/integration/plugins/{}/instances/{}/regenerate-secret",
            encode_path_segment(plugin_id),
            encode_path_segment(instance_id)
        );
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Redrive a failed integration instance event.
    pub async fn redrive(
        &self,
        plugin_id: &str,
        instance_id: &str,
    ) -> Result<RedriveResponse, OpenWAError> {
        let path = format!(
            "/api/integration/instances/{}/{}/redrive",
            encode_path_segment(plugin_id),
            encode_path_segment(instance_id)
        );
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Send an ingress request (GET) to an integration instance.
    pub async fn ingress_get(
        &self,
        plugin_id: &str,
        instance_id: &str,
        subpath: &str,
    ) -> Result<serde_json::Value, OpenWAError> {
        let path = format!(
            "/api/ingress/{}/{}/{}",
            encode_path_segment(plugin_id),
            encode_path_segment(instance_id),
            subpath.trim_start_matches('/')
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Send an ingress request (POST) to an integration instance.
    pub async fn ingress_post(
        &self,
        plugin_id: &str,
        instance_id: &str,
        subpath: &str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, OpenWAError> {
        let path = format!(
            "/api/ingress/{}/{}/{}",
            encode_path_segment(plugin_id),
            encode_path_segment(instance_id),
            subpath.trim_start_matches('/')
        );
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Send an ingress request (PUT) to an integration instance.
    pub async fn ingress_put(
        &self,
        plugin_id: &str,
        instance_id: &str,
        subpath: &str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, OpenWAError> {
        let path = format!(
            "/api/ingress/{}/{}/{}",
            encode_path_segment(plugin_id),
            encode_path_segment(instance_id),
            subpath.trim_start_matches('/')
        );
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Send an ingress request (DELETE) to an integration instance.
    pub async fn ingress_delete(
        &self,
        plugin_id: &str,
        instance_id: &str,
        subpath: &str,
    ) -> Result<serde_json::Value, OpenWAError> {
        let path = format!(
            "/api/ingress/{}/{}/{}",
            encode_path_segment(plugin_id),
            encode_path_segment(instance_id),
            subpath.trim_start_matches('/')
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }

    /// Send an ingress request (PATCH) to an integration instance.
    pub async fn ingress_patch(
        &self,
        plugin_id: &str,
        instance_id: &str,
        subpath: &str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, OpenWAError> {
        let path = format!(
            "/api/ingress/{}/{}/{}",
            encode_path_segment(plugin_id),
            encode_path_segment(instance_id),
            subpath.trim_start_matches('/')
        );
        self.transport
            .execute(Method::PATCH, &path, None, Some(body))
            .await
    }
}
