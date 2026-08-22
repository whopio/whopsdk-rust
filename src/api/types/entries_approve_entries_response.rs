pub use crate::prelude::*;

/// An object representing an asynchronous job.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApproveEntriesResponse {
    /// The ID of the job.
    #[serde(default)]
    pub job_id: String,
}

impl ApproveEntriesResponse {
    pub fn builder() -> ApproveEntriesResponseBuilder {
        <ApproveEntriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApproveEntriesResponseBuilder {
    job_id: Option<String>,
}

impl ApproveEntriesResponseBuilder {
    pub fn job_id(mut self, value: impl Into<String>) -> Self {
        self.job_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApproveEntriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`job_id`](ApproveEntriesResponseBuilder::job_id)
    pub fn build(self) -> Result<ApproveEntriesResponse, BuildError> {
        Ok(ApproveEntriesResponse {
            job_id: self
                .job_id
                .ok_or_else(|| BuildError::missing_field("job_id"))?,
        })
    }
}
