use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

/// Profile management resource (push name, status/about, avatar).
#[derive(Clone)]
pub struct ProfileResource {
    pub(crate) transport: Arc<Transport>,
}

impl ProfileResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// Set the session display / push name.
    pub async fn set_name(
        &self,
        session_id: &str,
        req: SetProfileNameRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/profile/name",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Set the session status (About info text).
    pub async fn set_status(
        &self,
        session_id: &str,
        req: SetProfileStatusRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/profile/status",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Set the profile picture / avatar.
    pub async fn set_picture(
        &self,
        session_id: &str,
        req: SetProfilePictureRequest,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/profile/picture",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::PUT, &path, None, Some(body))
            .await
    }

    /// Delete the profile picture / avatar.
    pub async fn delete_picture(&self, session_id: &str) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/profile/picture",
            encode_path_segment(session_id)
        );
        self.transport
            .execute(Method::DELETE, &path, None, None)
            .await
    }
}
