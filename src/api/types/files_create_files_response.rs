pub use crate::prelude::*;

/// A file that has been uploaded or is pending upload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFilesResponse {
    /// The MIME type of the uploaded file (e.g., image/jpeg, video/mp4, audio/mpeg).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The original filename of the uploaded file, including its file extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The unique identifier for the file.
    #[serde(default)]
    pub id: String,
    /// The file size in bytes. Null if the file has not finished uploading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Headers to include in the upload request. Only present in the response from the create mutation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_headers: Option<HashMap<String, serde_json::Value>>,
    /// The current upload status of the file (e.g., pending, ready).
    pub upload_status: UploadStatuses,
    /// The presigned URL to upload the file contents to. Only present in the response from the create mutation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
    /// The URL for accessing the file. For public files, this is a permanent CDN URL. For private files, this is a signed URL that expires. Null if the file has not finished uploading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Whether the file is publicly accessible or requires authentication.
    pub visibility: FileVisibility,
}

impl CreateFilesResponse {
    pub fn builder() -> CreateFilesResponseBuilder {
        <CreateFilesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFilesResponseBuilder {
    content_type: Option<String>,
    filename: Option<String>,
    id: Option<String>,
    size: Option<String>,
    upload_headers: Option<HashMap<String, serde_json::Value>>,
    upload_status: Option<UploadStatuses>,
    upload_url: Option<String>,
    url: Option<String>,
    visibility: Option<FileVisibility>,
}

impl CreateFilesResponseBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
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

    pub fn size(mut self, value: impl Into<String>) -> Self {
        self.size = Some(value.into());
        self
    }

    pub fn upload_headers(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.upload_headers = Some(value);
        self
    }

    pub fn upload_status(mut self, value: UploadStatuses) -> Self {
        self.upload_status = Some(value);
        self
    }

    pub fn upload_url(mut self, value: impl Into<String>) -> Self {
        self.upload_url = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: FileVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateFilesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateFilesResponseBuilder::id)
    /// - [`upload_status`](CreateFilesResponseBuilder::upload_status)
    /// - [`visibility`](CreateFilesResponseBuilder::visibility)
    pub fn build(self) -> Result<CreateFilesResponse, BuildError> {
        Ok(CreateFilesResponse {
            content_type: self.content_type,
            filename: self.filename,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            size: self.size,
            upload_headers: self.upload_headers,
            upload_status: self
                .upload_status
                .ok_or_else(|| BuildError::missing_field("upload_status"))?,
            upload_url: self.upload_url,
            url: self.url,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
        })
    }
}
