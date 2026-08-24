pub use crate::prelude::*;

/// The thumbnail image displayed on course cards and previews. Null if no thumbnail has been uploaded.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseThumbnail {
    /// Uploaded file MIME type, such as image/jpeg, video/mp4, or audio/mpeg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The original filename of the uploaded attachment, including its file extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Represents a unique identifier that is Base64 obfuscated. It is often used to refetch an object or as key for a cache. The ID type appears in a JSON response as a String; however, it is not intended to be human-readable. When expected as an input type, any string (such as `"VXNlci0xMA=="`) or integer (such as `4`) input value will be accepted as an ID.
    #[serde(default)]
    pub id: String,
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimized_url: Option<String>,
    /// The original source URL of the attachment, such as a direct link to S3. This should never be displayed on the client and should always be passed through an Imgproxy transformer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
}

impl CourseThumbnail {
    pub fn builder() -> CourseThumbnailBuilder {
        <CourseThumbnailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseThumbnailBuilder {
    content_type: Option<String>,
    filename: Option<String>,
    id: Option<String>,
    optimized_url: Option<String>,
    source_url: Option<String>,
}

impl CourseThumbnailBuilder {
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

    pub fn optimized_url(mut self, value: impl Into<String>) -> Self {
        self.optimized_url = Some(value.into());
        self
    }

    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseThumbnail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseThumbnailBuilder::id)
    pub fn build(self) -> Result<CourseThumbnail, BuildError> {
        Ok(CourseThumbnail {
            content_type: self.content_type,
            filename: self.filename,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            optimized_url: self.optimized_url,
            source_url: self.source_url,
        })
    }
}
