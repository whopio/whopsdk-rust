pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompleteFilesRequest {
    /// Every uploaded part, in order.
    #[serde(default)]
    pub multipart_parts: Vec<CompleteFilesRequestMultipartPartsItem>,
    /// The ID of the multipart upload, returned by Create File.
    #[serde(default)]
    pub multipart_upload_id: String,
}

impl CompleteFilesRequest {
    pub fn builder() -> CompleteFilesRequestBuilder {
        <CompleteFilesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteFilesRequestBuilder {
    multipart_parts: Option<Vec<CompleteFilesRequestMultipartPartsItem>>,
    multipart_upload_id: Option<String>,
}

impl CompleteFilesRequestBuilder {
    pub fn multipart_parts(mut self, value: Vec<CompleteFilesRequestMultipartPartsItem>) -> Self {
        self.multipart_parts = Some(value);
        self
    }

    pub fn multipart_upload_id(mut self, value: impl Into<String>) -> Self {
        self.multipart_upload_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompleteFilesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`multipart_parts`](CompleteFilesRequestBuilder::multipart_parts)
    /// - [`multipart_upload_id`](CompleteFilesRequestBuilder::multipart_upload_id)
    pub fn build(self) -> Result<CompleteFilesRequest, BuildError> {
        Ok(CompleteFilesRequest {
            multipart_parts: self
                .multipart_parts
                .ok_or_else(|| BuildError::missing_field("multipart_parts"))?,
            multipart_upload_id: self
                .multipart_upload_id
                .ok_or_else(|| BuildError::missing_field("multipart_upload_id"))?,
        })
    }
}
