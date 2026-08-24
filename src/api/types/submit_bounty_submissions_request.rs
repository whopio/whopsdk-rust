pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitBountySubmissionsRequest {
    /// Work to attach to the submission. Combine `urls`, `file_ids`, and `caption` freely; all are optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverable: Option<SubmitBountySubmissionsRequestDeliverable>,
}

impl SubmitBountySubmissionsRequest {
    pub fn builder() -> SubmitBountySubmissionsRequestBuilder {
        <SubmitBountySubmissionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitBountySubmissionsRequestBuilder {
    deliverable: Option<SubmitBountySubmissionsRequestDeliverable>,
}

impl SubmitBountySubmissionsRequestBuilder {
    pub fn deliverable(mut self, value: SubmitBountySubmissionsRequestDeliverable) -> Self {
        self.deliverable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitBountySubmissionsRequest`].
    pub fn build(self) -> Result<SubmitBountySubmissionsRequest, BuildError> {
        Ok(SubmitBountySubmissionsRequest {
            deliverable: self.deliverable,
        })
    }
}
