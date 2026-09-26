# Chapter 4: Messages & Media

`openwa-rs` provides comprehensive coverage for all WhatsApp messaging formats: plain text, media attachments, voice notes, interactive buttons, lists, polls, reactions, and bulk batches.

---

## 1. Text Messages

### Basic Text

```rust
use openwa::SendTextRequest;

let req = SendTextRequest::new("1234567890@c.us", "Hello world!");
let res = client.messages().send_text("default", req).await?;
println!("Message ID: {}", res.message_id);
```

### Text with Formatting and User Mentions

WhatsApp supports Markdown styling: `*bold*`, `_italic_`, `~strikethrough~`, and ````monospace````.

```rust
use openwa::types::builders::TextRequestBuilder;

let req = TextRequestBuilder::new(
    "123-456@g.us",
    "Welcome @1234567890 to the group! Please check the *rules*.",
)
.mention("1234567890@c.us")
.build();

client.messages().send_text("default", req).await?;
```

---

## 2. Media Messages (Images, Videos, Documents)

You can send media from a public URL or by reading directly from a local file.

### Sending Media from a Public URL

```rust
use openwa::types::message::SendMediaRequest;

let req = SendMediaRequest::from_url("1234567890@c.us", "https://example.com/banner.png")
    .with_caption("Check out our new release! 🚀")
    .with_filename("banner.png");

client.messages().send_image("default", req).await?;
```

### Sending Media from a Local File

The SDK provides asynchronous local file helpers that automatically read bytes, encode to Base64, and detect the MIME type:

```rust
use openwa::types::message::SendMediaRequest;

// Image
let img_req = SendMediaRequest::from_file("1234567890@c.us", "./assets/invoice.png").await?
    .with_caption("Here is your receipt.");
client.messages().send_image("default", img_req).await?;

// PDF Document
let doc_req = SendMediaRequest::from_file("1234567890@c.us", "./documents/report.pdf").await?
    .with_caption("Monthly Financial Report");
client.messages().send_document("default", doc_req).await?;
```

---

## 3. Audio & Voice Notes (PTT)

WhatsApp distinguishes between standard audio files (shown as an audio player) and Push-to-Talk (PTT) voice notes (shown with a microphone icon and waveform).

```rust
use openwa::types::message::SendAudioRequest;

// 1. From local audio file as PTT voice note
let voice_req = SendAudioRequest::from_file(
    "1234567890@c.us",
    "./recordings/greeting.ogg",
    true, // ptt = true
).await?;
client.messages().send_audio("default", voice_req).await?;

// 2. From URL
let audio_url_req = SendAudioRequest::voice_note("1234567890@c.us", "https://example.com/audio.mp3");
client.messages().send_audio("default", audio_url_req).await?;
```

---

## 4. Interactive Polls

Create native multi-choice WhatsApp polls with [`PollBuilder`](../src/types/builders.rs):

```rust
use openwa::types::builders::PollBuilder;
use openwa::types::message::VotePollRequest;

// Send poll
let poll = PollBuilder::new("1234567890@c.us", "What is your preferred database?")
    .option("PostgreSQL 🐘")
    .option("SQLite 🪶")
    .option("MongoDB 🍃")
    .allow_multiple_answers(false)
    .build();

let res = client.messages().send_poll("default", poll).await?;

// Vote on poll
let vote = VotePollRequest {
    chat_id: "1234567890@c.us".into(),
    poll_message_id: res.message_id,
    options: vec!["PostgreSQL 🐘".into()],
};
client.messages().vote_poll("default", vote).await?;
```

---

## 5. Message Reactions, Edits & Deletion

### Adding a Reaction (Emoji)

```rust
use openwa::types::message::ReactMessageRequest;

let req = ReactMessageRequest {
    chat_id: "1234567890@c.us".into(),
    message_id: "false_1234567890@c.us_3EB0123456".into(),
    emoji: "🔥".into(), // Set to "" to remove reaction
};
client.messages().react("default", req).await?;
```

### Editing a Sent Message

```rust
use openwa::types::message::EditMessageRequest;

let req = EditMessageRequest {
    chat_id: "1234567890@c.us".into(),
    message_id: "false_1234567890@c.us_3EB0123456".into(),
    body: "Corrected message text ✨".into(),
};
client.messages().edit("default", req).await?;
```

### Deleting a Message

```rust
use openwa::types::message::DeleteMessageRequest;

let req = DeleteMessageRequest {
    chat_id: "1234567890@c.us".into(),
    message_id: "false_1234567890@c.us_3EB0123456".into(),
    for_everyone: true, // Delete for all participants
};
client.messages().delete("default", req).await?;
```

---

## 6. Streaming & Downloading Media to Disk

Download inbound media directly to disk asynchronously without holding large files in RAM:

```rust
// Stream download directly to file
let bytes_written = client
    .messages()
    .download_media_to_file("default", "1234567890@c.us", "msg_video_001", "./downloads/video.mp4")
    .await?;

println!("Saved {} bytes to ./downloads/video.mp4", bytes_written);
```

---

## 7. Bulk Message Batches

Schedule automated bulk campaigns with automatic pacing and rate control using [`BulkMessageBuilder`](../src/types/builders.rs):

```rust
use openwa::types::builders::BulkMessageBuilder;

let batch = BulkMessageBuilder::new()
    .batch_id("customer-update-2026")
    .delay_between_messages(1200) // 1.2s delay between messages
    .randomize_delay(true)        // Jitter to prevent spam detection
    .stop_on_error(false)
    .add_text("1111111111@c.us", "Your order #101 has shipped!")
    .add_text("2222222222@c.us", "Your order #102 has shipped!")
    .add_image("3333333333@c.us", "https://example.com/receipt.png", Some("Your receipt".into()))
    .build();

let res = client.messages().send_bulk("default", batch).await?;
println!("Batch queued! Total messages: {}", res.total);

// Check batch execution status
let status = client.messages().batch_status("default", &res.batch_id).await?;
println!("Progress: {}/{} sent", status.sent, status.total);
```
