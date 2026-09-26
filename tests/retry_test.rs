use openwa::retry::RetryPolicy;
use openwa::OpenWAClient;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_retry_idempotent_get_on_503() {
    let mock_server = MockServer::start().await;

    // First request returns 503, second returns 200
    Mock::given(method("GET"))
        .and(path("/api/health"))
        .respond_with(ResponseTemplate::new(503).set_body_json(serde_json::json!({
            "message": "Service Temporarily Unavailable"
        })))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "ok"
        })))
        .mount(&mock_server)
        .await;

    let retry_policy = RetryPolicy {
        max_retries: 3,
        base_delay: Duration::from_millis(10),
        max_delay: Duration::from_millis(50),
        retryable_statuses: vec![503],
        respect_retry_after: true,
    };

    let client = OpenWAClient::builder()
        .base_url(mock_server.uri())
        .api_key("key")
        .retry_policy(Some(retry_policy))
        .build()
        .unwrap();

    let health = client.health().check().await.unwrap();
    assert_eq!(health.status, "ok");
}

#[tokio::test]
async fn test_do_not_retry_non_idempotent_post_on_send_pacing() {
    let mock_server = MockServer::start().await;

    // Server returns 429 SEND_PACING_LIMITED
    Mock::given(method("POST"))
        .and(path("/api/sessions/s1/messages/send-text"))
        .respond_with(ResponseTemplate::new(429).set_body_json(serde_json::json!({
            "code": "SEND_PACING_LIMITED",
            "message": "Pacing limit exceeded",
            "retryAfterSeconds": 120
        })))
        .expect(1) // Should only be called once, NEVER retried
        .mount(&mock_server)
        .await;

    let retry_policy = RetryPolicy {
        max_retries: 3,
        base_delay: Duration::from_millis(10),
        max_delay: Duration::from_millis(50),
        retryable_statuses: vec![429, 503],
        respect_retry_after: true,
    };

    let client = OpenWAClient::builder()
        .base_url(mock_server.uri())
        .api_key("key")
        .retry_policy(Some(retry_policy))
        .build()
        .unwrap();

    let res = client
        .messages()
        .send_text("s1", openwa::SendTextRequest::new("123@c.us", "Hi"))
        .await;

    assert!(res.is_err());
    match res.unwrap_err() {
        openwa::OpenWAError::RateLimited(rl) => {
            assert!(rl.is_send_pacing);
            assert_eq!(rl.retry_after_seconds, Some(120));
        }
        other => panic!("Expected RateLimited, got: {:?}", other),
    }
}
