use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

/// WhatsApp Newsletters / Channels resource.
#[derive(Clone)]
pub struct ChannelsResource {
    pub(crate) transport: Arc<Transport>,
}

impl ChannelsResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List all subscribed channels.
    pub async fn list(&self, session_id: &str) -> Result<Vec<ChannelRecord>, OpenWAError> {
        let path = format!("/api/sessions/{}/channels", encode_path_segment(session_id));
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Create a new channel.
    pub async fn create(
        &self,
        session_id: &str,
        req: CreateChannelRequest,
    ) -> Result<ChannelRecord, OpenWAError> {
        let path = format!("/api/sessions/{}/channels", encode_path_segment(session_id));
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Get channel information by channel ID.
    pub async fn get(
        &self,
        session_id: &str,
        channel_id: &str,
    ) -> Result<ChannelRecord, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/channels/{}",
            encode_path_segment(session_id),
            encode_path_segment(channel_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Unsubscribe from a channel.
    pub async fn unsubscribe(
        &self,
        session_id: &str,
        channel_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/channels/{}",
            encode_path_segment(session_id),
            encode_path_segment(channel_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }

    /// Get messages from a channel.
    pub async fn get_messages(
        &self,
        session_id: &str,
        channel_id: &str,
        query: Option<ListChannelMessagesQuery>,
    ) -> Result<Vec<ChannelMessage>, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/channels/{}/messages",
            encode_path_segment(session_id),
            encode_path_segment(channel_id)
        );
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if let Some(ref q) = query {
            if let Some(limit) = q.limit {
                query_params.push(("limit", limit.to_string()));
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

    /// Delete / destroy a channel (if owner).
    pub async fn delete_channel(
        &self,
        session_id: &str,
        channel_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/channels/{}/delete",
            encode_path_segment(session_id),
            encode_path_segment(channel_id)
        );
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Mute or unmute notifications for a channel.
    pub async fn mute(
        &self,
        session_id: &str,
        channel_id: &str,
        req: MuteChannelRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/channels/{}/mute",
            encode_path_segment(session_id),
            encode_path_segment(channel_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Demote an admin in a channel.
    pub async fn demote_admin(
        &self,
        session_id: &str,
        channel_id: &str,
        req: DemoteChannelAdminRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/channels/{}/admins/demote",
            encode_path_segment(session_id),
            encode_path_segment(channel_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Transfer ownership of a channel.
    pub async fn transfer_ownership(
        &self,
        session_id: &str,
        channel_id: &str,
        req: TransferChannelOwnershipRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/channels/{}/owner/transfer",
            encode_path_segment(session_id),
            encode_path_segment(channel_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Subscribe to a channel via invite code.
    pub async fn subscribe(
        &self,
        session_id: &str,
        req: SubscribeChannelRequest,
    ) -> Result<ChannelRecord, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/channels/subscribe",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }
}
