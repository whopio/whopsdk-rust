pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FileMultipartUrl {
    /// The 1-based index of this part within the multipart upload.
    #[serde(default)]
    pub part_number: i64,
    /// The presigned URL to PUT this part's bytes to.
    #[serde(default)]
    pub url: String,
}

impl FileMultipartUrl {
    pub fn builder() -> FileMultipartUrlBuilder {
        <FileMultipartUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileMultipartUrlBuilder {
    part_number: Option<i64>,
    url: Option<String>,
}

impl FileMultipartUrlBuilder {
    pub fn part_number(mut self, value: i64) -> Self {
        self.part_number = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FileMultipartUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`part_number`](FileMultipartUrlBuilder::part_number)
    /// - [`url`](FileMultipartUrlBuilder::url)
    pub fn build(self) -> Result<FileMultipartUrl, BuildError> {
        Ok(FileMultipartUrl {
            part_number: self
                .part_number
                .ok_or_else(|| BuildError::missing_field("part_number"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
