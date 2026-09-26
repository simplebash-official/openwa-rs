use crate::error::OpenWAError;
use crate::transport::Transport;
use crate::types::{SearchMessagesParams, SearchMessagesResponse};
use reqwest::Method;
use std::sync::Arc;

/// Global message search resource.
#[derive(Clone)]
pub struct SearchResource {
    pub(crate) transport: Arc<Transport>,
}

impl SearchResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// Search indexed messages across sessions and chats.
    pub async fn search(
        &self,
        params: SearchMessagesParams,
    ) -> Result<SearchMessagesResponse, OpenWAError> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        query_params.push(("q", params.q));
        if let Some(session_id) = params.session_id {
            query_params.push(("sessionId", session_id));
        }
        if let Some(chat_id) = params.chat_id {
            query_params.push(("chatId", chat_id));
        }
        if let Some(from) = params.from {
            query_params.push(("from", from));
        }
        if let Some(direction) = params.direction {
            query_params.push(("direction", direction));
        }
        if let Some(message_type) = params.message_type {
            query_params.push(("type", message_type));
        }
        if let Some(date_from) = params.date_from {
            query_params.push(("dateFrom", date_from.to_string()));
        }
        if let Some(date_to) = params.date_to {
            query_params.push(("dateTo", date_to.to_string()));
        }
        if let Some(limit) = params.limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(offset) = params.offset {
            query_params.push(("offset", offset.to_string()));
        }

        let q_refs: Vec<(&str, &str)> =
            query_params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.transport
            .execute(Method::GET, "/api/search", Some(q_refs.as_slice()), None)
            .await
    }
}
