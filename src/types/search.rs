use serde::{Deserialize, Serialize};

/// Parameters for global message search.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesParams {
    pub q: String,
    #[serde(default, rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, rename = "chatId", skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(default, rename = "dateFrom", skip_serializing_if = "Option::is_none")]
    pub date_from: Option<i64>,
    #[serde(default, rename = "dateTo", skip_serializing_if = "Option::is_none")]
    pub date_to: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// A matched message search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItem {
    pub id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub from: String,
    pub to: String,
    pub body: String,
    pub timestamp: i64,
}

/// Response returned from message search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesResponse {
    pub results: Vec<SearchResultItem>,
    pub total: usize,
}
