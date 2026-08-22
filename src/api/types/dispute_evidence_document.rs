pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DisputeEvidenceDocument {
    /// The uploaded file's MIME type. Uploads are restricted to the types the processor accepts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<DisputeEvidenceDocumentContentType>,
    /// What kind of evidence the document is.
    pub document_type: DisputeEvidenceDocumentDocumentType,
    /// The uploaded file's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The attachment's ID, prefixed `file_`.
    #[serde(default)]
    pub id: String,
    /// A URL to download the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl DisputeEvidenceDocument {
    pub fn builder() -> DisputeEvidenceDocumentBuilder {
        <DisputeEvidenceDocumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeEvidenceDocumentBuilder {
    content_type: Option<DisputeEvidenceDocumentContentType>,
    document_type: Option<DisputeEvidenceDocumentDocumentType>,
    filename: Option<String>,
    id: Option<String>,
    url: Option<String>,
}

impl DisputeEvidenceDocumentBuilder {
    pub fn content_type(mut self, value: DisputeEvidenceDocumentContentType) -> Self {
        self.content_type = Some(value);
        self
    }

    pub fn document_type(mut self, value: DisputeEvidenceDocumentDocumentType) -> Self {
        self.document_type = Some(value);
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeEvidenceDocument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_type`](DisputeEvidenceDocumentBuilder::document_type)
    /// - [`id`](DisputeEvidenceDocumentBuilder::id)
    pub fn build(self) -> Result<DisputeEvidenceDocument, BuildError> {
        Ok(DisputeEvidenceDocument {
            content_type: self.content_type,
            document_type: self
                .document_type
                .ok_or_else(|| BuildError::missing_field("document_type"))?,
            filename: self.filename,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url,
        })
    }
}
