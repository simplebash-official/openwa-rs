use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::{CallLinkRequest, CallLinkResponse, SuccessResult};
use reqwest::Method;
use std::sync::Arc;

/// WhatsApp voice/video calls resource.
#[derive(Clone)]
pub struct CallsResource {
    pub(crate) transport: Arc<Transport>,
}

impl CallsResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// Create a WhatsApp call link.
    pub async fn create_link(
        &self,
        session_id: &str,
        req: CallLinkRequest,
    ) -> Result<CallLinkResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/calls/link",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }

    /// Reject an incoming call.
    pub async fn reject(
        &self,
        session_id: &str,
        call_id: &str,
    ) -> Result<SuccessResult, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/calls/{}/reject",
            encode_path_segment(session_id),
            encode_path_segment(call_id)
        );
        self.transport
            .execute(Method::POST, &path, None, None)
            .await
    }
}
