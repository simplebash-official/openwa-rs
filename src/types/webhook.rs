use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Webhook configuration record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookRecord {
    pub id: String,
    pub url: String,
    pub events: Vec<String>,
    #[serde(default)]
    pub active: bool,
    #[serde(default, rename = "retryCount")]
    pub retry_count: u32,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Request to create a new webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub events: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(default = "default_true")]
    pub active: bool,
    #[serde(
        default,
        rename = "retryCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_count: Option<u32>,
    #[serde(
        default,
        rename = "customHeaders",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_headers: Option<HashMap<String, String>>,
}

fn default_true() -> bool {
    true
}

/// Request to update an existing webhook.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWebhookRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(
        default,
        rename = "retryCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_count: Option<u32>,
    #[serde(
        default,
        rename = "customHeaders",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_headers: Option<HashMap<String, String>>,
}

/// Response returned from testing a webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookTestResponse {
    pub success: bool,
    #[serde(
        default,
        rename = "statusCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_code: Option<u16>,
    #[serde(default, rename = "latencyMs", skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Delivery failure diagnostic record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDeliveryFailure {
    pub id: String,
    #[serde(rename = "webhookId")]
    pub webhook_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub event: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub attempts: u32,
    pub timestamp: String,
}

/// Query parameters for listing delivery failures.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeliveryFailureQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(default, rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, rename = "webhookId", skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
}
