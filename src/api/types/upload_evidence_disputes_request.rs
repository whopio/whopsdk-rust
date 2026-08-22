pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UploadEvidenceDisputesRequest {
    /// The full set of evidence documents the dispute should carry. Replaces all previously uploaded documents.
    #[serde(default)]
    pub documents: Vec<UploadEvidenceDisputesRequestDocumentsItem>,
}

impl UploadEvidenceDisputesRequest {
    pub fn builder() -> UploadEvidenceDisputesRequestBuilder {
        <UploadEvidenceDisputesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadEvidenceDisputesRequestBuilder {
    documents: Option<Vec<UploadEvidenceDisputesRequestDocumentsItem>>,
}

impl UploadEvidenceDisputesRequestBuilder {
    pub fn documents(mut self, value: Vec<UploadEvidenceDisputesRequestDocumentsItem>) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UploadEvidenceDisputesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`documents`](UploadEvidenceDisputesRequestBuilder::documents)
    pub fn build(self) -> Result<UploadEvidenceDisputesRequest, BuildError> {
        Ok(UploadEvidenceDisputesRequest {
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
