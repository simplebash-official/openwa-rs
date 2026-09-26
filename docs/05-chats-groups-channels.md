# Chapter 5: Chats, Groups & Channels

This guide details managing customer conversations, group administration, and WhatsApp Newsletters (Channels).

---

## 1. Chat Operations

Access chat methods via `client.chats()`.

### Listing Chats with Paging

```rust
use openwa::types::chat::ListChatsQuery;

let chats = client.chats().list("default", Some(ListChatsQuery {
    limit: Some(25),
    offset: Some(0),
    archived: None,
    unread_only: None,
})).await?;

for chat in chats {
    println!("Chat: {} (Unread: {})", chat.name.unwrap_or(chat.id), chat.unread_count);
}
```

### Unread Counters & Mark as Read / Unread

```rust
use openwa::types::chat::{MarkChatReadRequest, MarkChatUnreadRequest};

let chat_id = "1234567890@c.us";

// Mark as read
client.chats().mark_read("default", chat_id, MarkChatReadRequest { mark_read: true }).await?;

// Mark as unread
client.chats().mark_unread("default", chat_id, MarkChatUnreadRequest { unread: true }).await?;
```

### Pinning, Archiving & Muting Chats

```rust
use openwa::types::chat::{ArchiveChatRequest, MuteChatRequest, PinChatRequest};
use chrono::{Utc, Duration};

let chat_id = "1234567890@c.us";

// Pin / Unpin
client.chats().pin("default", chat_id, PinChatRequest { pin: true }).await?;

// Archive / Unarchive
client.chats().archive("default", chat_id, ArchiveChatRequest { archive: true }).await?;

// Mute for 8 hours
let mute_until = (Utc::now() + Duration::hours(8)).timestamp();
client.chats().mute("default", chat_id, MuteChatRequest {
    mute: true,
    mute_until: Some(mute_until),
}).await?;
```

### Sending Chat Presence Indicators (Typing / Recording)

```rust
use openwa::types::chat::ChatPresenceRequest;

// Send typing indicator
client.chats().send_presence("default", "1234567890@c.us", ChatPresenceRequest::typing()).await?;

// Send recording audio indicator
client.chats().send_presence("default", "1234567890@c.us", ChatPresenceRequest::recording()).await?;

// Clear presence
client.chats().send_presence("default", "1234567890@c.us", ChatPresenceRequest::paused()).await?;
```

---

## 2. Group Administration

Access group features via `client.groups()`.

### Creating a Group

```rust
use openwa::types::group::CreateGroupRequest;

let participants = vec!["1234567890@c.us".to_string(), "9876543210@c.us".to_string()];
let req = CreateGroupRequest::new("Product Launch Team", participants);

let group = client.groups().create("default", req).await?;
println!("Group created! ID: {}", group.id);
```

### Managing Participants (Add, Remove, Promote, Demote)

```rust
use openwa::types::group::ParticipantsRequest;

let members = vec!["5551234567@c.us".to_string()];

// Add members
client.groups().add_participants("default", &group.id, ParticipantsRequest::new(members.clone())).await?;

// Promote to admin
client.groups().promote_participants("default", &group.id, ParticipantsRequest::new(members.clone())).await?;

// Demote admin to member
client.groups().demote_participants("default", &group.id, ParticipantsRequest::new(members.clone())).await?;

// Remove members
client.groups().remove_participants("default", &group.id, ParticipantsRequest::new(members)).await?;
```

### Group Settings & Ephemeral Disappearing Messages

Configure who can send messages and set disappearing message timers:

```rust
use openwa::types::group::UpdateGroupSettingsRequest;

let settings = UpdateGroupSettingsRequest {
    announce: Some(true),       // Only admins can send messages
    locked: Some(true),         // Only admins can edit group subject/description
    ephemeral_seconds: Some(86400), // Messages disappear after 24 hours
    member_add_mode: Some("admin_add".into()), // Only admins can add members
};

client.groups().update_settings("default", &group.id, settings).await?;
```

### Invite Links & Join Codes

```rust
// Fetch existing invite code
let invite = client.groups().invite_code("default", &group.id).await?;
println!("Invite link: {}", invite.link.as_deref().unwrap_or(""));

// Revoke and generate new code
let new_invite = client.groups().revoke_invite_code("default", &group.id).await?;
println!("New code: {}", new_invite.code);
```

---

## 3. Channels (Newsletters)

Access WhatsApp Newsletters and Channels via `client.channels()`.

```rust
// Search public channels
let search_results = client.channels().search("default", "tech news", Some(10)).await?;
for channel in search_results {
    println!("Channel: {} (Subscribers: {:?})", channel.name, channel.subscribers_count);
}

// Follow / Subscribe to channel
client.channels().subscribe("default", "123456789@newsletter").await?;

// Unfollow / Unsubscribe
client.channels().unsubscribe("default", "123456789@newsletter").await?;

// Mute channel notifications
client.channels().mute("default", "123456789@newsletter", true).await?;
```
