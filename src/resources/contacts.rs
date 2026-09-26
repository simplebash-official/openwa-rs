use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

#[derive(Clone)]
pub struct ContactsResource {
    pub(crate) transport: Arc<Transport>,
}

impl ContactsResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// List contacts known to the session.
    pub async fn list(
        &self,
        session_id: &str,
        query: Option<ListContactsQuery>,
    ) -> Result<Vec<ContactRecord>, OpenWAError> {
        let path = format!("/api/sessions/{}/contacts", encode_path_segment(session_id));
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

    /// List IDs blocked by this account.
    pub async fn blocked(&self, session_id: &str) -> Result<Vec<String>, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/contacts/blocked",
            encode_path_segment(session_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Check whether a phone number is registered on WhatsApp.
    pub async fn check(
        &self,
        session_id: &str,
        number: &str,
    ) -> Result<CheckNumberResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/contacts/check/{}",
            encode_path_segment(session_id),
            encode_path_segment(number)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Get details of a single contact by JID.
    pub async fn get(
        &self,
        session_id: &str,
        contact_id: &str,
    ) -> Result<ContactRecord, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/contacts/{}",
            encode_path_segment(session_id),
            encode_path_segment(contact_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Get the contact's profile picture URL.
    pub async fn profile_picture(
        &self,
        session_id: &str,
        contact_id: &str,
    ) -> Result<ProfilePictureResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/contacts/{}/profile-picture",
            encode_path_segment(session_id),
            encode_path_segment(contact_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Batch-resolve profile picture URLs for up to 50 contacts.
    pub async fn profile_pictures(
        &self,
        session_id: &str,
        ids: &[&str],
    ) -> Result<ProfilePicturesResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/contacts/profile-pictures",
            encode_path_segment(session_id)
        );
        let joined_ids = ids.join(",");
        let query = [("ids", joined_ids.as_str())];
        self.transport
            .execute(Method::GET, &path, Some(&query), None)
            .await
    }

    /// Resolve a contact privacy ID (@lid) to an MSISDN phone number.
    pub async fn phone(
        &self,
        session_id: &str,
        contact_id: &str,
    ) -> Result<ContactPhoneResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/contacts/{}/phone",
            encode_path_segment(session_id),
            encode_path_segment(contact_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Save or edit a contact in the account address book. (Requires OPERATOR role).
    pub async fn upsert(
        &self,
        session_id: &str,
        contact_id: &str,
        req: UpsertContactRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/contacts/{}",
            encode_path_segment(session_id),
            encode_path_segment(contact_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Delete a contact from the account address book. (Requires OPERATOR role).
    pub async fn delete(
        &self,
        session_id: &str,
        contact_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/contacts/{}",
            encode_path_segment(session_id),
            encode_path_segment(contact_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }

    /// Block a contact. (Requires OPERATOR role).
    pub async fn block(
        &self,
        session_id: &str,
        contact_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/contacts/{}/block",
            encode_path_segment(session_id),
            encode_path_segment(contact_id)
        );
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }

    /// Unblock a contact. (Requires OPERATOR role).
    pub async fn unblock(
        &self,
        session_id: &str,
        contact_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/contacts/{}/block",
            encode_path_segment(session_id),
            encode_path_segment(contact_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }
}
