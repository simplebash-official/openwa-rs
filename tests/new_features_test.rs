use openwa::pagination::{fetch_all_pages, Paginator};
use openwa::types::builders::{BulkMessageBuilder, PollBuilder, TextRequestBuilder};
use openwa::types::media::guess_mime_type;
use openwa::types::message::{SendAudioRequest, SendMediaRequest};
use openwa::types::status::StatusMediaInput;
use std::path::Path;

#[test]
fn test_poll_builder() {
    let req = PollBuilder::new("1234567890@c.us", "What language do you write?")
        .option("Rust")
        .option("TypeScript")
        .options(vec!["Python", "Go"])
        .allow_multiple_answers(false)
        .quoted_message_id("msg_001")
        .build();

    assert_eq!(req.chat_id, "1234567890@c.us");
    assert_eq!(req.name, "What language do you write?");
    assert_eq!(req.options, vec!["Rust", "TypeScript", "Python", "Go"]);
    assert_eq!(req.allow_multiple_answers, Some(false));
    assert_eq!(req.quoted_message_id, Some("msg_001".into()));

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["chatId"], "1234567890@c.us");
    assert_eq!(json["name"], "What language do you write?");
    assert_eq!(json["options"].as_array().unwrap().len(), 4);
    assert_eq!(json["allowMultipleAnswers"], false);
    assert_eq!(json["quotedMessageId"], "msg_001");
}

#[test]
fn test_text_builder() {
    let req = TextRequestBuilder::new("123@c.us", "Hello @1234 and @5678")
        .mention("1234@c.us")
        .mentions(vec!["5678@c.us"])
        .build();

    assert_eq!(req.chat_id, "123@c.us");
    assert_eq!(req.text, "Hello @1234 and @5678");
    assert_eq!(req.mentions.as_ref().unwrap().len(), 2);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["chatId"], "123@c.us");
    assert_eq!(json["text"], "Hello @1234 and @5678");
    assert_eq!(json["mentions"].as_array().unwrap().len(), 2);
}

#[test]
fn test_bulk_builder() {
    let req = BulkMessageBuilder::new()
        .batch_id("batch-001")
        .delay_between_messages(500)
        .randomize_delay(true)
        .stop_on_error(true)
        .add_text("111@c.us", "Hello user 1")
        .add_image(
            "222@c.us",
            "https://example.com/pic.png",
            Some("Photo".into()),
        )
        .build();

    assert_eq!(req.batch_id, Some("batch-001".into()));
    assert_eq!(req.messages.len(), 2);

    let opts = req.options.as_ref().unwrap();
    assert_eq!(opts.delay_between_messages, Some(500));
    assert_eq!(opts.randomize_delay, Some(true));
    assert_eq!(opts.stop_on_error, Some(true));

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["batchId"], "batch-001");
    assert_eq!(json["messages"].as_array().unwrap().len(), 2);
    assert_eq!(json["options"]["delayBetweenMessages"], 500);
}

#[test]
fn test_guess_mime_type() {
    assert_eq!(guess_mime_type(Path::new("image.png")), "image/png");
    assert_eq!(guess_mime_type(Path::new("photo.jpg")), "image/jpeg");
    assert_eq!(guess_mime_type(Path::new("clip.mp4")), "video/mp4");
    assert_eq!(
        guess_mime_type(Path::new("voice.ogg")),
        "audio/ogg; codecs=opus"
    );
    assert_eq!(guess_mime_type(Path::new("doc.pdf")), "application/pdf");
    assert_eq!(guess_mime_type(Path::new("data.csv")), "text/csv");
    assert_eq!(
        guess_mime_type(Path::new("unknown.xyz123")),
        "application/octet-stream"
    );
}

#[tokio::test]
async fn test_media_from_file() {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_openwa_upload.png");
    tokio::fs::write(&file_path, b"fake-png-binary-content")
        .await
        .unwrap();

    let media_req = SendMediaRequest::from_file("123@c.us", &file_path)
        .await
        .unwrap();
    assert_eq!(media_req.chat_id, "123@c.us");
    assert_eq!(media_req.mimetype, Some("image/png".into()));
    assert_eq!(media_req.filename, Some("test_openwa_upload.png".into()));
    assert!(media_req.base64.is_some());

    let audio_req = SendAudioRequest::from_file("123@c.us", &file_path, true)
        .await
        .unwrap();
    assert_eq!(audio_req.chat_id, "123@c.us");
    assert_eq!(audio_req.ptt, Some(true));
    assert!(audio_req.base64.is_some());

    let status_req = StatusMediaInput::from_file(&file_path).await.unwrap();
    assert_eq!(status_req.mimetype, Some("image/png".into()));
    assert!(status_req.base64.is_some());

    let _ = tokio::fs::remove_file(&file_path).await;
}

#[tokio::test]
async fn test_fetch_all_pages_pagination() {
    // Mock paginated endpoint returning 10 items per page up to 25 total
    let total_items: Vec<String> = (0..25).map(|i| format!("item_{}", i)).collect();

    let collected = fetch_all_pages(10, None, |offset, limit| {
        let items = total_items.clone();
        async move {
            let start = offset as usize;
            if start >= items.len() {
                return Ok(vec![]);
            }
            let end = (start + limit as usize).min(items.len());
            Ok(items[start..end].to_vec())
        }
    })
    .await
    .unwrap();

    assert_eq!(collected.len(), 25);
    assert_eq!(collected[0], "item_0");
    assert_eq!(collected[24], "item_24");
}

#[tokio::test]
async fn test_paginator_lazy_stream() {
    let total_items: Vec<i32> = (1..=7).collect();

    let mut paginator = Paginator::new(3, |offset, limit| {
        let items = total_items.clone();
        async move {
            let start = offset as usize;
            if start >= items.len() {
                return Ok(Vec::<i32>::new());
            }
            let end = (start + limit as usize).min(items.len());
            Ok(items[start..end].to_vec())
        }
    });

    let mut results = Vec::new();
    while let Some(item) = paginator.next_item().await.unwrap() {
        results.push(item);
    }

    assert_eq!(results, vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(paginator.next_item().await.unwrap(), None);
}

#[cfg(feature = "axum")]
#[tokio::test]
async fn test_axum_webhook_extractor() {
    use axum::{
        body::Body,
        http::{header, Request, StatusCode},
        routing::post,
        Extension, Router,
    };
    use openwa::webhook::{OpenWAWebhook, WebhookSecret};
    use tower::ServiceExt;

    let secret = "test_webhook_secret_key";

    let app = Router::new()
        .route(
            "/webhook",
            post(
                |OpenWAWebhook(delivery): OpenWAWebhook<serde_json::Value>| async move {
                    assert_eq!(delivery.event, "message.received");
                    assert_eq!(delivery.session_id, "session-123");
                    StatusCode::OK
                },
            ),
        )
        .layer(Extension(WebhookSecret::new(secret)));

    let body = serde_json::json!({
        "event": "message.received",
        "timestamp": "2026-09-26T12:00:00Z",
        "sessionId": "session-123",
        "data": { "body": "Hello World" }
    });
    let raw_bytes = serde_json::to_vec(&body).unwrap();

    // 1. Valid signature test
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(&raw_bytes);
    let sig_hex = hex::encode(mac.finalize().into_bytes());

    let req = Request::builder()
        .uri("/webhook")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .header("x-openwa-signature", format!("sha256={}", sig_hex))
        .body(Body::from(raw_bytes.clone()))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // 2. Invalid signature test
    let req_invalid = Request::builder()
        .uri("/webhook")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .header("x-openwa-signature", "sha256=badsignature1234567890")
        .body(Body::from(raw_bytes))
        .unwrap();

    let resp_invalid = app.oneshot(req_invalid).await.unwrap();
    assert_eq!(resp_invalid.status(), StatusCode::UNAUTHORIZED);
}
