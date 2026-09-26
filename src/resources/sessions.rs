use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

#[derive(Clone)]
pub struct SessionsResource {
    pub(crate) transport: Arc<Transport>,
}

impl SessionsResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List all sessions visible to the API key.
    pub async fn list(
        &self,
        query: Option<ListSessionsQuery>,
    ) -> Result<Vec<SessionResponse>, OpenWAError> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if let Some(ref q) = query {
            if let Some(limit) = q.limit {
                query_params.push(("limit", limit.to_string()));
            }
            if let Some(offset) = q.offset {
                query_params.push(("offset", offset.to_string()));
            }
            if let Some(ref name) = q.name {
                query_params.push(("name", name.clone()));
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
            .execute(Method::GET, "/api/sessions", q_opt, None)
            .await
    }

    /// Get details of a single session by its UUID.
    pub async fn get(&self, session_id: &str) -> Result<SessionResponse, OpenWAError> {
        let path = format!("/api/sessions/{}", encode_path_segment(session_id));
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Create a new session. (Requires OPERATOR role).
    pub async fn create(&self, req: CreateSessionRequest) -> Result<SessionResponse, OpenWAError> {
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, "/api/sessions", None, Some(body))
            .await
    }

    /// Delete a session. (Requires OPERATOR role).
    pub async fn delete(&self, session_id: &str) -> Result<(), OpenWAError> {
        let path = format!("/api/sessions/{}", encode_path_segment(session_id));
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }

    /// Start a session and connect to WhatsApp. (Requires OPERATOR role).
    pub async fn start(&self, session_id: &str) -> Result<SessionResponse, OpenWAError> {
        let path = format!("/api/sessions/{}/start", encode_path_segment(session_id));
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Stop a session gracefully. (Requires OPERATOR role).
    pub async fn stop(&self, session_id: &str) -> Result<SessionResponse, OpenWAError> {
        let path = format!("/api/sessions/{}/stop", encode_path_segment(session_id));
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Unlink WhatsApp device and stop session. (Requires OPERATOR role).
    pub async fn logout(&self, session_id: &str) -> Result<SessionResponse, OpenWAError> {
        let path = format!("/api/sessions/{}/logout", encode_path_segment(session_id));
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Force-terminate a stuck session (SIGKILL + teardown). (Requires OPERATOR role).
    pub async fn force_kill(&self, session_id: &str) -> Result<SessionResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/force-kill",
            encode_path_segment(session_id)
        );
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Get the current pairing QR code as PNG data URL. (Requires OPERATOR role).
    pub async fn get_qr_code(&self, session_id: &str) -> Result<QrCodeResponse, OpenWAError> {
        let path = format!("/api/sessions/{}/qr", encode_path_segment(session_id));
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Request an 8-character pairing code for phone login. (Requires OPERATOR role).
    pub async fn request_pairing_code(
        &self,
        session_id: &str,
        req: PairingCodeRequest,
    ) -> Result<PairingCodeResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/pairing-code",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Read a session's engine configuration.
    pub async fn get_config(&self, session_id: &str) -> Result<SessionConfig, OpenWAError> {
        let path = format!("/api/sessions/{}/config", encode_path_segment(session_id));
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Update runtime engine configuration. (Requires OPERATOR role).
    pub async fn update_config(
        &self,
        session_id: &str,
        req: UpdateSessionConfigRequest,
    ) -> Result<SessionConfig, OpenWAError> {
        let path = format!("/api/sessions/{}/config", encode_path_segment(session_id));
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PATCH, &path, None, Some(body))
            .await
    }

    /// Read masked proxy configuration for a session.
    pub async fn get_proxy(&self, session_id: &str) -> Result<SessionProxy, OpenWAError> {
        let path = format!("/api/sessions/{}/proxy", encode_path_segment(session_id));
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Update session proxy settings. (Requires OPERATOR role).
    pub async fn update_proxy(
        &self,
        session_id: &str,
        req: UpdateSessionProxyRequest,
    ) -> Result<SessionProxy, OpenWAError> {
        let path = format!("/api/sessions/{}/proxy", encode_path_segment(session_id));
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PATCH, &path, None, Some(body))
            .await
    }

    /// Aggregate session statistics overview across visible sessions.
    pub async fn stats(&self) -> Result<SessionStatsOverview, OpenWAError> {
        self.transport
            .execute(Method::GET, "/api/sessions/stats/overview", None, None)
            .await
    }

    /// Set account's online presence (appear online or offline). (Requires OPERATOR role).
    pub async fn set_online_presence(
        &self,
        session_id: &str,
        req: SetOnlinePresenceRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!("/api/sessions/{}/presence", encode_path_segment(session_id));
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
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

    /// List active chats for a session.
    pub async fn list_chats(
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

    /// Mark chat as read. (Requires OPERATOR role).
    pub async fn mark_chat_read(
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
    pub async fn mark_chat_unread(
        &self,
        session_id: &str,
        req: MarkChatRequest,
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
    pub async fn archive_chat(
        &self,
        session_id: &str,
        req: MarkChatRequest,
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

    /// Mute or unmute a chat. (Requires OPERATOR role).
    pub async fn mute_chat(
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

    /// Pin or unpin a chat. (Requires OPERATOR role).
    pub async fn pin_chat(
        &self,
        session_id: &str,
        req: MarkChatRequest,
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

    /// Delete a chat. (Requires OPERATOR role).
    pub async fn delete_chat(
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
    pub async fn send_chat_state(
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

    /// Clear all messages in a chat. (Requires OPERATOR role).
    pub async fn clear_chat_messages(
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

    /// List groups the session belongs to.
    pub async fn list_groups(
        &self,
        session_id: &str,
        query: Option<ListContactsQuery>,
    ) -> Result<Vec<GroupSummary>, OpenWAError> {
        let path = format!("/api/sessions/{}/groups", encode_path_segment(session_id));
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
}
