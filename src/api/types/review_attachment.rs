pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReviewAttachment {
    /// Uploaded file MIME type, such as image/jpeg, video/mp4, or audio/mpeg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The original filename of the uploaded attachment, including its file extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The unique identifier for the attachment.
    #[serde(default)]
    pub id: String,
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ReviewAttachment {
    pub fn builder() -> ReviewAttachmentBuilder {
        <ReviewAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewAttachmentBuilder {
    content_type: Option<String>,
    filename: Option<String>,
    id: Option<String>,
    url: Option<String>,
}

impl ReviewAttachmentBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReviewAttachment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReviewAttachmentBuilder::id)
    pub fn build(self) -> Result<ReviewAttachment, BuildError> {
        Ok(ReviewAttachment {
            content_type: self.content_type,
            filename: self.filename,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url,
        })
    }
}
