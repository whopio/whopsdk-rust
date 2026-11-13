pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct File {
    /// BlurHash placeholder for an image or video. Null when unavailable or removed by moderation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blurhash: Option<String>,
    /// The original file's MIME type, for example `application/pdf`. It is kept when moderation removes the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// When the file was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Audio or video duration in seconds. Null when unavailable or removed by moderation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub duration_seconds: Option<f64>,
    /// The original filename, including its extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Image or video height in pixels. Null when unavailable or removed by moderation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The file's ID, prefixed `file_`.
    #[serde(default)]
    pub id: String,
    /// The byte size each part (except the last) must be. Present only on create, and only for multipart uploads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipart_chunk_size: Option<i64>,
    /// The ID of the multipart upload, passed back to `complete`. Present only on create, and only for multipart uploads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipart_upload_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipart_upload_urls: Option<Vec<FileMultipartUrl>>,
    /// The type of this object, always `file`.
    #[serde(default)]
    pub object: String,
    /// Whether moderation removed the file. When `true`, `url` serves a replacement image. `filename`, `size` and `content_type` still describe the original file.
    #[serde(default)]
    pub removed_by_moderation: bool,
    /// The file size in bytes. `null` until the upload has finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Headers to send with the upload PUT. Present only on create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_headers: Option<HashMap<String, serde_json::Value>>,
    /// Where the file is in its upload lifecycle.
    pub upload_status: FileUploadStatus,
    /// Presigned URL to PUT the file's bytes to. Present only on create, and only for single-part uploads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
    /// A URL to download the file. Public files get a permanent CDN URL, and private files get a signed URL that expires. `null` until the upload has finished. When moderation removes the file, a replacement image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// `public` files are served from an unsigned CDN URL. `private` files are served from a signed URL that expires.
    pub visibility: FileVisibility,
    /// Audio waveform URL. Null when unavailable or removed by moderation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waveform_url: Option<String>,
    /// Image or video width in pixels. Null when unavailable or removed by moderation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

impl File {
    pub fn builder() -> FileBuilder {
        <FileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileBuilder {
    blurhash: Option<String>,
    content_type: Option<String>,
    created_at: Option<String>,
    duration_seconds: Option<f64>,
    filename: Option<String>,
    height: Option<i64>,
    id: Option<String>,
    multipart_chunk_size: Option<i64>,
    multipart_upload_id: Option<String>,
    multipart_upload_urls: Option<Vec<FileMultipartUrl>>,
    object: Option<String>,
    removed_by_moderation: Option<bool>,
    size: Option<i64>,
    upload_headers: Option<HashMap<String, serde_json::Value>>,
    upload_status: Option<FileUploadStatus>,
    upload_url: Option<String>,
    url: Option<String>,
    visibility: Option<FileVisibility>,
    waveform_url: Option<String>,
    width: Option<i64>,
}

impl FileBuilder {
    pub fn blurhash(mut self, value: impl Into<String>) -> Self {
        self.blurhash = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn duration_seconds(mut self, value: f64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn multipart_chunk_size(mut self, value: i64) -> Self {
        self.multipart_chunk_size = Some(value);
        self
    }

    pub fn multipart_upload_id(mut self, value: impl Into<String>) -> Self {
        self.multipart_upload_id = Some(value.into());
        self
    }

    pub fn multipart_upload_urls(mut self, value: Vec<FileMultipartUrl>) -> Self {
        self.multipart_upload_urls = Some(value);
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn removed_by_moderation(mut self, value: bool) -> Self {
        self.removed_by_moderation = Some(value);
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn upload_headers(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.upload_headers = Some(value);
        self
    }

    pub fn upload_status(mut self, value: FileUploadStatus) -> Self {
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

    pub fn waveform_url(mut self, value: impl Into<String>) -> Self {
        self.waveform_url = Some(value.into());
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`File`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](FileBuilder::created_at)
    /// - [`id`](FileBuilder::id)
    /// - [`object`](FileBuilder::object)
    /// - [`removed_by_moderation`](FileBuilder::removed_by_moderation)
    /// - [`upload_status`](FileBuilder::upload_status)
    /// - [`visibility`](FileBuilder::visibility)
    pub fn build(self) -> Result<File, BuildError> {
        Ok(File {
            blurhash: self.blurhash,
            content_type: self.content_type,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            duration_seconds: self.duration_seconds,
            filename: self.filename,
            height: self.height,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            multipart_chunk_size: self.multipart_chunk_size,
            multipart_upload_id: self.multipart_upload_id,
            multipart_upload_urls: self.multipart_upload_urls,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            removed_by_moderation: self
                .removed_by_moderation
                .ok_or_else(|| BuildError::missing_field("removed_by_moderation"))?,
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
            waveform_url: self.waveform_url,
            width: self.width,
        })
    }
}
