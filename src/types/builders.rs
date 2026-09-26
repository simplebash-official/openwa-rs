use crate::types::message::{
    BulkMediaDto, BulkMessageContent, BulkMessageItem, BulkMessageOptions, SendBulkRequest,
    SendPollRequest, SendTextRequest,
};

/// Fluent builder for constructing [`SendPollRequest`] instances.
#[derive(Debug, Clone, Default)]
pub struct PollBuilder {
    chat_id: String,
    name: String,
    options: Vec<String>,
    allow_multiple_answers: Option<bool>,
    quoted_message_id: Option<String>,
}

impl PollBuilder {
    /// Create a new poll builder with a target chat and question name.
    pub fn new(chat_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            name: name.into(),
            options: Vec::new(),
            allow_multiple_answers: None,
            quoted_message_id: None,
        }
    }

    /// Add a single answer choice to the poll.
    pub fn option(mut self, opt: impl Into<String>) -> Self {
        self.options.push(opt.into());
        self
    }

    /// Add multiple answer choices to the poll.
    pub fn options<I, S>(mut self, opts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for opt in opts {
            self.options.push(opt.into());
        }
        self
    }

    /// Allow voters to select multiple choices.
    pub fn allow_multiple_answers(mut self, allow: bool) -> Self {
        self.allow_multiple_answers = Some(allow);
        self
    }

    /// Quote/reply to a specific message ID.
    pub fn quoted_message_id(mut self, id: impl Into<String>) -> Self {
        self.quoted_message_id = Some(id.into());
        self
    }

    /// Finalize and build the [`SendPollRequest`].
    pub fn build(self) -> SendPollRequest {
        SendPollRequest {
            chat_id: self.chat_id,
            name: self.name,
            options: self.options,
            allow_multiple_answers: self.allow_multiple_answers,
            quoted_message_id: self.quoted_message_id,
        }
    }
}

/// Fluent builder for constructing [`SendTextRequest`] instances.
#[derive(Debug, Clone, Default)]
pub struct TextRequestBuilder {
    chat_id: String,
    text: String,
    mentions: Option<Vec<String>>,
}

impl TextRequestBuilder {
    /// Create a new text request builder.
    pub fn new(chat_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            text: text.into(),
            mentions: None,
        }
    }

    /// Add a user JID to mention in the text message.
    pub fn mention(mut self, jid: impl Into<String>) -> Self {
        self.mentions.get_or_insert_with(Vec::new).push(jid.into());
        self
    }

    /// Add multiple user JIDs to mention.
    pub fn mentions<I, S>(mut self, jids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let list = self.mentions.get_or_insert_with(Vec::new);
        for jid in jids {
            list.push(jid.into());
        }
        self
    }

    /// Finalize and build the [`SendTextRequest`].
    pub fn build(self) -> SendTextRequest {
        SendTextRequest {
            chat_id: self.chat_id,
            text: self.text,
            mentions: self.mentions,
        }
    }
}

/// Fluent builder for constructing bulk message batches ([`SendBulkRequest`]).
#[derive(Debug, Clone, Default)]
pub struct BulkMessageBuilder {
    batch_id: Option<String>,
    messages: Vec<BulkMessageItem>,
    delay_between_messages: Option<u64>,
    randomize_delay: Option<bool>,
    stop_on_error: Option<bool>,
}

impl BulkMessageBuilder {
    /// Create a new bulk message builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set an optional client-assigned batch ID.
    pub fn batch_id(mut self, id: impl Into<String>) -> Self {
        self.batch_id = Some(id.into());
        self
    }

    /// Delay in milliseconds between sending each message in the batch.
    pub fn delay_between_messages(mut self, delay_ms: u64) -> Self {
        self.delay_between_messages = Some(delay_ms);
        self
    }

    /// Randomize the delay slightly to mimic natural human typing.
    pub fn randomize_delay(mut self, randomize: bool) -> Self {
        self.randomize_delay = Some(randomize);
        self
    }

    /// Halt execution of the batch immediately if any individual send fails.
    pub fn stop_on_error(mut self, stop: bool) -> Self {
        self.stop_on_error = Some(stop);
        self
    }

    /// Append a plain text message to the batch.
    pub fn add_text(mut self, chat_id: impl Into<String>, text: impl Into<String>) -> Self {
        self.messages.push(BulkMessageItem {
            chat_id: chat_id.into(),
            message_type: "text".into(),
            content: BulkMessageContent {
                text: Some(text.into()),
                ..Default::default()
            },
            variables: None,
        });
        self
    }

    /// Append an image message to the batch.
    pub fn add_image(
        mut self,
        chat_id: impl Into<String>,
        url: impl Into<String>,
        caption: Option<String>,
    ) -> Self {
        self.messages.push(BulkMessageItem {
            chat_id: chat_id.into(),
            message_type: "image".into(),
            content: BulkMessageContent {
                image: Some(BulkMediaDto {
                    url: Some(url.into()),
                    ..Default::default()
                }),
                caption,
                ..Default::default()
            },
            variables: None,
        });
        self
    }

    /// Append a generic bulk item.
    pub fn add_item(mut self, item: BulkMessageItem) -> Self {
        self.messages.push(item);
        self
    }

    /// Finalize and build the [`SendBulkRequest`].
    pub fn build(self) -> SendBulkRequest {
        let options = if self.delay_between_messages.is_some()
            || self.randomize_delay.is_some()
            || self.stop_on_error.is_some()
        {
            Some(BulkMessageOptions {
                delay_between_messages: self.delay_between_messages,
                randomize_delay: self.randomize_delay,
                stop_on_error: self.stop_on_error,
            })
        } else {
            None
        };

        SendBulkRequest {
            batch_id: self.batch_id,
            messages: self.messages,
            options,
        }
    }
}
