use std::sync::Arc;

use crate::resources::audit::AuditResource;
use crate::resources::auth_keys::AuthKeysResource;
use crate::resources::automation::AutomationResource;
use crate::resources::infra::InfraResource;
use crate::resources::integration::IntegrationResource;
use crate::resources::metrics::MetricsResource;
use crate::resources::plugins::PluginsResource;
use crate::resources::settings::SettingsResource;
use crate::resources::stats::StatsResource;
use crate::transport::Transport;

/// Namespace grouping administrative and infrastructure resources.
#[derive(Clone)]
pub struct AdminResource {
    pub(crate) transport: Arc<Transport>,
}

impl AdminResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// API keys management and validation.
    pub fn auth_keys(&self) -> AuthKeysResource {
        AuthKeysResource::new(self.transport.clone())
    }

    /// Automation rules for auto-replies.
    pub fn automation(&self) -> AutomationResource {
        AutomationResource::new(self.transport.clone())
    }

    /// Core infrastructure, update checks, engine controls, and storage.
    pub fn infra(&self) -> InfraResource {
        InfraResource::new(self.transport.clone())
    }

    /// Plugin manager and marketplace catalog.
    pub fn plugins(&self) -> PluginsResource {
        PluginsResource::new(self.transport.clone())
    }

    /// Third-party integrations and ingress event delivery.
    pub fn integration(&self) -> IntegrationResource {
        IntegrationResource::new(self.transport.clone())
    }

    /// System usage and message statistics.
    pub fn stats(&self) -> StatsResource {
        StatsResource::new(self.transport.clone())
    }

    /// Global server configuration and settings.
    pub fn settings(&self) -> SettingsResource {
        SettingsResource::new(self.transport.clone())
    }

    /// Server audit logs.
    pub fn audit(&self) -> AuditResource {
        AuditResource::new(self.transport.clone())
    }

    /// Prometheus scraping endpoint.
    pub fn metrics(&self) -> MetricsResource {
        MetricsResource::new(self.transport.clone())
    }
}
