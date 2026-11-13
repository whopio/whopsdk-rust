pub use crate::prelude::*;

/// Represents an image attachment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SupportChannelListItemLastMessageAttachmentsItem {
    /// Uploaded file MIME type, such as image/jpeg, video/mp4, or audio/mpeg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The original filename of the uploaded attachment, including its file extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

impl SupportChannelListItemLastMessageAttachmentsItem {
    pub fn builder() -> SupportChannelListItemLastMessageAttachmentsItemBuilder {
        <SupportChannelListItemLastMessageAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SupportChannelListItemLastMessageAttachmentsItemBuilder {
    content_type: Option<String>,
    filename: Option<String>,
}

impl SupportChannelListItemLastMessageAttachmentsItemBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SupportChannelListItemLastMessageAttachmentsItem`].
    pub fn build(self) -> Result<SupportChannelListItemLastMessageAttachmentsItem, BuildError> {
        Ok(SupportChannelListItemLastMessageAttachmentsItem {
            content_type: self.content_type,
            filename: self.filename,
        })
    }
}
