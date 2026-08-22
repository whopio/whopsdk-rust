pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostCardApplicationCreatedPayloadData {
    /// URL where the applicant completes additional identity verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_url: Option<String>,
    /// Card application ID, prefixed `ciac_`.
    #[serde(default)]
    pub id: String,
    pub object: PostCardApplicationCreatedPayloadDataObject,
    /// The application status.
    pub status: PostCardApplicationCreatedPayloadDataStatus,
}

impl PostCardApplicationCreatedPayloadData {
    pub fn builder() -> PostCardApplicationCreatedPayloadDataBuilder {
        <PostCardApplicationCreatedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCardApplicationCreatedPayloadDataBuilder {
    hosted_url: Option<String>,
    id: Option<String>,
    object: Option<PostCardApplicationCreatedPayloadDataObject>,
    status: Option<PostCardApplicationCreatedPayloadDataStatus>,
}

impl PostCardApplicationCreatedPayloadDataBuilder {
    pub fn hosted_url(mut self, value: impl Into<String>) -> Self {
        self.hosted_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: PostCardApplicationCreatedPayloadDataObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn status(mut self, value: PostCardApplicationCreatedPayloadDataStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostCardApplicationCreatedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostCardApplicationCreatedPayloadDataBuilder::id)
    /// - [`object`](PostCardApplicationCreatedPayloadDataBuilder::object)
    /// - [`status`](PostCardApplicationCreatedPayloadDataBuilder::status)
    pub fn build(self) -> Result<PostCardApplicationCreatedPayloadData, BuildError> {
        Ok(PostCardApplicationCreatedPayloadData {
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
