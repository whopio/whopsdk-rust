pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostCardApplicationApprovedPayloadData {
    /// URL where the applicant completes additional identity verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_url: Option<String>,
    /// Card application ID, prefixed `ciac_`.
    #[serde(default)]
    pub id: String,
    pub object: PostCardApplicationApprovedPayloadDataObject,
    /// The application status.
    pub status: PostCardApplicationApprovedPayloadDataStatus,
}

impl PostCardApplicationApprovedPayloadData {
    pub fn builder() -> PostCardApplicationApprovedPayloadDataBuilder {
        <PostCardApplicationApprovedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCardApplicationApprovedPayloadDataBuilder {
    hosted_url: Option<String>,
    id: Option<String>,
    object: Option<PostCardApplicationApprovedPayloadDataObject>,
    status: Option<PostCardApplicationApprovedPayloadDataStatus>,
}

impl PostCardApplicationApprovedPayloadDataBuilder {
    pub fn hosted_url(mut self, value: impl Into<String>) -> Self {
        self.hosted_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: PostCardApplicationApprovedPayloadDataObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn status(mut self, value: PostCardApplicationApprovedPayloadDataStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostCardApplicationApprovedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostCardApplicationApprovedPayloadDataBuilder::id)
    /// - [`object`](PostCardApplicationApprovedPayloadDataBuilder::object)
    /// - [`status`](PostCardApplicationApprovedPayloadDataBuilder::status)
    pub fn build(self) -> Result<PostCardApplicationApprovedPayloadData, BuildError> {
        Ok(PostCardApplicationApprovedPayloadData {
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
