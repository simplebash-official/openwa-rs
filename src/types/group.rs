use serde::{Deserialize, Serialize};

/// Summary information for a group in a list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSummary {
    pub id: String,
    pub subject: String,
    #[serde(default, rename = "creation")]
    pub creation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, rename = "participantCount")]
    pub participant_count: Option<usize>,
}

/// Detailed group participant record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupParticipant {
    pub id: String,
    #[serde(default, rename = "isAdmin")]
    pub is_admin: bool,
    #[serde(default, rename = "isSuperAdmin")]
    pub is_super_admin: bool,
}

/// Detailed group information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInfo {
    pub id: String,
    pub subject: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default)]
    pub participants: Vec<GroupParticipant>,
    #[serde(default, rename = "creation")]
    pub creation: Option<i64>,
}

/// Preview of a group via invite code without joining.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupJoinInfo {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, rename = "createdAt")]
    pub created_at: Option<i64>,
    #[serde(default, rename = "participantCount")]
    pub participant_count: Option<usize>,
}

/// Group settings (announce mode, locked settings, ephemeral duration, member add mode).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSettings {
    #[serde(default)]
    pub announce: Option<bool>,
    #[serde(default)]
    pub locked: Option<bool>,
    #[serde(default, rename = "ephemeralSeconds", alias = "ephemeralDuration")]
    pub ephemeral_seconds: Option<u32>,
    #[serde(default, rename = "memberAddMode")]
    pub member_add_mode: Option<String>,
}

/// Request to update group settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGroupSettingsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub announce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(
        default,
        rename = "ephemeralSeconds",
        alias = "ephemeralDuration",
        skip_serializing_if = "Option::is_none"
    )]
    pub ephemeral_seconds: Option<u32>,
    #[serde(
        default,
        rename = "memberAddMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_add_mode: Option<String>,
}

/// Pending group membership request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMembershipRequest {
    pub id: String,
    #[serde(default, rename = "requestMethod")]
    pub request_method: Option<String>,
    #[serde(default, rename = "requestTime")]
    pub request_time: Option<i64>,
}

/// Group invite code response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteCodeResponse {
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

/// Request to create a new group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    #[serde(rename = "name", alias = "subject")]
    pub name: String,
    pub participants: Vec<String>,
}

impl CreateGroupRequest {
    pub fn new(name: impl Into<String>, participants: Vec<String>) -> Self {
        Self {
            name: name.into(),
            participants,
        }
    }

    pub fn subject(&self) -> &str {
        &self.name
    }
}

/// Request to join a group via invite code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinGroupRequest {
    #[serde(rename = "inviteCode", alias = "code")]
    pub invite_code: String,
}

impl JoinGroupRequest {
    pub fn new(invite_code: impl Into<String>) -> Self {
        Self {
            invite_code: invite_code.into(),
        }
    }

    pub fn code(&self) -> &str {
        &self.invite_code
    }
}

/// Request to update group subject.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSubjectRequest {
    pub subject: String,
}

/// Request to update group description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDescriptionRequest {
    pub description: String,
}

/// Request to update group picture (url or base64).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetGroupPictureRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
}

/// Request specifying participants for add/remove/promote/demote actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantsRequest {
    pub participants: Vec<String>,
}

/// Request to approve or reject membership requests.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MembershipRequestActionRequest {
    /// Omit to approve/reject all pending requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<String>>,
}
