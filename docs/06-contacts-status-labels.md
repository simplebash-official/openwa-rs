# Chapter 6: Contacts, Status Stories & Labels

This chapter covers verifying WhatsApp numbers, reading and posting WhatsApp Stories (Status updates), and categorizing chats with WhatsApp Business Labels.

---

## 1. Contact Verification & Profiles

Access contact features via `client.contacts()`.

### Checking if a Phone Number is Registered on WhatsApp

Before sending messages to customer lists, verify registration to maintain high phone number reputation and avoid bans:

```rust
let check = client.contacts().check_number("default", "1234567890").await?;

if check.number_exists {
    println!("Number is registered! JID: {}", check.jid.unwrap_or_default());
} else {
    println!("Number is NOT registered on WhatsApp.");
}
```

### Fetching Contact Profiles & Avatars

```rust
// Fetch contact details (name, pushName, status message)
let profile = client.contacts().get_profile("default", "1234567890@c.us").await?;
println!("Name: {:?}, About: {:?}", profile.name, profile.about);

// Fetch high-resolution profile picture URL
let picture = client.contacts().get_picture("default", "1234567890@c.us").await?;
if let Some(url) = picture.url {
    println!("Avatar URL: {}", url);
}
```

---

## 2. WhatsApp Status (Stories)

WhatsApp Status updates (ephemeral 24-hour stories) can be text, photos, videos, or voice clips. Access status methods via `client.status()`.

### Reading Status Stories from Contacts

```rust
let statuses = client.status().get_all("default").await?;
for s in statuses {
    println!("Status from {} (Type: {}, Posted: {})", s.contact.id, s.status_type, s.posted_at);
}
```

### Posting a Text Status Update

```rust
use openwa::types::status::SendTextStatusRequest;

let req = SendTextStatusRequest {
    text: "Excited to launch openwa-rs 0.1! 🦀✨".into(),
    background_color: Some("#25D366".into()), // WhatsApp green
    font: Some(1),
};

let res = client.status().send_text("default", req).await?;
println!("Status posted! ID: {}", res.status_id);
```

### Posting an Image, Video, or Voice Status Update

Use [`StatusMediaInput`](../src/types/status.rs) with public URLs or local file paths:

```rust
use openwa::types::status::{SendImageStatusRequest, SendVoiceStatusRequest, StatusMediaInput};

// Post Image Story from local file
let img_input = StatusMediaInput::from_file("./assets/promo.jpg").await?;
let img_req = SendImageStatusRequest {
    image: img_input,
    caption: Some("Check out our weekend sale!".into()),
    recipients: None, // None = all contacts
};
client.status().send_image("default", img_req).await?;

// Post Voice Status Clip from local audio file
let voice_input = StatusMediaInput::from_file("./audio/shoutout.ogg").await?;
let voice_req = SendVoiceStatusRequest {
    audio: voice_input,
    background_color: Some("#128C7E".into()),
    recipients: None,
};
client.status().send_voice("default", voice_req).await?;
```

---

## 3. WhatsApp Business Labels

Labels help organize customer chats into visual, color-coded categories (e.g., "New Customer", "Pending Payment", "VIP"). Access label methods via `client.labels()`.

### Managing Labels

```rust
use openwa::types::label::CreateLabelRequest;

// List existing labels
let labels = client.labels().list("default").await?;
for label in &labels {
    println!("Label #{} '{}' (color: {:?})", label.id, label.name, label.color);
}

// Create a new label
let new_label = client.labels().create("default", CreateLabelRequest {
    name: "VIP Client".into(),
    color: Some(1), // Color palette index (1-20)
}).await?;
println!("Created label with ID: {}", new_label.id);
```

### Attaching and Detaching Labels to Customer Chats

```rust
let chat_id = "1234567890@c.us";
let label_id = "1";

// Add label to chat
client.labels().add_to_chat("default", label_id, chat_id).await?;

// Get all chats associated with this label
let tagged_chats = client.labels().get_chats("default", label_id).await?;
println!("VIP Chats: {:?}", tagged_chats);

// Remove label from chat
client.labels().remove_from_chat("default", label_id, chat_id).await?;
```
