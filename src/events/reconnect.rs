use crate::error::OpenWAError;
use crate::events::client::EventStream;
use crate::events::types::WSServerMessage;
use std::time::Duration;

/// Configuration for reconnecting WebSocket event streams.
#[derive(Debug, Clone)]
pub struct ReconnectConfig {
    /// Maximum number of reconnection attempts before giving up (default: 10).
    pub max_retries: u32,
    /// Base delay before the first reconnection attempt (default: 500ms).
    pub base_delay: Duration,
    /// Maximum backoff delay cap (default: 30s).
    pub max_delay: Duration,
}

impl Default for ReconnectConfig {
    fn default() -> Self {
        Self {
            max_retries: 10,
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
        }
    }
}

/// A resilient, auto-reconnecting WebSocket event stream for OpenWA Socket.IO.
///
/// Automatically handles network disconnections and server restarts by reconnecting
/// with exponential backoff and re-subscribing to all active session filters.
pub struct ReconnectingEventStream {
    base_url: String,
    api_key: String,
    active_stream: Option<EventStream>,
    subscriptions: Vec<(String, Vec<String>)>,
    config: ReconnectConfig,
}

impl ReconnectingEventStream {
    /// Establish a new auto-reconnecting event stream.
    pub async fn connect(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        config: Option<ReconnectConfig>,
    ) -> Result<Self, OpenWAError> {
        let base_url = base_url.into();
        let api_key = api_key.into();
        let config = config.unwrap_or_default();

        let stream = EventStream::connect(&base_url, &api_key).await?;

        Ok(Self {
            base_url,
            api_key,
            active_stream: Some(stream),
            subscriptions: Vec::new(),
            config,
        })
    }

    /// Register a subscription filter for a session and event types.
    ///
    /// Use `"*"` to subscribe to all sessions, and `&["*"]` to subscribe to all event types.
    /// Subscriptions are automatically replayed if the stream reconnects.
    pub fn subscribe(&mut self, session_id: &str, events: &[&str]) -> Result<(), OpenWAError> {
        let events_vec: Vec<String> = events.iter().map(|s| s.to_string()).collect();

        // Update or add to stored subscriptions
        if let Some(pos) = self.subscriptions.iter().position(|(s, _)| s == session_id) {
            self.subscriptions[pos].1 = events_vec.clone();
        } else {
            self.subscriptions
                .push((session_id.to_string(), events_vec.clone()));
        }

        if let Some(ref mut stream) = self.active_stream {
            stream.subscribe(session_id, events)?;
        }

        Ok(())
    }

    /// Read the next server message from the stream, automatically reconnecting if disconnected.
    pub async fn next_message(&mut self) -> Option<WSServerMessage> {
        loop {
            if let Some(ref mut stream) = self.active_stream {
                if let Some(msg) = stream.next_message().await {
                    return Some(msg);
                }
                // Stream ended/disconnected
                self.active_stream = None;
            }

            // Attempt reconnection
            let mut reconnected = false;
            for attempt in 0..self.config.max_retries {
                let delay = self.compute_backoff(attempt);
                tokio::time::sleep(delay).await;

                match EventStream::connect(&self.base_url, &self.api_key).await {
                    Ok(new_stream) => {
                        // Re-subscribe to all recorded filters
                        let mut sub_err = false;
                        for (sess, evts) in &self.subscriptions {
                            let str_refs: Vec<&str> = evts.iter().map(|s| s.as_str()).collect();
                            if new_stream.subscribe(sess, &str_refs).is_err() {
                                sub_err = true;
                                break;
                            }
                        }

                        if !sub_err {
                            self.active_stream = Some(new_stream);
                            reconnected = true;
                            break;
                        }
                    }
                    Err(_) => {
                        // Retry on next backoff interval
                    }
                }
            }

            if !reconnected {
                // Max retries exceeded
                return None;
            }
        }
    }

    fn compute_backoff(&self, attempt: u32) -> Duration {
        let factor = 2u64.saturating_pow(attempt);
        let millis = self
            .config
            .base_delay
            .as_millis()
            .saturating_mul(factor as u128);
        let delay = Duration::from_millis(millis as u64);
        if delay > self.config.max_delay {
            self.config.max_delay
        } else {
            delay
        }
    }
}
