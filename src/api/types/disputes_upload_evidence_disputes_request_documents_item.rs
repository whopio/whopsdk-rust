pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UploadEvidenceDisputesRequestDocumentsItem {
    /// The ID returned by a direct upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    /// What kind of evidence the document is.
    pub document_type: UploadEvidenceDisputesRequestDocumentsItemDocumentType,
    /// The file itself. Send it as a file part to upload and attach in one call, or use `id`/`direct_upload_id` for a file that is already stored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// The ID of a file already stored on Whop, prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UploadEvidenceDisputesRequestDocumentsItem {
    pub fn builder() -> UploadEvidenceDisputesRequestDocumentsItemBuilder {
        <UploadEvidenceDisputesRequestDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadEvidenceDisputesRequestDocumentsItemBuilder {
    direct_upload_id: Option<String>,
    document_type: Option<UploadEvidenceDisputesRequestDocumentsItemDocumentType>,
    file: Option<String>,
    id: Option<String>,
}

impl UploadEvidenceDisputesRequestDocumentsItemBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn document_type(
        mut self,
        value: UploadEvidenceDisputesRequestDocumentsItemDocumentType,
    ) -> Self {
        self.document_type = Some(value);
        self
    }

    pub fn file(mut self, value: impl Into<String>) -> Self {
        self.file = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UploadEvidenceDisputesRequestDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_type`](UploadEvidenceDisputesRequestDocumentsItemBuilder::document_type)
    pub fn build(self) -> Result<UploadEvidenceDisputesRequestDocumentsItem, BuildError> {
        Ok(UploadEvidenceDisputesRequestDocumentsItem {
            direct_upload_id: self.direct_upload_id,
            document_type: self
                .document_type
                .ok_or_else(|| BuildError::missing_field("document_type"))?,
            file: self.file,
            id: self.id,
        })
    }
}
