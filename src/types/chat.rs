use serde::{Deserialize, Serialize};

/// Summary information for a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSummary {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, rename = "isGroup")]
    pub is_group: bool,
    #[serde(default, rename = "unreadCount")]
    pub unread_count: u32,
    #[serde(default)]
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub archived: bool,
    #[serde(default, rename = "isMuted")]
    pub is_muted: bool,
    #[serde(default, rename = "muteExpiration")]
    pub mute_expiration: Option<i64>,
}

/// Query parameters for listing chats.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListChatsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// Request to mark a chat as read.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkChatReadRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
}

/// Request to mark a chat (archive / pin / delete / unread).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarkChatRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seen: Option<bool>,
}

/// Request to mute a chat until an epoch-milliseconds timestamp, or null to unmute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuteChatRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    /// Epoch MILLISECONDS timestamp to mute until, or None/null to unmute.
    #[serde(rename = "muteExpiration")]
    pub mute_expiration: Option<i64>,
}

/// Chat typing/recording presence state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatState {
    Typing,
    Recording,
    Paused,
}

/// Request to send a presence state in a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendChatStateRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub state: ChatState,
}

/// Request to subscribe to chat presence updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribePresenceRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
}

/// Presence state of a participant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantPresence {
    pub id: String,
    pub state: String, // "available", "unavailable", "composing", "recording", "paused"
    #[serde(default, rename = "lastSeen", skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<i64>,
}

/// Reported presence for a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPresence {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(default)]
    pub participants: Vec<ParticipantPresence>,
    #[serde(
        default,
        rename = "groupOnlineCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_online_count: Option<usize>,
    #[serde(rename = "observedAt")]
    pub observed_at: String,
}
