pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostCardApplicationUpdatedPayloadData {
    /// URL where the applicant completes additional identity verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_url: Option<String>,
    /// Card application ID, prefixed `ciac_`.
    #[serde(default)]
    pub id: String,
    pub object: PostCardApplicationUpdatedPayloadDataObject,
    /// The application status.
    pub status: PostCardApplicationUpdatedPayloadDataStatus,
}

impl PostCardApplicationUpdatedPayloadData {
    pub fn builder() -> PostCardApplicationUpdatedPayloadDataBuilder {
        <PostCardApplicationUpdatedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCardApplicationUpdatedPayloadDataBuilder {
    hosted_url: Option<String>,
    id: Option<String>,
    object: Option<PostCardApplicationUpdatedPayloadDataObject>,
    status: Option<PostCardApplicationUpdatedPayloadDataStatus>,
}

impl PostCardApplicationUpdatedPayloadDataBuilder {
    pub fn hosted_url(mut self, value: impl Into<String>) -> Self {
        self.hosted_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: PostCardApplicationUpdatedPayloadDataObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn status(mut self, value: PostCardApplicationUpdatedPayloadDataStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostCardApplicationUpdatedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostCardApplicationUpdatedPayloadDataBuilder::id)
    /// - [`object`](PostCardApplicationUpdatedPayloadDataBuilder::object)
    /// - [`status`](PostCardApplicationUpdatedPayloadDataBuilder::status)
    pub fn build(self) -> Result<PostCardApplicationUpdatedPayloadData, BuildError> {
        Ok(PostCardApplicationUpdatedPayloadData {
            hosted_url: self.hosted_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
