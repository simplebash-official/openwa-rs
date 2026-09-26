use serde::{Deserialize, Serialize};

/// Automation autoreply rule record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub name: String,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<serde_json::Value>,
    #[serde(rename = "replyText")]
    pub reply_text: String,
    #[serde(rename = "cooldownSeconds")]
    pub cooldown_seconds: u32,
    #[serde(default, rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Request to create an automation rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAutomationRuleRequest {
    pub name: String,
    #[serde(rename = "replyText")]
    pub reply_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<serde_json::Value>,
    #[serde(
        default,
        rename = "cooldownSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub cooldown_seconds: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl CreateAutomationRuleRequest {
    pub fn new(name: impl Into<String>, reply_text: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            reply_text: reply_text.into(),
            conditions: None,
            cooldown_seconds: None,
            enabled: Some(true),
        }
    }
}

/// Request to update an automation rule.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAutomationRuleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, rename = "replyText", skip_serializing_if = "Option::is_none")]
    pub reply_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<serde_json::Value>,
    #[serde(
        default,
        rename = "cooldownSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub cooldown_seconds: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
