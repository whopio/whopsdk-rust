pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeAttachment {
    /// The uploaded file's MIME type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The uploaded file's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The attachment's ID. `null` for a Whop-hosted policy, which is not an uploaded file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether this is Whop's own hosted policy, standing in because the seller uploaded none. Sending it back on a PATCH changes nothing.
    #[serde(default)]
    pub platform: bool,
    /// A URL to download the attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl DisputeAttachment {
    pub fn builder() -> DisputeAttachmentBuilder {
        <DisputeAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeAttachmentBuilder {
    content_type: Option<String>,
    filename: Option<String>,
    id: Option<String>,
    platform: Option<bool>,
    url: Option<String>,
}

impl DisputeAttachmentBuilder {
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

    pub fn platform(mut self, value: bool) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeAttachment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](DisputeAttachmentBuilder::platform)
    pub fn build(self) -> Result<DisputeAttachment, BuildError> {
        Ok(DisputeAttachment {
            content_type: self.content_type,
            filename: self.filename,
            id: self.id,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            url: self.url,
        })
    }
}
