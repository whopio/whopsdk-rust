pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MediaAssetFile {
    /// File ID, prefixed `file_`.
    #[serde(default)]
    pub id: String,
    /// CDN URL for downloading the file.
    #[serde(default)]
    pub url: String,
}

impl MediaAssetFile {
    pub fn builder() -> MediaAssetFileBuilder {
        <MediaAssetFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MediaAssetFileBuilder {
    id: Option<String>,
    url: Option<String>,
}

impl MediaAssetFileBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MediaAssetFile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MediaAssetFileBuilder::id)
    /// - [`url`](MediaAssetFileBuilder::url)
    pub fn build(self) -> Result<MediaAssetFile, BuildError> {
        Ok(MediaAssetFile {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
