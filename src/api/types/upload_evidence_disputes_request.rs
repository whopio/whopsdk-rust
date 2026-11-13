pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UploadEvidenceDisputesRequest {
    /// The full set of evidence documents the dispute should carry, beyond the four fixed evidence slots. Replaces all previously uploaded documents; an empty list removes them all. Send up to 10, 10MB each and 25MB in total. Accepted content types: application/pdf, application/json, image/jpeg, image/png, image/webp — any other type is rejected. Upload files through `POST /files` and reference them by `id`, or send the files as multipart file parts to upload and attach in one call. Policy documents (`return_policy`, `shipping_policy`, `cancellation_policy`, `terms_of_service`) default from the account's own documents; uploading one here replaces the account copy for this dispute, and a `cancellation_policy` or `return_policy` upload also takes precedence over the matching fixed evidence slot.
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
