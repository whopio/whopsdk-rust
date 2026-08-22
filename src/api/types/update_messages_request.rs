pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMessagesRequest {
    /// A replacement list of file attachments for this message, such as images or videos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<UpdateMessagesRequestAttachmentsItem>>,
    /// The updated body of the message in Markdown format. For example, 'Hello **world**'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Whether this message should be pinned to the top of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<bool>,
}

impl UpdateMessagesRequest {
    pub fn builder() -> UpdateMessagesRequestBuilder {
        <UpdateMessagesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMessagesRequestBuilder {
    attachments: Option<Vec<UpdateMessagesRequestAttachmentsItem>>,
    content: Option<String>,
    is_pinned: Option<bool>,
}

impl UpdateMessagesRequestBuilder {
    pub fn attachments(mut self, value: Vec<UpdateMessagesRequestAttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn is_pinned(mut self, value: bool) -> Self {
        self.is_pinned = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMessagesRequest`].
    pub fn build(self) -> Result<UpdateMessagesRequest, BuildError> {
        Ok(UpdateMessagesRequest {
            attachments: self.attachments,
            content: self.content,
            is_pinned: self.is_pinned,
        })
    }
}
