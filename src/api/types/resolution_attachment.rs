pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolutionAttachment {
    /// The file's MIME type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The original file name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Unique identifier for the attachment, prefixed `att_`.
    #[serde(default)]
    pub id: String,
    /// A URL to view or download the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ResolutionAttachment {
    pub fn builder() -> ResolutionAttachmentBuilder {
        <ResolutionAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionAttachmentBuilder {
    content_type: Option<String>,
    filename: Option<String>,
    id: Option<String>,
    url: Option<String>,
}

impl ResolutionAttachmentBuilder {
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

    /// Consumes the builder and constructs a [`ResolutionAttachment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ResolutionAttachmentBuilder::id)
    pub fn build(self) -> Result<ResolutionAttachment, BuildError> {
        Ok(ResolutionAttachment {
            content_type: self.content_type,
            filename: self.filename,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url,
        })
    }
}
