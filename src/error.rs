use std::time::Duration;
use thiserror::Error;

/// The primary error type returned by the OpenWA SDK.
#[derive(Debug, Error)]
pub enum OpenWAError {
    /// 401 Unauthorized - Missing, invalid, or expired API key, or blocked IP.
    #[error("Authentication error (401): {0}")]
    Auth(OpenWAApiError),

    /// 403 Forbidden - Insufficient role, or API key restricted to other chats/sessions.
    #[error("Forbidden error (403): {0}")]
    Forbidden(OpenWAApiError),

    /// 404 Not Found - Addressed resource (session, message, chat, etc.) not found.
    #[error("Not found error (404): {0}")]
    NotFound(OpenWAApiError),

    /// 409 Conflict - Engine not ready, session name collision, or multi-node lock conflict.
    #[error("Conflict error (409): {0}")]
    Conflict(OpenWAApiError),

    /// 429 Too Many Requests - Global rate limiter or send pacing limit.
    #[error("Rate limit error (429): {0}")]
    RateLimited(Box<OpenWARateLimitError>),

    /// 501 Not Implemented - Active engine does not support this operation.
    #[error("Not implemented error (501): {0}")]
    NotImplemented(OpenWAApiError),

    /// 502 Bad Gateway - WhatsApp engine socket failure or upstream gateway failure.
    #[error("Bad gateway error (502): {0}")]
    BadGateway(OpenWAApiError),

    /// 503 Service Unavailable - Engine is reconnecting, booting, or temporary outage.
    #[error("Service unavailable error (503): {0}")]
    ServiceUnavailable(OpenWAApiError),

    /// 504 Gateway Timeout - Upstream WhatsApp Web or forwarded node did not respond in time.
    #[error("Gateway timeout error (504): {0}")]
    GatewayTimeout(OpenWAApiError),

    /// Any other non-2xx status code from the OpenWA API gateway.
    #[error("API error: {0}")]
    Api(OpenWAApiError),

    /// Request timed out.
    #[error("Request timed out after {timeout:?}: {context}")]
    Timeout {
        timeout: Duration,
        context: String,
        #[source]
        source: Option<reqwest::Error>,
    },

    /// Network or transport level failure (e.g., DNS, connection refused).
    #[error("Transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// JSON serialization or deserialization failure.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Data decoding error (e.g. invalid UTF-8).
    #[error("Decoding error: {0}")]
    Decoding(String),

    /// WebSocket error during real-time event streaming.
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// Client configuration error (e.g. invalid URL, missing API key).
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Details of a non-2xx API response from OpenWA.
#[derive(Debug, Clone, Error)]
#[error("API {status} [{context}]: {message}")]
pub struct OpenWAApiError {
    /// HTTP status code.
    pub status: u16,
    /// Human-readable message parsed from the response body.
    pub message: String,
    /// NestJS error field (e.g. "Not Found", "Conflict"), if present.
    pub kind: Option<String>,
    /// Full parsed JSON body, or string value if not JSON.
    pub body: serde_json::Value,
    /// Request context in the format "METHOD /path".
    pub context: String,
}

/// Details of a 429 Rate Limit response, including send pacing information.
#[derive(Debug, Clone, Error)]
#[error("API 429 [{context}]: {message}")]
pub struct OpenWARateLimitError {
    /// HTTP status code (429).
    pub status: u16,
    /// Human-readable message.
    pub message: String,
    /// NestJS error field.
    pub kind: Option<String>,
    /// Full parsed body.
    pub body: serde_json::Value,
    /// Request context.
    pub context: String,
    /// True if this is a send pacing restriction (`code: "SEND_PACING_LIMITED"`).
    pub is_send_pacing: bool,
    /// Retry delay in seconds, if specified by the server in body or Retry-After header.
    pub retry_after_seconds: Option<u64>,
}

impl OpenWAError {
    /// Classifies an HTTP status and raw response body into the most specific typed `OpenWAError`.
    pub fn from_response(
        status: reqwest::StatusCode,
        raw: &[u8],
        context: &str,
        retry_after_hdr: Option<&str>,
    ) -> Self {
        let code = status.as_u16();
        let (body, message, kind) = parse_error_body(raw);

        if code == 429 {
            let is_send_pacing =
                body.get("code").and_then(|v| v.as_str()) == Some("SEND_PACING_LIMITED");
            let retry_after_seconds = body
                .get("retryAfterSeconds")
                .and_then(|v| v.as_u64())
                .or_else(|| retry_after_hdr.and_then(|s| s.parse::<u64>().ok()));

            return OpenWAError::RateLimited(Box::new(OpenWARateLimitError {
                status: code,
                message,
                kind,
                body,
                context: context.to_string(),
                is_send_pacing,
                retry_after_seconds,
            }));
        }

        let api_err = OpenWAApiError {
            status: code,
            message,
            kind,
            body,
            context: context.to_string(),
        };

        match code {
            401 => OpenWAError::Auth(api_err),
            403 => OpenWAError::Forbidden(api_err),
            404 => OpenWAError::NotFound(api_err),
            409 => OpenWAError::Conflict(api_err),
            501 => OpenWAError::NotImplemented(api_err),
            502 => OpenWAError::BadGateway(api_err),
            503 => OpenWAError::ServiceUnavailable(api_err),
            504 => OpenWAError::GatewayTimeout(api_err),
            _ => OpenWAError::Api(api_err),
        }
    }

    /// Returns the HTTP status code if this error originated from an HTTP response.
    pub fn status_code(&self) -> Option<u16> {
        match self {
            OpenWAError::Auth(e)
            | OpenWAError::Forbidden(e)
            | OpenWAError::NotFound(e)
            | OpenWAError::Conflict(e)
            | OpenWAError::NotImplemented(e)
            | OpenWAError::BadGateway(e)
            | OpenWAError::ServiceUnavailable(e)
            | OpenWAError::GatewayTimeout(e)
            | OpenWAError::Api(e) => Some(e.status),
            OpenWAError::RateLimited(e) => Some(e.status),
            _ => None,
        }
    }
}

fn parse_error_body(raw: &[u8]) -> (serde_json::Value, String, Option<String>) {
    if raw.is_empty() {
        return (
            serde_json::Value::Null,
            "Empty response body".to_string(),
            None,
        );
    }

    if let Ok(val) = serde_json::from_slice::<serde_json::Value>(raw) {
        let kind = val
            .get("error")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let message = if let Some(msg) = val.get("message") {
            if let Some(s) = msg.as_str() {
                s.to_string()
            } else if let Some(arr) = msg.as_array() {
                let items: Vec<String> = arr
                    .iter()
                    .filter_map(|x| x.as_str().map(|s| s.to_string()))
                    .collect();
                items.join("; ")
            } else {
                msg.to_string()
            }
        } else if let Some(k) = &kind {
            k.clone()
        } else {
            val.to_string()
        };

        (val, message, kind)
    } else {
        let text = String::from_utf8_lossy(raw).to_string();
        (serde_json::Value::String(text.clone()), text, None)
    }
}
