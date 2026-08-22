pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBountySubmissionsRequest {
    /// Affiliate code crediting the referrer, when the worker arrived through one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_code: Option<String>,
    /// The bounty to submit to (`bnty_` tag).
    #[serde(default)]
    pub bounty_id: String,
    /// The submitted work. Combine `urls`, `file_ids`, and `caption` freely; at least one link or file is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverable: Option<CreateBountySubmissionsRequestDeliverable>,
    /// Optional capture metadata describing where and how the footage was recorded. Persisted on the submission. On a `data_capture` bounty every field except `fov` is required whenever metadata is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CreateBountySubmissionsRequestMetadata>,
}

impl CreateBountySubmissionsRequest {
    pub fn builder() -> CreateBountySubmissionsRequestBuilder {
        <CreateBountySubmissionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBountySubmissionsRequestBuilder {
    affiliate_code: Option<String>,
    bounty_id: Option<String>,
    deliverable: Option<CreateBountySubmissionsRequestDeliverable>,
    metadata: Option<CreateBountySubmissionsRequestMetadata>,
}

impl CreateBountySubmissionsRequestBuilder {
    pub fn affiliate_code(mut self, value: impl Into<String>) -> Self {
        self.affiliate_code = Some(value.into());
        self
    }

    pub fn bounty_id(mut self, value: impl Into<String>) -> Self {
        self.bounty_id = Some(value.into());
        self
    }

    pub fn deliverable(mut self, value: CreateBountySubmissionsRequestDeliverable) -> Self {
        self.deliverable = Some(value);
        self
    }

    pub fn metadata(mut self, value: CreateBountySubmissionsRequestMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBountySubmissionsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bounty_id`](CreateBountySubmissionsRequestBuilder::bounty_id)
    pub fn build(self) -> Result<CreateBountySubmissionsRequest, BuildError> {
        Ok(CreateBountySubmissionsRequest {
            affiliate_code: self.affiliate_code,
            bounty_id: self
                .bounty_id
                .ok_or_else(|| BuildError::missing_field("bounty_id"))?,
            deliverable: self.deliverable,
            metadata: self.metadata,
        })
    }
}
