use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

#[derive(Clone)]
pub struct MessagesResource {
    pub(crate) transport: Arc<Transport>,
}

impl MessagesResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List stored messages.
    pub async fn list(
        &self,
        session_id: &str,
        query: Option<ListMessagesQuery>,
    ) -> Result<Vec<MessageRecord>, OpenWAError> {
        let path = format!("/api/sessions/{}/messages", encode_path_segment(session_id));
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if let Some(ref q) = query {
            if let Some(ref cid) = q.chat_id {
                query_params.push(("chatId", cid.clone()));
            }
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

    /// Send a text message (text max 4096 chars). (Requires OPERATOR role).
    pub async fn send_text(
        &self,
        session_id: &str,
        req: SendTextRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/send-text",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Send an image (url or base64). (Requires OPERATOR role).
    pub async fn send_image(
        &self,
        session_id: &str,
        req: SendMediaRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/send-image",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Send a video (url or base64). (Requires OPERATOR role).
    pub async fn send_video(
        &self,
        session_id: &str,
        req: SendMediaRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/send-video",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Send an audio file or voice note (url or base64). (Requires OPERATOR role).
    pub async fn send_audio(
        &self,
        session_id: &str,
        req: SendAudioRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/send-audio",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Send a document (url or base64). (Requires OPERATOR role).
    pub async fn send_document(
        &self,
        session_id: &str,
        req: SendMediaRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/send-document",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Send a sticker (url or base64). (Requires OPERATOR role).
    pub async fn send_sticker(
        &self,
        session_id: &str,
        req: SendMediaRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/send-sticker",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Send a location. (Requires OPERATOR role).
    pub async fn send_location(
        &self,
        session_id: &str,
        req: SendLocationRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/send-location",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Send a contact card. (Requires OPERATOR role).
    pub async fn send_contact(
        &self,
        session_id: &str,
        req: SendContactRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/send-contact",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Send a template message. (Requires OPERATOR role).
    pub async fn send_template(
        &self,
        session_id: &str,
        req: SendTemplateRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/send-template",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Send a native poll. (Requires OPERATOR role).
    pub async fn send_poll(
        &self,
        session_id: &str,
        req: SendPollRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/send-poll",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Reply to a specific message. (Requires OPERATOR role).
    pub async fn reply(
        &self,
        session_id: &str,
        req: ReplyMessageRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/reply",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Forward a message to another chat. (Requires OPERATOR role).
    pub async fn forward(
        &self,
        session_id: &str,
        req: ForwardMessageRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/forward",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Tap a button or list row on a WhatsApp Business prompt (Baileys only). (Requires OPERATOR role).
    pub async fn click_button(
        &self,
        session_id: &str,
        req: ClickButtonRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/click-button",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// React to a message with an emoji. (Requires OPERATOR role).
    pub async fn react(
        &self,
        session_id: &str,
        req: ReactMessageRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/react",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Delete a message. (Requires OPERATOR role).
    pub async fn delete(
        &self,
        session_id: &str,
        req: DeleteMessageRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/delete",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Edit the text of a sent message. (Requires OPERATOR role).
    pub async fn edit(
        &self,
        session_id: &str,
        req: EditMessageRequest,
    ) -> Result<MessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/edit",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Pin a message in a chat. (Requires OPERATOR role).
    pub async fn pin(
        &self,
        session_id: &str,
        req: PinMessageRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/pin",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Unpin a message in a chat. (Requires OPERATOR role).
    pub async fn unpin(
        &self,
        session_id: &str,
        req: UnpinMessageRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/unpin",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Star or unstar a message. (Requires OPERATOR role).
    pub async fn star(
        &self,
        session_id: &str,
        req: StarMessageRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/star",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Cast a vote on a poll. (Requires OPERATOR role).
    pub async fn vote_poll(
        &self,
        session_id: &str,
        req: VotePollRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/vote-poll",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Fetch message history for a chat directly from WhatsApp.
    pub async fn history(
        &self,
        session_id: &str,
        chat_id: &str,
        query: Option<ChatHistoryQuery>,
    ) -> Result<Vec<ChatHistoryMessage>, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/{}/history",
            encode_path_segment(session_id),
            encode_path_segment(chat_id)
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

    /// List reactions for a specific message.
    pub async fn reactions(
        &self,
        session_id: &str,
        chat_id: &str,
        message_id: &str,
    ) -> Result<Vec<ReactionRecord>, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/{}/{}/reactions",
            encode_path_segment(session_id),
            encode_path_segment(chat_id),
            encode_path_segment(message_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Fetch stored media bytes for a message. Returns raw bytes.
    pub async fn media_bytes(
        &self,
        session_id: &str,
        chat_id: &str,
        message_id: &str,
    ) -> Result<Vec<u8>, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/{}/{}/media",
            encode_path_segment(session_id),
            encode_path_segment(chat_id),
            encode_path_segment(message_id)
        );
        self.transport
            .execute_raw(Method::GET, &path, None, None)
            .await
    }

    /// Enqueue an asynchronous bulk message send batch. (Requires OPERATOR role).
    pub async fn send_bulk(
        &self,
        session_id: &str,
        req: SendBulkRequest,
    ) -> Result<BulkMessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/send-bulk",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Poll the status and progress of a bulk send batch.
    pub async fn batch_status(
        &self,
        session_id: &str,
        batch_id: &str,
    ) -> Result<BatchStatusResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/batch/{}",
            encode_path_segment(session_id),
            encode_path_segment(batch_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Cancel a running bulk send batch. (Requires OPERATOR role).
    pub async fn cancel_batch(
        &self,
        session_id: &str,
        batch_id: &str,
    ) -> Result<BatchStatusResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/batch/{}/cancel",
            encode_path_segment(session_id),
            encode_path_segment(batch_id)
        );
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }
}
