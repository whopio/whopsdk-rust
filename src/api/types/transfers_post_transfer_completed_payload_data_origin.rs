pub use crate::prelude::*;

/// Business account or user sending the transfer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostTransferCompletedPayloadDataOrigin {
    /// Account or user ID.
    #[serde(default)]
    pub id: String,
    /// Business logo or user avatar URL. Null when no image is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Business or user display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the profile is a business account or a user.
    pub object: PostTransferCompletedPayloadDataOriginObject,
}

impl PostTransferCompletedPayloadDataOrigin {
    pub fn builder() -> PostTransferCompletedPayloadDataOriginBuilder {
        <PostTransferCompletedPayloadDataOriginBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostTransferCompletedPayloadDataOriginBuilder {
    id: Option<String>,
    logo_url: Option<String>,
    name: Option<String>,
    object: Option<PostTransferCompletedPayloadDataOriginObject>,
}

impl PostTransferCompletedPayloadDataOriginBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn object(mut self, value: PostTransferCompletedPayloadDataOriginObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostTransferCompletedPayloadDataOrigin`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostTransferCompletedPayloadDataOriginBuilder::id)
    /// - [`object`](PostTransferCompletedPayloadDataOriginBuilder::object)
    pub fn build(self) -> Result<PostTransferCompletedPayloadDataOrigin, BuildError> {
        Ok(PostTransferCompletedPayloadDataOrigin {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo_url: self.logo_url,
            name: self.name,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}
