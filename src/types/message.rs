use serde::{Deserialize, Serialize};

/// Standard response returned when a message send is accepted by the gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// Unix epoch seconds.
    pub timestamp: i64,
}

/// Request to send a plain text message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTextRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
}

impl SendTextRequest {
    pub fn new(chat_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            text: text.into(),
            mentions: None,
        }
    }

    pub fn with_mentions(mut self, mentions: Vec<String>) -> Self {
        self.mentions = Some(mentions);
        self
    }
}

/// Request to send media (Image, Video, Audio, Document, Sticker).
/// Exactly one of `url` or `base64` must be supplied. `base64` requires `mimetype`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMediaRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
    #[serde(default, rename = "quotedMessageId", skip_serializing_if = "Option::is_none")]
    pub quoted_message_id: Option<String>,
}

/// Request to send an audio message (file or voice note PTT).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendAudioRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
    #[serde(default, rename = "quotedMessageId", skip_serializing_if = "Option::is_none")]
    pub quoted_message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ptt: Option<bool>,
}

impl SendAudioRequest {
    pub fn from_url(chat_id: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            url: Some(url.into()),
            ..Default::default()
        }
    }

    pub fn voice_note(chat_id: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            url: Some(url.into()),
            ptt: Some(true),
            ..Default::default()
        }
    }
}

impl SendMediaRequest {
    pub fn from_url(chat_id: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            url: Some(url.into()),
            ..Default::default()
        }
    }

    pub fn from_base64(
        chat_id: impl Into<String>,
        base64: impl Into<String>,
        mimetype: impl Into<String>,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            base64: Some(base64.into()),
            mimetype: Some(mimetype.into()),
            ..Default::default()
        }
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn with_filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
        self
    }

    pub fn with_mentions(mut self, mentions: Vec<String>) -> Self {
        self.mentions = Some(mentions);
        self
    }
}

/// Request to send a geographic location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendLocationRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to send a contact card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendContactRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "contactName", alias = "name")]
    pub contact_name: String,
    #[serde(rename = "contactNumber", alias = "contactId")]
    pub contact_number: String,
    #[serde(default, rename = "quotedMessageId", skip_serializing_if = "Option::is_none")]
    pub quoted_message_id: Option<String>,
}

impl SendContactRequest {
    pub fn new(
        chat_id: impl Into<String>,
        contact_name: impl Into<String>,
        contact_number: impl Into<String>,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            contact_name: contact_name.into(),
            contact_number: contact_number.into(),
            quoted_message_id: None,
        }
    }
}

/// Request to send a stored message template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTemplateRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(default, rename = "templateId", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(default, rename = "templateName", skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(
        default,
        rename = "vars",
        alias = "variables",
        skip_serializing_if = "Option::is_none"
    )]
    pub vars: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
    #[serde(default, rename = "linkPreview", skip_serializing_if = "Option::is_none")]
    pub link_preview: Option<bool>,
}

/// Request to send a native poll.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPollRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub name: String,
    pub options: Vec<String>,
    #[serde(
        default,
        rename = "allowMultipleAnswers",
        alias = "multipleAnswers",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_multiple_answers: Option<bool>,
    #[serde(default, rename = "quotedMessageId", skip_serializing_if = "Option::is_none")]
    pub quoted_message_id: Option<String>,
}

/// Request to reply to an existing message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyMessageRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub text: String,
    #[serde(rename = "quotedMessageId")]
    pub quoted_message_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
}

/// Request to forward an existing message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardMessageRequest {
    #[serde(rename = "fromChatId")]
    pub from_chat_id: String,
    #[serde(rename = "toChatId")]
    pub to_chat_id: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
}

/// Request to tap a button or list item on a WhatsApp Business prompt (Baileys only).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickButtonRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "buttonId")]
    pub button_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Request to react to a message with an emoji (empty string clears reaction).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactMessageRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "emoji", alias = "reaction")]
    pub emoji: String,
}

/// Request to delete a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(
        default,
        rename = "forEveryone",
        alias = "everyone",
        skip_serializing_if = "Option::is_none"
    )]
    pub for_everyone: Option<bool>,
}

/// Request to edit an already sent text message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMessageRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "body", alias = "text")]
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
}

/// Request to pin a message in a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinMessageRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// Must be 86400 (24h), 604800 (7d), or 2592000 (30d).
    #[serde(rename = "durationSeconds")]
    pub duration_seconds: u32,
}

/// Request to unpin a pinned message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnpinMessageRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
}

/// Request to star or unstar a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMessageRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub star: bool,
}

/// Request to vote on a poll.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotePollRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "pollMessageId", alias = "messageId")]
    pub poll_message_id: String,
    /// Texts of the options selected (not option IDs).
    pub options: Vec<String>,
}

/// Stored message record returned by message listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRecord {
    pub id: String,
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub from: String,
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "type")]
    pub message_type: String,
    /// Unix epoch seconds.
    pub timestamp: i64,
    #[serde(default, rename = "fromMe")]
    pub from_me: bool,
    #[serde(default, rename = "isGroup")]
    pub is_group: bool,
    #[serde(default, rename = "hasMedia")]
    pub has_media: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Stored media item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMedia {
    pub data: String, // base64 string
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// Message history entry read live from WhatsApp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHistoryMessage {
    pub id: String,
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub from: String,
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "type")]
    pub message_type: String,
    pub timestamp: i64,
}

/// Single reaction snapshot on a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionRecord {
    #[serde(rename = "senderId")]
    pub sender_id: String,
    pub reaction: String,
}

/// Media payload inside a bulk message item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkMediaDto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ptt: Option<bool>,
}

/// Content payload inside a bulk message item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkMessageContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<BulkMediaDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<BulkMediaDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<BulkMediaDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<BulkMediaDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
}

/// Single message item in a bulk send batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkMessageItem {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "type")]
    pub message_type: String, // "text" | "image" | "video" | "audio" | "document"
    pub content: BulkMessageContent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, String>>,
}

impl BulkMessageItem {
    pub fn text(chat_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_type: "text".to_string(),
            content: BulkMessageContent {
                text: Some(text.into()),
                ..Default::default()
            },
            variables: None,
        }
    }
}

/// Options controlling bulk batch execution.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkMessageOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay_between_messages: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub randomize_delay: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_on_error: Option<bool>,
}

/// Request to enqueue a bulk send batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBulkRequest {
    #[serde(default, rename = "batchId", skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    pub messages: Vec<BulkMessageItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BulkMessageOptions>,
}

impl SendBulkRequest {
    pub fn new(messages: Vec<BulkMessageItem>) -> Self {
        Self {
            batch_id: None,
            messages,
            options: None,
        }
    }
}

/// Response returned when a bulk batch is enqueued (HTTP 202).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkMessageResponse {
    #[serde(rename = "batchId")]
    pub batch_id: String,
    pub status: String,
    #[serde(rename = "totalMessages")]
    pub total_messages: usize,
}

/// Detailed status of an asynchronous bulk send batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchStatusResponse {
    #[serde(rename = "batchId")]
    pub batch_id: String,
    pub status: String,
    pub total: usize,
    pub sent: usize,
    pub failed: usize,
    #[serde(default, rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(
        default,
        rename = "completedAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_at: Option<String>,
}

/// Query parameters for listing stored messages.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMessagesQuery {
    #[serde(default, rename = "chatId", skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// Query parameters for live chat history.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatHistoryQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
