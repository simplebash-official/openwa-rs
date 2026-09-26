use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Method;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::time::Duration;

use crate::error::OpenWAError;
use crate::resources::*;
use crate::retry::RetryPolicy;
use crate::transport::Transport;

#[cfg(feature = "events")]
use crate::events::EventStream;

/// The primary entry point for communicating with an OpenWA WhatsApp API Gateway instance.
#[derive(Clone)]
pub struct OpenWAClient {
    pub(crate) transport: Arc<Transport>,
}

impl OpenWAClient {
    /// Create a new `OpenWAClient` with default configuration and timeout (30 seconds).
    pub fn new(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Result<Self, OpenWAError> {
        Self::builder().base_url(base_url).api_key(api_key).build()
    }

    /// Create a new builder to configure the `OpenWAClient`.
    pub fn builder() -> OpenWAClientBuilder {
        OpenWAClientBuilder::default()
    }

    // ==========================================
    // Standard User / Operator Resources
    // ==========================================

    /// WhatsApp session lifecycle and credentials management.
    pub fn sessions(&self) -> SessionsResource {
        SessionsResource::new(self.transport.clone())
    }

    /// Send, receive, react to, and manipulate WhatsApp messages.
    pub fn messages(&self) -> MessagesResource {
        MessagesResource::new(self.transport.clone())
    }

    /// Contact information, presence checks, and profile photos.
    pub fn contacts(&self) -> ContactsResource {
        ContactsResource::new(self.transport.clone())
    }

    /// WhatsApp group chats, administration, settings, and invites.
    pub fn groups(&self) -> GroupsResource {
        GroupsResource::new(self.transport.clone())
    }

    /// Chat summaries, mute/unmute, presence state, and read receipts.
    pub fn chats(&self) -> ChatsResource {
        ChatsResource::new(self.transport.clone())
    }

    /// Webhook subscriptions, secrets, ping tests, and failure outbox.
    pub fn webhooks(&self) -> WebhooksResource {
        WebhooksResource::new(self.transport.clone())
    }

    /// WhatsApp Business labels and chat associations.
    pub fn labels(&self) -> LabelsResource {
        LabelsResource::new(self.transport.clone())
    }

    /// WhatsApp Newsletters and broadcast channels.
    pub fn channels(&self) -> ChannelsResource {
        ChannelsResource::new(self.transport.clone())
    }

    /// WhatsApp Business product catalog and product messages.
    pub fn catalog(&self) -> CatalogResource {
        CatalogResource::new(self.transport.clone())
    }

    /// Contact stories and WhatsApp status broadcasts.
    pub fn status(&self) -> StatusResource {
        StatusResource::new(self.transport.clone())
    }

    /// Global indexed message search.
    pub fn search(&self) -> SearchResource {
        SearchResource::new(self.transport.clone())
    }

    /// HSM and quick-reply templates.
    pub fn templates(&self) -> TemplatesResource {
        TemplatesResource::new(self.transport.clone())
    }

    /// Profile display name, about/status bio, and avatar.
    pub fn profile(&self) -> ProfileResource {
        ProfileResource::new(self.transport.clone())
    }

    /// Voice and video call links and call handling.
    pub fn calls(&self) -> CallsResource {
        CallsResource::new(self.transport.clone())
    }

    /// FFmpeg media conversion utilities (voice notes, MP4 videos).
    pub fn media(&self) -> MediaResource {
        MediaResource::new(self.transport.clone())
    }

    /// Gateway system health, liveness, and readiness probes.
    pub fn health(&self) -> HealthResource {
        HealthResource::new(self.transport.clone())
    }

    // ==========================================
    // Administrative & Infrastructure Namespace
    // ==========================================

    /// Administrative resources (API keys, stats, plugins, integrations, infra, settings, audit).
    pub fn admin(&self) -> AdminResource {
        AdminResource::new(self.transport.clone())
    }

    /// Shortcut for API key management and validation.
    pub fn auth(&self) -> AuthKeysResource {
        AuthKeysResource::new(self.transport.clone())
    }

    // ==========================================
    // Real-Time Events (WebSocket)
    // ==========================================

    /// Connect to the real-time Socket.IO WebSocket `/events` stream.
    #[cfg(feature = "events")]
    pub async fn events(&self) -> Result<EventStream, OpenWAError> {
        EventStream::connect(&self.transport.base_url, &self.transport.api_key).await
    }

    // ==========================================
    // Raw Request Escape Hatch
    // ==========================================

    /// Execute a custom or unmapped request against the OpenWA Gateway with automatic auth and retries.
    pub async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<serde_json::Value>,
    ) -> Result<T, OpenWAError> {
        self.transport.execute(method, path, query, body).await
    }

    /// Execute a raw request returning the raw response bytes.
    pub async fn request_raw(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<serde_json::Value>,
    ) -> Result<Vec<u8>, OpenWAError> {
        self.transport.execute_raw(method, path, query, body).await
    }
}

/// Builder for configuring and constructing an `OpenWAClient`.
#[derive(Debug, Clone)]
pub struct OpenWAClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    timeout: Duration,
    retry_policy: Option<RetryPolicy>,
    default_headers: HeaderMap,
}

impl Default for OpenWAClientBuilder {
    fn default() -> Self {
        Self {
            base_url: None,
            api_key: None,
            timeout: Duration::from_secs(30),
            retry_policy: Some(RetryPolicy::default()),
            default_headers: HeaderMap::new(),
        }
    }
}

impl OpenWAClientBuilder {
    /// Create a new client builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL of the OpenWA Gateway (e.g. `http://localhost:3000`).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the API key used for authenticating requests (`X-API-Key`).
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set HTTP request timeout. Defaults to 30 seconds.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Configure custom retry policy, or `None` to disable retries.
    pub fn retry_policy(mut self, policy: Option<RetryPolicy>) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Add a default HTTP header to send with all requests.
    pub fn default_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.default_headers.insert(name, value);
        self
    }

    /// Build the configured `OpenWAClient`.
    pub fn build(self) -> Result<OpenWAClient, OpenWAError> {
        let base_url = self
            .base_url
            .ok_or_else(|| OpenWAError::Config("Base URL must be specified".to_string()))?;
        let api_key = self
            .api_key
            .ok_or_else(|| OpenWAError::Config("API key must be specified".to_string()))?;

        let transport = Transport::new(
            base_url,
            api_key,
            self.timeout,
            self.default_headers,
            self.retry_policy,
        )?;

        Ok(OpenWAClient {
            transport: Arc::new(transport),
        })
    }
}
