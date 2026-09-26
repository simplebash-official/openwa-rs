use serde::{Deserialize, Serialize};

/// Channel / Newsletter record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelRecord {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        default,
        rename = "inviteCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub invite_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default)]
    pub verified: bool,
    #[serde(
        default,
        rename = "subscribersCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscribers_count: Option<u64>,
}

/// Channel message record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMessage {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub timestamp: i64,
}

/// Request to create a new channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to mute or unmute a channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuteChannelRequest {
    pub mute: bool,
}

/// Request to subscribe to a channel using its invite code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeChannelRequest {
    #[serde(rename = "inviteCode")]
    pub invite_code: String,
}

/// Request to demote an admin in a channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoteChannelAdminRequest {
    #[serde(rename = "userId", alias = "participantId")]
    pub user_id: String,
}

impl DemoteChannelAdminRequest {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }
}

/// Request to transfer channel ownership.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferChannelOwnershipRequest {
    #[serde(rename = "newOwnerId")]
    pub new_owner_id: String,
}

/// Query parameters for listing channel messages.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListChannelMessagesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
