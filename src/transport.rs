use crate::error::OpenWAError;
use crate::retry::RetryPolicy;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// Percent-encode character set preserving WhatsApp-safe characters (@, :, +)
/// while escaping path breakout characters (/, #, ?, etc.).
const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}')
    .add(b'/');

pub fn encode_path_segment(segment: &str) -> String {
    utf8_percent_encode(segment, PATH_SEGMENT_ENCODE_SET).to_string()
}

#[derive(Clone)]
pub struct Transport {
    pub base_url: String,
    pub api_key: String,
    pub client: Client,
    pub default_headers: HeaderMap,
    pub timeout: Duration,
    pub retry_policy: Option<RetryPolicy>,
}

impl Transport {
    pub fn new(
        base_url: String,
        api_key: String,
        timeout: Duration,
        default_headers: HeaderMap,
        retry_policy: Option<RetryPolicy>,
    ) -> Result<Self, OpenWAError> {
        let base = base_url.trim_end_matches('/').to_string();
        if base.is_empty() {
            return Err(OpenWAError::Config("Base URL cannot be empty".to_string()));
        }
        if api_key.trim().is_empty() {
            return Err(OpenWAError::Config("API key cannot be empty".to_string()));
        }

        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(timeout)
            .build()?;

        Ok(Self {
            base_url: base,
            api_key,
            client,
            default_headers,
            timeout,
            retry_policy,
        })
    }

    /// Executes an HTTP request, automatically applying auth, JSON headers, retries, and deserializing the result.
    pub async fn execute<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<serde_json::Value>,
    ) -> Result<T, OpenWAError> {
        let raw_bytes = self.execute_raw(method, path, query, body).await?;
        if raw_bytes.is_empty() {
            // For endpoints returning empty body (or 204), deserialize from JSON null
            return serde_json::from_str("null").map_err(OpenWAError::from);
        }
        serde_json::from_slice(&raw_bytes).map_err(OpenWAError::from)
    }

    /// Executes an HTTP request and returns the raw response bytes (e.g. for binary media downloads).
    pub async fn execute_raw(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<serde_json::Value>,
    ) -> Result<Vec<u8>, OpenWAError> {
        let full_url = format!("{}{}", self.base_url, path);
        let context = format!("{} {}", method, path);

        let mut attempt = 0;
        loop {
            let mut req = self.client.request(method.clone(), &full_url);

            // 1. Merge caller-supplied default headers first
            for (k, v) in self.default_headers.iter() {
                req = req.header(k, v);
            }

            // 2. Set Content-Type, Accept, and X-API-Key with strict precedence
            req = req.header(ACCEPT, "application/json").header(
                HeaderName::from_static("x-api-key"),
                HeaderValue::from_str(&self.api_key)
                    .map_err(|e| OpenWAError::Config(e.to_string()))?,
            );

            if let Some(ref q) = query {
                req = req.query(q);
            }

            if let Some(ref b) = body {
                req = req.header(CONTENT_TYPE, "application/json").json(b);
            }

            match req.send().await {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        let bytes = resp.bytes().await?.to_vec();
                        return Ok(bytes);
                    }

                    // Handle non-2xx status code
                    let retry_after_hdr = resp
                        .headers()
                        .get("retry-after")
                        .and_then(|h| h.to_str().ok())
                        .map(|s| s.to_string());
                    let bytes = resp.bytes().await.unwrap_or_default().to_vec();
                    let classified_err = OpenWAError::from_response(
                        status,
                        &bytes,
                        &context,
                        retry_after_hdr.as_deref(),
                    );

                    // Check if retry is allowed
                    if let Some(ref policy) = self.retry_policy {
                        if attempt < policy.max_retries {
                            // Never retry send pacing refusal
                            if let OpenWAError::RateLimited(ref rl) = classified_err {
                                if rl.is_send_pacing {
                                    return Err(classified_err);
                                }
                            }

                            let code = status.as_u16();
                            let is_retryable_code = policy.retryable_statuses.contains(&code);

                            if is_retryable_code
                                && RetryPolicy::can_retry_method_on_status(&method, code)
                            {
                                let retry_after = retry_after_hdr
                                    .as_deref()
                                    .and_then(|s| s.parse::<u64>().ok())
                                    .map(Duration::from_secs);
                                let delay = policy.backoff_delay(attempt, retry_after);
                                attempt += 1;
                                tokio::time::sleep(delay).await;
                                continue;
                            }
                        }
                    }

                    return Err(classified_err);
                }
                Err(err) => {
                    // Network or timeout failure
                    if let Some(ref policy) = self.retry_policy {
                        if attempt < policy.max_retries && RetryPolicy::is_idempotent(&method) {
                            // Only retry idempotent methods on network failures
                            let delay = policy.backoff_delay(attempt, None);
                            attempt += 1;
                            tokio::time::sleep(delay).await;
                            continue;
                        }
                    }

                    if err.is_timeout() {
                        return Err(OpenWAError::Timeout {
                            timeout: self.timeout,
                            context,
                            source: Some(err),
                        });
                    }

                    return Err(OpenWAError::Transport(err));
                }
            }
        }
    }
}
