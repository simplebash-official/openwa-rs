use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

#[derive(Clone)]
pub struct ChatsResource {
    pub(crate) transport: Arc<Transport>,
}

impl ChatsResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List active chats for a session.
    pub async fn list(
        &self,
        session_id: &str,
        query: Option<ListChatsQuery>,
    ) -> Result<Vec<ChatSummary>, OpenWAError> {
        let path = format!("/api/sessions/{}/chats", encode_path_segment(session_id));
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if let Some(ref q) = query {
            if let Some(limit) = q.limit {
                query_params.push(("limit", limit.to_string()));
            }
            if let Some(offset) = q.offset {
                query_params.push(("offset", offset.to_string()));
            }
        }
        let q_refs: Vec<(&str, &str)> =
            query_params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let q_opt = if q_refs.is_empty() {
            None
        } else {
            Some(q_refs.as_slice())
        };

        self.transport
            .execute(Method::GET, &path, q_opt, None)
            .await
    }

    /// Subscribe to a chat's presence updates. (Requires OPERATOR role).
    pub async fn subscribe_presence(
        &self,
        session_id: &str,
        req: SubscribePresenceRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/presence/subscribe",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Get last known presence for a chat.
    pub async fn get_presence(
        &self,
        session_id: &str,
        chat_id: &str,
    ) -> Result<Option<ChatPresence>, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/presence/{}",
            encode_path_segment(session_id),
            encode_path_segment(chat_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Mark chat as read. (Requires OPERATOR role).
    pub async fn mark_read(
        &self,
        session_id: &str,
        req: MarkChatReadRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/chats/read",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Mark chat as unread. (Requires OPERATOR role).
    pub async fn mark_unread(
        &self,
        session_id: &str,
        req: MarkChatUnreadRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/chats/unread",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Archive or unarchive a chat. (Requires OPERATOR role).
    pub async fn archive(
        &self,
        session_id: &str,
        req: ArchiveChatRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/chats/archive",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Pin or unpin a chat. (Requires OPERATOR role).
    pub async fn pin(
        &self,
        session_id: &str,
        req: PinChatRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/chats/pin",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Mute or unmute a chat. (Requires OPERATOR role).
    pub async fn mute(
        &self,
        session_id: &str,
        req: MuteChatRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/chats/mute",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Delete all messages in a chat. (Requires OPERATOR role).
    pub async fn clear_messages(
        &self,
        session_id: &str,
        chat_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/chats/{}/messages",
            encode_path_segment(session_id),
            encode_path_segment(chat_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }

    /// Delete a chat. (Requires OPERATOR role).
    pub async fn delete(
        &self,
        session_id: &str,
        req: MarkChatRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/chats/delete",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Send typing or recording presence state. (Requires OPERATOR role).
    pub async fn send_state(
        &self,
        session_id: &str,
        req: SendChatStateRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/chats/typing",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }
}
