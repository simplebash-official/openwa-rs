use crate::error::OpenWAError;
use crate::events::types::{
    WSEventMessage, WSPingRequest, WSServerMessage, WSSubscribeRequest, WSUnsubscribeRequest,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

/// Real-time WebSocket Socket.IO event stream for OpenWA.
pub struct EventStream {
    tx_cmd: mpsc::UnboundedSender<String>,
    rx_events: mpsc::UnboundedReceiver<WSServerMessage>,
    is_closed: Arc<AtomicBool>,
}

impl EventStream {
    /// Connect to the OpenWA gateway `/events` Socket.IO WebSocket endpoint.
    ///
    /// `base_url` should be the HTTP or WS base URL (e.g. `http://localhost:3000` or `ws://localhost:3000`).
    /// `api_key` is the authentication key.
    pub async fn connect(base_url: &str, api_key: &str) -> Result<Self, OpenWAError> {
        let ws_url = Self::build_ws_url(base_url)?;
        let mut request = ws_url.to_string().into_client_request().map_err(|e| {
            OpenWAError::Config(format!("Failed to build WebSocket request: {}", e))
        })?;

        // Add headers for authentication
        let headers = request.headers_mut();
        headers.insert(
            "X-API-Key",
            api_key
                .parse()
                .map_err(|e| OpenWAError::Config(format!("Invalid API key header value: {}", e)))?,
        );

        let (ws_stream, _) = connect_async(request)
            .await
            .map_err(|e| OpenWAError::WebSocket(e.to_string()))?;

        let (mut write_half, mut read_half) = ws_stream.split();

        // 1. Wait for Engine.IO open packet ("0{...}")
        let open_packet = read_half
            .next()
            .await
            .ok_or_else(|| OpenWAError::Config("WebSocket closed before handshake".to_string()))?
            .map_err(|e| OpenWAError::WebSocket(e.to_string()))?;

        let open_text = open_packet.to_text().map_err(|e| {
            OpenWAError::Config(format!("Engine.IO handshake packet was not text: {}", e))
        })?;

        if !open_text.starts_with('0') {
            return Err(OpenWAError::Config(format!(
                "Unexpected Engine.IO open packet: {}",
                open_text
            )));
        }

        // 2. Send Socket.IO connect packet to /events namespace with auth payload:
        //    "40/events,{\"apiKey\":\"...\"}"
        let connect_payload = serde_json::json!({
            "apiKey": api_key,
        });
        let connect_packet = format!("40/events,{}", connect_payload);
        write_half
            .send(Message::Text(connect_packet))
            .await
            .map_err(|e| OpenWAError::WebSocket(e.to_string()))?;

        // 3. Wait for Socket.IO connect ack ("40/events,{...}")
        let mut connected = false;
        while let Some(msg) = read_half.next().await {
            let msg = msg.map_err(|e| OpenWAError::WebSocket(e.to_string()))?;
            if let Ok(text) = msg.to_text() {
                if text.starts_with("40/events") {
                    connected = true;
                    break;
                } else if text.starts_with("44/events") {
                    return Err(OpenWAError::WebSocket(format!(
                        "Socket.IO connection rejected on /events: {}",
                        text
                    )));
                } else if text.starts_with('2') {
                    // Engine.IO ping received, respond with pong
                    let _ = write_half.send(Message::Text("3".to_string())).await;
                }
            }
        }

        if !connected {
            return Err(OpenWAError::Config(
                "Socket.IO failed to connect to /events namespace".to_string(),
            ));
        }

        let (tx_cmd, mut rx_cmd) = mpsc::unbounded_channel::<String>();
        let (tx_events, rx_events) = mpsc::unbounded_channel::<WSServerMessage>();
        let is_closed = Arc::new(AtomicBool::new(false));
        let is_closed_clone = is_closed.clone();

        // Spawn background handler task
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(cmd) = rx_cmd.recv() => {
                        let packet = format!("42/events,[\"message\",{}]", cmd);
                        if let Err(e) = write_half.send(Message::Text(packet)).await {
                            tracing::warn!("Failed to send packet over websocket: {}", e);
                            break;
                        }
                    }
                    Some(msg_res) = read_half.next() => {
                        match msg_res {
                            Ok(Message::Text(text)) => {
                                if text == "2" || text == "2probe" {
                                    // Engine.IO ping -> respond with pong
                                    let pong = if text == "2probe" { "3probe" } else { "3" };
                                    let _ = write_half.send(Message::Text(pong.to_string())).await;
                                } else if let Some(payload) = text.strip_prefix("42/events,") {
                                    if let Ok(serde_json::Value::Array(arr)) = serde_json::from_str::<serde_json::Value>(payload) {
                                        if arr.len() >= 2 && arr[0].as_str() == Some("message") {
                                            if let Ok(parsed) = serde_json::from_value::<WSServerMessage>(arr[1].clone()) {
                                                let _ = tx_events.send(parsed);
                                            }
                                        }
                                    }
                                } else if text.starts_with("41/events") {

                                    // Disconnected from namespace
                                    break;
                                }
                            }
                            Ok(Message::Ping(p)) => {
                                let _ = write_half.send(Message::Pong(p)).await;
                            }
                            Ok(Message::Close(_)) => {
                                break;
                            }
                            Err(_) => {
                                break;
                            }
                            _ => {}
                        }
                    }
                    else => break,
                }
            }
            is_closed_clone.store(true, Ordering::SeqCst);
        });

        Ok(Self {
            tx_cmd,
            rx_events,
            is_closed,
        })
    }

    /// Subscribe to real-time events for a specific session or all sessions ("*").
    pub fn subscribe(&self, session_id: &str, events: &[&str]) -> Result<(), OpenWAError> {
        let req =
            WSSubscribeRequest::new(session_id, events.iter().map(|s| s.to_string()).collect());
        let json = serde_json::to_string(&req)?;
        self.tx_cmd
            .send(json)
            .map_err(|_| OpenWAError::Config("Event stream is closed".to_string()))
    }

    /// Unsubscribe from a session's events.
    pub fn unsubscribe(&self, session_id: &str) -> Result<(), OpenWAError> {
        let req = WSUnsubscribeRequest::new(session_id);
        let json = serde_json::to_string(&req)?;
        self.tx_cmd
            .send(json)
            .map_err(|_| OpenWAError::Config("Event stream is closed".to_string()))
    }

    /// Send a heartbeat ping request.
    pub fn ping(&self) -> Result<(), OpenWAError> {
        let req = WSPingRequest::default();
        let json = serde_json::to_string(&req)?;
        self.tx_cmd
            .send(json)
            .map_err(|_| OpenWAError::Config("Event stream is closed".to_string()))
    }

    /// Receive the next incoming server message or event.
    pub async fn next_message(&mut self) -> Option<WSServerMessage> {
        self.rx_events.recv().await
    }

    /// Receive the next event notification, skipping ack/pong frames.
    pub async fn next_event(&mut self) -> Option<WSEventMessage> {
        while let Some(msg) = self.rx_events.recv().await {
            if let WSServerMessage::Event { payload, timestamp } = msg {
                return Some(WSEventMessage {
                    message_type: "event".to_string(),
                    payload,
                    timestamp,
                });
            }
        }
        None
    }

    /// Whether the stream connection has terminated.
    pub fn is_closed(&self) -> bool {
        self.is_closed.load(Ordering::SeqCst)
    }

    fn build_ws_url(base_url: &str) -> Result<Url, OpenWAError> {
        let mut parsed = Url::parse(base_url)
            .map_err(|e| OpenWAError::Config(format!("Invalid base URL: {}", e)))?;

        let scheme = match parsed.scheme() {
            "https" | "wss" => "wss",
            "http" | "ws" => "ws",
            other => {
                return Err(OpenWAError::Config(format!(
                    "Unsupported URL scheme: {}",
                    other
                )))
            }
        };

        parsed
            .set_scheme(scheme)
            .map_err(|_| OpenWAError::Config("Failed to set WebSocket scheme".to_string()))?;

        parsed.set_path("/socket.io/");
        parsed.set_query(Some("EIO=4&transport=websocket"));

        Ok(parsed)
    }
}
