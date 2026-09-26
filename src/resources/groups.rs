use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

#[derive(Clone)]
pub struct GroupsResource {
    pub(crate) transport: Arc<Transport>,
}

impl GroupsResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List all groups for a session.
    pub async fn list(
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

    /// Get detailed group info including participants.
    pub async fn get(&self, session_id: &str, group_id: &str) -> Result<GroupInfo, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Create a new group. (Requires OPERATOR role).
    pub async fn create(
        &self,
        session_id: &str,
        req: CreateGroupRequest,
    ) -> Result<GroupInfo, OpenWAError> {
        let path = format!("/api/sessions/{}/groups", encode_path_segment(session_id));
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Preview a group from its invite code WITHOUT joining.
    pub async fn join_info(
        &self,
        session_id: &str,
        code: &str,
    ) -> Result<GroupJoinInfo, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/join-info",
            encode_path_segment(session_id)
        );
        let query = [("code", code)];
        self.transport
            .execute(Method::GET, &path, Some(&query), None)
            .await
    }

    /// Join a group via its invite code. (Requires OPERATOR role).
    pub async fn join(
        &self,
        session_id: &str,
        req: JoinGroupRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/join",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Add participants to a group. (Requires OPERATOR role).
    pub async fn add_participants(
        &self,
        session_id: &str,
        group_id: &str,
        req: ParticipantsRequest,
    ) -> Result<ParticipantsResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/participants",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Remove participants from a group. (Requires OPERATOR role).
    pub async fn remove_participants(
        &self,
        session_id: &str,
        group_id: &str,
        req: ParticipantsRequest,
    ) -> Result<ParticipantsResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/participants",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::DELETE, &path, None, Some(body))
            .await
    }

    /// Promote participants to group admin. (Requires OPERATOR role).
    pub async fn promote_participants(
        &self,
        session_id: &str,
        group_id: &str,
        req: ParticipantsRequest,
    ) -> Result<ParticipantsResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/participants/promote",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Demote participants from group admin. (Requires OPERATOR role).
    pub async fn demote_participants(
        &self,
        session_id: &str,
        group_id: &str,
        req: ParticipantsRequest,
    ) -> Result<ParticipantsResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/participants/demote",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Update group subject (name). (Requires OPERATOR role).
    pub async fn set_subject(
        &self,
        session_id: &str,
        group_id: &str,
        req: SetSubjectRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/subject",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Update group description. (Requires OPERATOR role).
    pub async fn set_description(
        &self,
        session_id: &str,
        group_id: &str,
        req: SetDescriptionRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/description",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Leave a group. (Requires OPERATOR role).
    pub async fn leave(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/leave",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Get group picture URL.
    pub async fn get_picture(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<ProfilePictureResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/picture",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Set group picture (url or base64). (Requires OPERATOR role).
    pub async fn set_picture(
        &self,
        session_id: &str,
        group_id: &str,
        req: SetGroupPictureRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/picture",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Delete group picture. (Requires OPERATOR role).
    pub async fn delete_picture(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/picture",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }

    /// Get group invite code. (Requires OPERATOR role).
    pub async fn invite_code(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<InviteCodeResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/invite-code",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Revoke current invite code and generate a new one. (Requires OPERATOR role).
    pub async fn revoke_invite_code(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<InviteCodeResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/invite-code/revoke",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Get group settings.
    pub async fn get_settings(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<GroupSettings, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/settings",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Update group settings. (Requires OPERATOR role).
    pub async fn update_settings(
        &self,
        session_id: &str,
        group_id: &str,
        req: UpdateGroupSettingsRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/settings",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// List pending group join requests.
    pub async fn get_membership_requests(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<Vec<GroupMembershipRequest>, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/membership-requests",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Approve pending group join requests. (Requires OPERATOR role).
    pub async fn approve_membership_requests(
        &self,
        session_id: &str,
        group_id: &str,
        req: MembershipRequestActionRequest,
    ) -> Result<ParticipantsResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/membership-requests/approve",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Reject pending group join requests. (Requires OPERATOR role).
    pub async fn reject_membership_requests(
        &self,
        session_id: &str,
        group_id: &str,
        req: MembershipRequestActionRequest,
    ) -> Result<ParticipantsResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/groups/{}/membership-requests/reject",
            encode_path_segment(session_id),
            encode_path_segment(group_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }
}
