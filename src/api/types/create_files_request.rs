pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateFilesRequest {
    /// The name of the file including its extension (e.g., "photo.png" or "document.pdf").
    #[serde(default)]
    pub filename: String,
    /// Controls whether the file is publicly accessible via CDN or requires authentication. Defaults to private.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<FileVisibility>,
}

impl CreateFilesRequest {
    pub fn builder() -> CreateFilesRequestBuilder {
        <CreateFilesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFilesRequestBuilder {
    filename: Option<String>,
    visibility: Option<FileVisibility>,
}

impl CreateFilesRequestBuilder {
    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: FileVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateFilesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`filename`](CreateFilesRequestBuilder::filename)
    pub fn build(self) -> Result<CreateFilesRequest, BuildError> {
        Ok(CreateFilesRequest {
            filename: self
                .filename
                .ok_or_else(|| BuildError::missing_field("filename"))?,
            visibility: self.visibility,
        })
    }
}
