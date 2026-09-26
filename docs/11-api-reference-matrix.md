# Chapter 11: API Reference Matrix

This matrix maps every OpenWA REST endpoint to its corresponding `openwa-rs` client method, required HTTP method, request DTO, and return type.

---

## 1. Sessions (`client.sessions()`)

| HTTP Method & Path | Rust SDK Method | Request DTO | Response Type | Role |
|:---|:---|:---|:---|:---|
| `GET /api/sessions` | `list(query)` | `Option<ListSessionsQuery>` | `Vec<SessionResponse>` | `VIEWER` |
| `POST /api/sessions` | `create(req)` | `CreateSessionRequest` | `SessionResponse` | `OPERATOR` |
| `GET /api/sessions/{id}` | `get(id)` | — | `SessionResponse` | `VIEWER` |
| `DELETE /api/sessions/{id}` | `delete(id)` | — | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/start` | `start(id)` | — | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/stop` | `stop(id)` | — | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/restart` | `restart(id)` | — | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/logout` | `logout(id)` | — | `SuccessResult` | `OPERATOR` |
| `GET /api/sessions/{id}/qr-code` | `get_qr_code(id)` | — | `QRCodeResponse` | `VIEWER` |
| `GET /api/sessions/{id}/pairing-code` | `get_pairing_code(id, phone)` | — | `PairingCodeResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/presence` | `set_presence(id, req)` | `SetOwnPresenceRequest` | `SuccessResult` | `OPERATOR` |
| `GET /api/sessions/{id}/config` | `get_config(id)` | — | `SessionConfigResponse` | `VIEWER` |
| `PATCH /api/sessions/{id}/config` | `update_config(id, req)` | `UpdateSessionConfigRequest` | `SessionConfigResponse` | `OPERATOR` |
| `GET /api/sessions/{id}/proxy` | `get_proxy(id)` | — | `ProxyConfigResponse` | `VIEWER` |
| `PUT /api/sessions/{id}/proxy` | `set_proxy(id, req)` | `SetProxyRequest` | `SuccessResult` | `OPERATOR` |
| `DELETE /api/sessions/{id}/proxy` | `delete_proxy(id)` | — | `SuccessResult` | `OPERATOR` |

---

## 2. Messages (`client.messages()`)

| HTTP Method & Path | Rust SDK Method | Request DTO | Response Type | Role |
|:---|:---|:---|:---|:---|
| `GET /api/sessions/{id}/messages` | `list(id, query)` | `Option<ListMessagesQuery>` | `Vec<MessageRecord>` | `VIEWER` |
| `POST /api/sessions/{id}/messages/send-text` | `send_text(id, req)` | `SendTextRequest` | `MessageResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/send-image` | `send_image(id, req)` | `SendMediaRequest` | `MessageResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/send-video` | `send_video(id, req)` | `SendMediaRequest` | `MessageResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/send-audio` | `send_audio(id, req)` | `SendAudioRequest` | `MessageResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/send-document` | `send_document(id, req)` | `SendMediaRequest` | `MessageResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/send-sticker` | `send_sticker(id, req)` | `SendMediaRequest` | `MessageResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/send-location` | `send_location(id, req)` | `SendLocationRequest` | `MessageResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/send-contact` | `send_contact(id, req)` | `SendContactRequest` | `MessageResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/send-template` | `send_template(id, req)` | `SendTemplateRequest` | `MessageResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/send-poll` | `send_poll(id, req)` | `SendPollRequest` | `MessageResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/reply` | `reply(id, req)` | `ReplyMessageRequest` | `MessageResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/forward` | `forward(id, req)` | `ForwardMessageRequest` | `MessageResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/click-button` | `click_button(id, req)` | `ClickButtonRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/reaction` | `react(id, req)` | `ReactMessageRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/edit` | `edit(id, req)` | `EditMessageRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/delete` | `delete(id, req)` | `DeleteMessageRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/pin` | `pin(id, req)` | `PinMessageRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/unpin` | `unpin(id, req)` | `UnpinMessageRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/star` | `star(id, req)` | `StarMessageRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/messages/vote-poll` | `vote_poll(id, req)` | `VotePollRequest` | `SuccessResult` | `OPERATOR` |
| `GET /api/sessions/{id}/messages/{chatId}/{msgId}/media` | `media_bytes(...)` | — | `Vec<u8>` | `VIEWER` |
| `GET /api/sessions/{id}/messages/{chatId}/{msgId}/media` | `download_media_to_file(...)` | — | `u64` (bytes written) | `VIEWER` |
| `POST /api/sessions/{id}/messages/send-bulk` | `send_bulk(id, req)` | `SendBulkRequest` | `BulkMessageResponse` | `OPERATOR` |
| `GET /api/sessions/{id}/messages/batch/{id}` | `batch_status(id, batchId)` | — | `BatchStatusResponse` | `VIEWER` |
| `POST /api/sessions/{id}/messages/batch/{id}/cancel` | `cancel_batch(id, batchId)` | — | `BatchStatusResponse` | `OPERATOR` |

---

## 3. Chats (`client.chats()`)

| HTTP Method & Path | Rust SDK Method | Request DTO | Response Type | Role |
|:---|:---|:---|:---|:---|
| `GET /api/sessions/{id}/chats` | `list(id, query)` | `Option<ListChatsQuery>` | `Vec<ChatRecord>` | `VIEWER` |
| `GET /api/sessions/{id}/chats/{chatId}` | `get(id, chatId)` | — | `ChatDetails` | `VIEWER` |
| `POST /api/sessions/{id}/chats/{chatId}/mark-read` | `mark_read(...)` | `MarkChatReadRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/chats/{chatId}/mark-unread` | `mark_unread(...)` | `MarkChatUnreadRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/chats/{chatId}/archive` | `archive(...)` | `ArchiveChatRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/chats/{chatId}/pin` | `pin(...)` | `PinChatRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/chats/{chatId}/mute` | `mute(...)` | `MuteChatRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/chats/{chatId}/delete` | `delete(...)` | `DeleteChatRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/chats/{chatId}/presence` | `send_presence(...)` | `ChatPresenceRequest` | `SuccessResult` | `OPERATOR` |

---

## 4. Groups (`client.groups()`)

| HTTP Method & Path | Rust SDK Method | Request DTO | Response Type | Role |
|:---|:---|:---|:---|:---|
| `GET /api/sessions/{id}/groups` | `list(id)` | — | `Vec<GroupSummary>` | `VIEWER` |
| `POST /api/sessions/{id}/groups` | `create(id, req)` | `CreateGroupRequest` | `GroupInfo` | `OPERATOR` |
| `GET /api/sessions/{id}/groups/{gid}` | `get(id, gid)` | — | `GroupDetails` | `VIEWER` |
| `POST /api/sessions/{id}/groups/join` | `join(id, req)` | `JoinGroupRequest` | `SuccessResult` | `OPERATOR` |
| `GET /api/sessions/{id}/groups/join-info` | `join_info(id, code)` | — | `GroupJoinInfo` | `VIEWER` |
| `POST /api/sessions/{id}/groups/{gid}/participants` | `add_participants(...)` | `ParticipantsRequest` | `ParticipantsResult` | `OPERATOR` |
| `DELETE /api/sessions/{id}/groups/{gid}/participants` | `remove_participants(...)` | `ParticipantsRequest` | `ParticipantsResult` | `OPERATOR` |
| `POST /api/sessions/{id}/groups/{gid}/admins/promote` | `promote_participants(...)` | `ParticipantsRequest` | `ParticipantsResult` | `OPERATOR` |
| `POST /api/sessions/{id}/groups/{gid}/admins/demote` | `demote_participants(...)` | `ParticipantsRequest` | `ParticipantsResult` | `OPERATOR` |
| `PUT /api/sessions/{id}/groups/{gid}/subject` | `set_subject(...)` | `SetSubjectRequest` | `SuccessResult` | `OPERATOR` |
| `PUT /api/sessions/{id}/groups/{gid}/description` | `set_description(...)` | `SetDescriptionRequest` | `SuccessResult` | `OPERATOR` |
| `POST /api/sessions/{id}/groups/{gid}/leave` | `leave(id, gid)` | — | `SuccessResult` | `OPERATOR` |
| `GET /api/sessions/{id}/groups/{gid}/invite-code` | `invite_code(id, gid)` | — | `InviteCodeResponse` | `OPERATOR` |
| `POST /api/sessions/{id}/groups/{gid}/invite-code/revoke` | `revoke_invite_code(...)` | — | `InviteCodeResponse` | `OPERATOR` |
| `GET /api/sessions/{id}/groups/{gid}/settings` | `get_settings(id, gid)` | — | `GroupSettings` | `VIEWER` |
| `PUT /api/sessions/{id}/groups/{gid}/settings` | `update_settings(...)` | `UpdateGroupSettingsRequest` | `SuccessResult` | `OPERATOR` |

---

## 5. Webhooks, Status, Channels, Contacts & Admin

| Resource Accessor | Method Range | Description |
|:---|:---|:---|
| `client.webhooks()` | `list`, `create`, `get`, `update`, `delete`, `rotate_secret`, `ping`, `failed` | Full webhook management, secret rotation, and dead letter outbox inspection. |
| `client.contacts()` | `list`, `get_profile`, `get_picture`, `check_number`, `get_card`, `sync` | Verification and profile access. |
| `client.status()` | `get_all`, `get_by_id`, `send_text`, `send_image`, `send_video`, `send_voice`, `delete` | WhatsApp Stories updates and media downloads. |
| `client.channels()` | `search`, `get`, `subscribe`, `unsubscribe`, `mute`, `update`, `transfer_owner`, `demote_admin` | WhatsApp Channels (Newsletters). |
| `client.labels()` | `list`, `create`, `update`, `delete`, `add_to_chat`, `remove_from_chat`, `get_chats` | WhatsApp Business customer labels. |
| `client.catalog()` | `list`, `get`, `create`, `update`, `delete`, `send_product` | Business catalog and collections. |
| `client.search()` | `search(query)` | Global search across indexed WhatsApp messages. |
| `client.profile()` | `get`, `set_name`, `set_about`, `upload_avatar`, `delete_avatar` | Session account profile management. |
| `client.calls()` | `create_link`, `reject` | Call links generation and call rejection. |
| `client.media()` | `conversion_status`, `convert_voice`, `convert_video` | Audio and video ffmpeg transcoding. |
| `client.health()` | `liveness`, `readiness`, `status` | Gateway container health probes. |
| `client.admin()` | `auth_keys`, `infra`, `plugins`, `integrations`, `audit`, `metrics`, `stats`, `automation` | Privileged system administration. |
