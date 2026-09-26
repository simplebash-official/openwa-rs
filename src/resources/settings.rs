use crate::error::OpenWAError;
use crate::transport::Transport;
use crate::types::SystemSettings;
use reqwest::Method;
use std::sync::Arc;

/// System settings resource (Requires ADMIN role).
#[derive(Clone)]
pub struct SettingsResource {
    pub(crate) transport: Arc<Transport>,
}

impl SettingsResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// Retrieve system runtime settings.
    pub async fn get(&self) -> Result<SystemSettings, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/settings", None, None)
            .await
    }
}
