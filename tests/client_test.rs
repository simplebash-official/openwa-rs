use openwa::{OpenWAClient, SendTextRequest};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_client_sends_api_key_header() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/health"))
        .and(header("x-api-key", "test-api-key-123"))
        .and(header("accept", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "ok",
            "version": "1.0.0"
        })))
        .mount(&mock_server)
        .await;

    let client = OpenWAClient::new(mock_server.uri(), "test-api-key-123").unwrap();
    let health = client.health().check().await.unwrap();

    assert_eq!(health.status, "ok");
    assert_eq!(health.version.as_deref(), Some("1.0.0"));
}

#[tokio::test]
async fn test_client_preserves_at_colon_plus_in_path() {
    let mock_server = MockServer::start().await;

    // Contact ID with '@' and '+' should NOT have '@' or '+' encoded as %40 or %2B
    Mock::given(method("GET"))
        .and(path("/api/sessions/sess-1/contacts/+1234567890@c.us"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "+1234567890@c.us",
            "name": "Alice",
            "number": "1234567890"
        })))
        .mount(&mock_server)
        .await;

    let client = OpenWAClient::new(mock_server.uri(), "key").unwrap();
    let contact = client
        .contacts()
        .get("sess-1", "+1234567890@c.us")
        .await
        .unwrap();

    assert_eq!(contact.id, "+1234567890@c.us");
    assert_eq!(contact.name.as_deref(), Some("Alice"));
}

#[tokio::test]
async fn test_client_escapes_forward_slash_in_path() {
    let mock_server = MockServer::start().await;

    // Forward slash in ID must be encoded as %2F to prevent path traversal
    Mock::given(method("GET"))
        .and(path("/api/sessions/sess-1/contacts/user%2Fslash"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "user/slash",
            "number": "123"
        })))
        .mount(&mock_server)
        .await;

    let client = OpenWAClient::new(mock_server.uri(), "key").unwrap();
    let contact = client.contacts().get("sess-1", "user/slash").await.unwrap();

    assert_eq!(contact.id, "user/slash");
}

#[tokio::test]
async fn test_client_rejects_redirects() {
    let mock_server = MockServer::start().await;

    // A 301/302 redirect MUST NOT be followed to prevent credential leakage
    Mock::given(method("GET"))
        .and(path("/api/health"))
        .respond_with(
            ResponseTemplate::new(302).insert_header("Location", "http://attacker.example.com/"),
        )
        .mount(&mock_server)
        .await;

    let client = OpenWAClient::new(mock_server.uri(), "secret-key").unwrap();
    let result = client.health().check().await;

    // With redirect policy set to none, reqwest does not follow the redirect and returns the 302 as an Api error
    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        openwa::OpenWAError::Api(api_err) => {
            assert_eq!(api_err.status, 302);
        }
        other => panic!("Expected Api 302 error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_send_message_post_json() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/sessions/s1/messages/send-text"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "messageId": "msg-abc-123",
            "timestamp": 1758888999
        })))
        .mount(&mock_server)
        .await;

    let client = OpenWAClient::new(mock_server.uri(), "key").unwrap();
    let res = client
        .messages()
        .send_text("s1", SendTextRequest::new("12345@c.us", "Hello"))
        .await
        .unwrap();

    assert_eq!(res.message_id, "msg-abc-123");
    assert_eq!(res.timestamp, 1758888999);
}

#[tokio::test]
async fn test_admin_namespace_auth_keys() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/auth/api-keys"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": "key-1",
                "name": "production",
                "role": "admin",
                "keyPrefix": "opw_live_1234",
                "revoked": false,
                "createdAt": "2026-09-26T12:00:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = OpenWAClient::new(mock_server.uri(), "master-key").unwrap();
    let keys = client.admin().auth_keys().list().await.unwrap();

    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].id, "key-1");
    assert_eq!(keys[0].role, openwa::types::ApiKeyRole::Admin);
}
