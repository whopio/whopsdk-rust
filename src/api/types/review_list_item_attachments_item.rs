pub use crate::prelude::*;

/// Represents an image attachment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReviewListItemAttachmentsItem {
    /// Uploaded file MIME type, such as image/jpeg, video/mp4, or audio/mpeg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The original filename of the uploaded attachment, including its file extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Represents a unique identifier that is Base64 obfuscated. It is often used to refetch an object or as key for a cache. The ID type appears in a JSON response as a String; however, it is not intended to be human-readable. When expected as an input type, any string (such as `"VXNlci0xMA=="`) or integer (such as `4`) input value will be accepted as an ID.
    #[serde(default)]
    pub id: String,
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ReviewListItemAttachmentsItem {
    pub fn builder() -> ReviewListItemAttachmentsItemBuilder {
        <ReviewListItemAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewListItemAttachmentsItemBuilder {
    content_type: Option<String>,
    filename: Option<String>,
    id: Option<String>,
    url: Option<String>,
}

impl ReviewListItemAttachmentsItemBuilder {
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

    /// Consumes the builder and constructs a [`ReviewListItemAttachmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReviewListItemAttachmentsItemBuilder::id)
    pub fn build(self) -> Result<ReviewListItemAttachmentsItem, BuildError> {
        Ok(ReviewListItemAttachmentsItem {
            content_type: self.content_type,
            filename: self.filename,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url,
        })
    }
}
