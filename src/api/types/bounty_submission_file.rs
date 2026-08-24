pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BountySubmissionFile {
    /// Broad kind of file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<BountySubmissionFileAttachmentType>,
    /// MIME type of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Name the file was uploaded with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// File ID, prefixed `file_`.
    #[serde(default)]
    pub id: String,
    /// Temporary download URL for the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl BountySubmissionFile {
    pub fn builder() -> BountySubmissionFileBuilder {
        <BountySubmissionFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BountySubmissionFileBuilder {
    attachment_type: Option<BountySubmissionFileAttachmentType>,
    content_type: Option<String>,
    filename: Option<String>,
    id: Option<String>,
    url: Option<String>,
}

impl BountySubmissionFileBuilder {
    pub fn attachment_type(mut self, value: BountySubmissionFileAttachmentType) -> Self {
        self.attachment_type = Some(value);
        self
    }

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

    /// Consumes the builder and constructs a [`BountySubmissionFile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BountySubmissionFileBuilder::id)
    pub fn build(self) -> Result<BountySubmissionFile, BuildError> {
        Ok(BountySubmissionFile {
            attachment_type: self.attachment_type,
            content_type: self.content_type,
            filename: self.filename,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url,
        })
    }
}
