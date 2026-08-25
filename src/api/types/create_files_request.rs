pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateFilesRequest {
    /// The file's size in bytes. Required when `multipart` is `true`. Multipart uploads support at most 10,000 parts of 5MB each (about 50 GB).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_size: Option<i64>,
    /// The name of the file including its extension, e.g. `terms.pdf`.
    #[serde(default)]
    pub filename: String,
    /// Upload the file in 5MB parts. Required for files larger than 5GB; useful above ~100MB. The file must be larger than 5MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipart: Option<bool>,
    /// `public` files are served via an unsigned CDN URL — use for assets anyone may see. `private` files are served via a signed, expiring URL — use for sensitive documents. Defaults to `private`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<CreateFilesRequestVisibility>,
}

impl CreateFilesRequest {
    pub fn builder() -> CreateFilesRequestBuilder {
        <CreateFilesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFilesRequestBuilder {
    byte_size: Option<i64>,
    filename: Option<String>,
    multipart: Option<bool>,
    visibility: Option<CreateFilesRequestVisibility>,
}

impl CreateFilesRequestBuilder {
    pub fn byte_size(mut self, value: i64) -> Self {
        self.byte_size = Some(value);
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn multipart(mut self, value: bool) -> Self {
        self.multipart = Some(value);
        self
    }

    pub fn visibility(mut self, value: CreateFilesRequestVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateFilesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`filename`](CreateFilesRequestBuilder::filename)
    pub fn build(self) -> Result<CreateFilesRequest, BuildError> {
        Ok(CreateFilesRequest {
            byte_size: self.byte_size,
            filename: self
                .filename
                .ok_or_else(|| BuildError::missing_field("filename"))?,
            multipart: self.multipart,
            visibility: self.visibility,
        })
    }
}
