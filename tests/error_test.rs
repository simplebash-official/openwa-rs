use openwa::error::OpenWAError;
use reqwest::StatusCode;

#[test]
fn test_classify_401_unauthorized() {
    let raw =
        br#"{"statusCode":401,"message":"Invalid or missing API key","error":"Unauthorized"}"#;
    let err = OpenWAError::from_response(StatusCode::UNAUTHORIZED, raw, "GET /api/sessions", None);

    match err {
        OpenWAError::Auth(api_err) => {
            assert_eq!(api_err.status, 401);
            assert_eq!(api_err.message, "Invalid or missing API key");
            assert_eq!(api_err.kind.as_deref(), Some("Unauthorized"));
        }
        other => panic!("Expected Auth error, got {:?}", other),
    }
}

#[test]
fn test_classify_403_forbidden() {
    let raw = br#"{"statusCode":403,"message":"Requires OPERATOR role","error":"Forbidden"}"#;
    let err = OpenWAError::from_response(StatusCode::FORBIDDEN, raw, "POST /api/sessions", None);

    match err {
        OpenWAError::Forbidden(api_err) => {
            assert_eq!(api_err.status, 403);
            assert_eq!(api_err.message, "Requires OPERATOR role");
        }
        other => panic!("Expected Forbidden error, got {:?}", other),
    }
}

#[test]
fn test_classify_404_not_found() {
    let raw = br#"{"statusCode":404,"message":"Session not found","error":"Not Found"}"#;
    let err = OpenWAError::from_response(StatusCode::NOT_FOUND, raw, "GET /api/sessions/foo", None);

    match err {
        OpenWAError::NotFound(api_err) => {
            assert_eq!(api_err.status, 404);
            assert_eq!(api_err.message, "Session not found");
        }
        other => panic!("Expected NotFound error, got {:?}", other),
    }
}

#[test]
fn test_classify_429_rate_limited_general() {
    let raw = br#"{"statusCode":429,"message":"Too Many Requests"}"#;
    let err = OpenWAError::from_response(
        StatusCode::TOO_MANY_REQUESTS,
        raw,
        "GET /api/search",
        Some("60"),
    );

    match err {
        OpenWAError::RateLimited(rl_err) => {
            assert_eq!(rl_err.status, 429);
            assert!(!rl_err.is_send_pacing);
            assert_eq!(rl_err.retry_after_seconds, Some(60));
        }
        other => panic!("Expected RateLimited error, got {:?}", other),
    }
}

#[test]
fn test_classify_429_send_pacing_limited() {
    let raw = br#"{
        "statusCode": 429,
        "message": "Recipient has not responded to an outbound message in the last 24h",
        "code": "SEND_PACING_LIMITED",
        "retryAfterSeconds": 300
    }"#;
    let err = OpenWAError::from_response(
        StatusCode::TOO_MANY_REQUESTS,
        raw,
        "POST /api/sessions/1/messages/send-text",
        None,
    );

    match err {
        OpenWAError::RateLimited(rl_err) => {
            assert!(rl_err.is_send_pacing);
            assert_eq!(rl_err.retry_after_seconds, Some(300));
        }
        other => panic!("Expected RateLimited error, got {:?}", other),
    }
}

#[test]
fn test_classify_503_service_unavailable() {
    let raw = br#"{"statusCode":503,"message":"Session engine reconnecting"}"#;
    let err = OpenWAError::from_response(
        StatusCode::SERVICE_UNAVAILABLE,
        raw,
        "POST /api/sessions/1/messages",
        None,
    );

    match err {
        OpenWAError::ServiceUnavailable(api_err) => {
            assert_eq!(api_err.status, 503);
            assert_eq!(api_err.message, "Session engine reconnecting");
        }
        other => panic!("Expected ServiceUnavailable error, got {:?}", other),
    }
}
