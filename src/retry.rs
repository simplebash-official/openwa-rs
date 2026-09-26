use reqwest::Method;
use std::time::Duration;

/// Configuration for automatic request retries.
///
/// Retries are disabled by default. When enabled with `OpenWAClientBuilder::retry_policy`,
/// requests are retried according to safe HTTP and WhatsApp gateway semantics.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts after the initial request failure.
    pub max_retries: u32,
    /// Base delay for exponential backoff (e.g. 200ms).
    pub base_delay: Duration,
    /// Maximum delay for backoff cap (e.g. 5s).
    pub max_delay: Duration,
    /// HTTP status codes that trigger a retry.
    pub retryable_statuses: Vec<u16>,
    /// Whether to honor the `Retry-After` header when received.
    pub respect_retry_after: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(5),
            retryable_statuses: vec![429, 500, 502, 503, 504],
            respect_retry_after: true,
        }
    }
}

impl RetryPolicy {
    /// Creates a default retry policy (3 retries, 200ms base, 5s max, 429/500/502/503/504).
    pub fn standard() -> Self {
        Self::default()
    }

    /// Determines if a method is idempotent under RFC 7231 (safe to retry on network errors).
    pub fn is_idempotent(method: &Method) -> bool {
        matches!(
            *method,
            Method::GET | Method::HEAD | Method::OPTIONS | Method::PUT | Method::DELETE
        )
    }

    /// Determines if a non-idempotent method (POST/PATCH) can retry on the given HTTP status.
    ///
    /// POST/PATCH requests are NEVER retried after network errors (to avoid duplicate WhatsApp messages).
    /// On explicit HTTP statuses, only backpressure statuses (429 and 503) are retryable because
    /// the gateway rejected the request before acting on it.
    pub fn can_retry_method_on_status(method: &Method, status: u16) -> bool {
        if Self::is_idempotent(method) {
            true
        } else {
            // Non-idempotent: only 429 and 503
            status == 429 || status == 503
        }
    }

    /// Calculates backoff delay for the given attempt index (0-indexed).
    pub fn backoff_delay(&self, attempt: u32, retry_after: Option<Duration>) -> Duration {
        let mut delay = self.base_delay.saturating_mul(1 << attempt.min(10));
        if delay > self.max_delay {
            delay = self.max_delay;
        }

        if self.respect_retry_after {
            if let Some(ra) = retry_after {
                if ra > delay {
                    delay = ra;
                }
            }
        }

        delay
    }
}
