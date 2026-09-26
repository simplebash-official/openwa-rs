use openwa::types::*;
use serde_json::json;

#[test]
fn test_create_group_request_serialization() {
    let req = CreateGroupRequest::new("Test Group", vec!["123@c.us".into(), "456@c.us".into()]);
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["name"], "Test Group");
    assert_eq!(v["participants"], json!(["123@c.us", "456@c.us"]));
    assert!(v.get("subject").is_none());
}

#[test]
fn test_join_group_request_serialization() {
    let req = JoinGroupRequest::new("abc123code");
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["inviteCode"], "abc123code");
    assert!(v.get("code").is_none());
}

#[test]
fn test_group_settings_serialization() {
    let req = UpdateGroupSettingsRequest {
        announce: Some(true),
        locked: Some(false),
        ephemeral_seconds: Some(86400),
        member_add_mode: Some("admins".into()),
    };
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["announce"], true);
    assert_eq!(v["locked"], false);
    assert_eq!(v["ephemeralSeconds"], 86400);
    assert_eq!(v["memberAddMode"], "admins");
    assert!(v.get("ephemeralDuration").is_none());
}

#[test]
fn test_mute_chat_request_serialization() {
    let req = MuteChatRequest::new("123@c.us", Some(1700000000000));
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["chatId"], "123@c.us");
    assert_eq!(v["muteUntil"], 1700000000000i64);
    assert!(v.get("muteExpiration").is_none());
}

#[test]
fn test_chat_action_requests_serialization() {
    let unread = MarkChatUnreadRequest {
        chat_id: "123@c.us".into(),
    };
    let v = serde_json::to_value(&unread).unwrap();
    assert_eq!(v, json!({ "chatId": "123@c.us" }));

    let archive = ArchiveChatRequest {
        chat_id: "123@c.us".into(),
        archive: Some(true),
    };
    let v = serde_json::to_value(&archive).unwrap();
    assert_eq!(v, json!({ "chatId": "123@c.us", "archive": true }));

    let pin = PinChatRequest {
        chat_id: "123@c.us".into(),
        pin: Some(false),
    };
    let v = serde_json::to_value(&pin).unwrap();
    assert_eq!(v, json!({ "chatId": "123@c.us", "pin": false }));

    let delete = DeleteChatRequest {
        chat_id: "123@c.us".into(),
    };
    let v = serde_json::to_value(&delete).unwrap();
    assert_eq!(v, json!({ "chatId": "123@c.us" }));
}

#[test]
fn test_webhook_requests_serialization() {
    let mut headers = std::collections::HashMap::new();
    headers.insert("X-Test".into(), "Val".into());
    let req = CreateWebhookRequest {
        url: "https://example.com/hook".into(),
        events: Some(vec!["message.received".into()]),
        secret: Some("a-very-long-webhook-secret-16chars".into()),
        headers: Some(headers.clone()),
        filters: None,
        retry_count: Some(3),
    };
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["url"], "https://example.com/hook");
    assert_eq!(v["events"], json!(["message.received"]));
    assert_eq!(v["headers"]["X-Test"], "Val");
    assert!(v.get("customHeaders").is_none());
    assert!(v.get("active").is_none());

    let upd = UpdateWebhookRequest {
        url: None,
        events: None,
        secret: None,
        headers: Some(headers),
        filters: None,
        retry_count: None,
        active: Some(true),
    };
    let v = serde_json::to_value(&upd).unwrap();
    assert_eq!(v["active"], true);
    assert_eq!(v["headers"]["X-Test"], "Val");
    assert!(v.get("customHeaders").is_none());
}

#[test]
fn test_automation_rule_requests_serialization() {
    let req = CreateAutomationRuleRequest::new("Auto Reply", "Hello there!");
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["name"], "Auto Reply");
    assert_eq!(v["replyText"], "Hello there!");
    assert_eq!(v["enabled"], true);
    assert!(v.get("reply").is_none());
    assert!(v.get("pattern").is_none());
    assert!(v.get("matchType").is_none());
    assert!(v.get("active").is_none());
}

#[test]
fn test_message_requests_serialization() {
    // SendContactRequest
    let contact = SendContactRequest::new("123@c.us", "Alice Smith", "+1234567890");
    let v = serde_json::to_value(&contact).unwrap();
    assert_eq!(v["chatId"], "123@c.us");
    assert_eq!(v["contactName"], "Alice Smith");
    assert_eq!(v["contactNumber"], "+1234567890");
    assert!(v.get("name").is_none());
    assert!(v.get("contactId").is_none());

    // SendTemplateRequest
    let template = SendTemplateRequest {
        chat_id: "123@c.us".into(),
        template_id: Some("tpl-1".into()),
        template_name: None,
        vars: Some(json!({"name": "Bob"})),
        mentions: None,
        link_preview: None,
    };
    let v = serde_json::to_value(&template).unwrap();
    assert_eq!(v["chatId"], "123@c.us");
    assert_eq!(v["templateId"], "tpl-1");
    assert_eq!(v["vars"]["name"], "Bob");
    assert!(v.get("variables").is_none());

    // SendPollRequest
    let poll = SendPollRequest {
        chat_id: "123@c.us".into(),
        name: "Lunch?".into(),
        options: vec!["Pizza".into(), "Sushi".into()],
        allow_multiple_answers: Some(true),
        quoted_message_id: None,
    };
    let v = serde_json::to_value(&poll).unwrap();
    assert_eq!(v["allowMultipleAnswers"], true);
    assert!(v.get("multipleAnswers").is_none());

    // ForwardMessageRequest
    let fwd = ForwardMessageRequest {
        from_chat_id: "111@c.us".into(),
        to_chat_id: "222@c.us".into(),
        message_id: "msg-123".into(),
    };
    let v = serde_json::to_value(&fwd).unwrap();
    assert_eq!(v["fromChatId"], "111@c.us");
    assert_eq!(v["toChatId"], "222@c.us");
    assert_eq!(v["messageId"], "msg-123");

    // ReactMessageRequest
    let react = ReactMessageRequest {
        chat_id: "123@c.us".into(),
        message_id: "msg-123".into(),
        emoji: "👍".into(),
    };
    let v = serde_json::to_value(&react).unwrap();
    assert_eq!(v["emoji"], "👍");
    assert!(v.get("reaction").is_none());

    // DeleteMessageRequest
    let del = DeleteMessageRequest {
        chat_id: "123@c.us".into(),
        message_id: "msg-123".into(),
        for_everyone: Some(true),
    };
    let v = serde_json::to_value(&del).unwrap();
    assert_eq!(v["forEveryone"], true);
    assert!(v.get("everyone").is_none());

    // VotePollRequest
    let vote = VotePollRequest {
        chat_id: "123@c.us".into(),
        poll_message_id: "poll-msg-1".into(),
        options: vec!["Pizza".into()],
    };
    let v = serde_json::to_value(&vote).unwrap();
    assert_eq!(v["pollMessageId"], "poll-msg-1");
    assert!(v.get("messageId").is_none());

    // EditMessageRequest
    let edit = EditMessageRequest {
        chat_id: "123@c.us".into(),
        message_id: "msg-1".into(),
        body: "Updated text".into(),
        mentions: None,
    };
    let v = serde_json::to_value(&edit).unwrap();
    assert_eq!(v["body"], "Updated text");
    assert!(v.get("text").is_none());

    // Bulk messages
    let bulk = SendBulkRequest::new(vec![BulkMessageItem::text("123@c.us", "Hi!")]);
    let v = serde_json::to_value(&bulk).unwrap();
    assert_eq!(v["messages"][0]["chatId"], "123@c.us");
    assert_eq!(v["messages"][0]["type"], "text");
    assert_eq!(v["messages"][0]["content"]["text"], "Hi!");
}

#[test]
fn test_status_requests_serialization() {
    let img_req = SendImageStatusRequest {
        image: StatusMediaInput::from_url("https://example.com/pic.jpg"),
        caption: Some("Look!".into()),
        recipients: None,
    };
    let v = serde_json::to_value(&img_req).unwrap();
    assert_eq!(v["image"]["url"], "https://example.com/pic.jpg");
    assert_eq!(v["caption"], "Look!");

    let voice_req = SendVoiceStatusRequest {
        audio: StatusMediaInput::from_url("https://example.com/voice.ogg"),
        background_color: Some("#25D366".into()),
        recipients: None,
    };
    let v = serde_json::to_value(&voice_req).unwrap();
    assert_eq!(v["audio"]["url"], "https://example.com/voice.ogg");
    assert_eq!(v["backgroundColor"], "#25D366");
}

#[test]
fn test_media_convert_request_does_not_serialize_mimetype() {
    let mut req = MediaConvertRequest::from_url("https://example.com/input.wav");
    req.mimetype = Some("audio/wav".into());
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["url"], "https://example.com/input.wav");
    assert!(
        v.get("mimetype").is_none(),
        "ConvertMediaDto rejects mimetype with 400"
    );
}

#[test]
fn test_send_product_request_does_not_serialize_footer() {
    let mut req = SendProductRequest::new("123@c.us", "prod-1");
    req.footer = Some("Legacy footer".into());
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["chatId"], "123@c.us");
    assert_eq!(v["productId"], "prod-1");
    assert!(
        v.get("footer").is_none(),
        "SendProductDto rejects footer with 400"
    );
}

#[test]
fn test_integration_instance_requests_serialization() {
    let create = CreateIntegrationInstanceRequest::new("test-inst-1");
    let v = serde_json::to_value(&create).unwrap();
    assert_eq!(v["instanceId"], "test-inst-1");
    assert!(v.get("name").is_none());
    assert!(v.get("active").is_none());

    let upd = UpdateIntegrationInstanceRequest {
        enabled: Some(true),
        session_scope: Some("sess-1".into()),
        config: Some(json!({"key": "val"})),
    };
    let v = serde_json::to_value(&upd).unwrap();
    assert_eq!(v["enabled"], true);
    assert_eq!(v["sessionScope"], "sess-1");
    assert_eq!(v["config"]["key"], "val");
    assert!(v.get("active").is_none());
}

#[test]
fn test_presence_requests_serialization() {
    let own = SetOwnPresenceRequest::available();
    let v = serde_json::to_value(&own).unwrap();
    assert_eq!(v["available"], true);
    assert!(v.get("presence").is_none());

    let legacy = SetOnlinePresenceRequest {
        available: true,
        presence: Some("available".into()),
    };
    let v = serde_json::to_value(&legacy).unwrap();
    assert_eq!(v["available"], true);
    assert!(
        v.get("presence").is_none(),
        "SetOwnPresenceDto rejects presence with 400"
    );
}
