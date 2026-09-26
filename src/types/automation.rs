use serde::{Deserialize, Serialize};

/// Automation autoreply rule record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub name: String,
    pub pattern: String,
    #[serde(rename = "matchType")]
    pub match_type: String, // "exact", "contains", "regex"
    pub reply: String,
    #[serde(default)]
    pub active: bool,
}

/// Request to create an automation rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAutomationRuleRequest {
    pub name: String,
    pub pattern: String,
    #[serde(rename = "matchType")]
    pub match_type: String,
    pub reply: String,
    #[serde(default = "default_true")]
    pub active: bool,
}

fn default_true() -> bool {
    true
}

/// Request to update an automation rule.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAutomationRuleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(default, rename = "matchType", skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}
