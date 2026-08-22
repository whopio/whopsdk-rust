pub use crate::prelude::*;

/// A file that has been uploaded or is pending upload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct File {
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
    /// The current upload status of the file (e.g., pending, ready).
    pub upload_status: UploadStatuses,
    /// The URL for accessing the file. For public files, this is a permanent CDN URL. For private files, this is a signed URL that expires. Null if the file has not finished uploading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Whether the file is publicly accessible or requires authentication.
    pub visibility: FileVisibility,
}

impl File {
    pub fn builder() -> FileBuilder {
        <FileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileBuilder {
    content_type: Option<String>,
    filename: Option<String>,
    id: Option<String>,
    size: Option<String>,
    upload_status: Option<UploadStatuses>,
    url: Option<String>,
    visibility: Option<FileVisibility>,
}

impl FileBuilder {
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

    pub fn upload_status(mut self, value: UploadStatuses) -> Self {
        self.upload_status = Some(value);
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

    /// Consumes the builder and constructs a [`File`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](FileBuilder::id)
    /// - [`upload_status`](FileBuilder::upload_status)
    /// - [`visibility`](FileBuilder::visibility)
    pub fn build(self) -> Result<File, BuildError> {
        Ok(File {
            content_type: self.content_type,
            filename: self.filename,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            size: self.size,
            upload_status: self
                .upload_status
                .ok_or_else(|| BuildError::missing_field("upload_status"))?,
            url: self.url,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
        })
    }
}
