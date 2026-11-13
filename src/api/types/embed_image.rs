pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmbedImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blurhash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(default)]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

impl EmbedImage {
    pub fn builder() -> EmbedImageBuilder {
        <EmbedImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmbedImageBuilder {
    blurhash: Option<String>,
    height: Option<i64>,
    url: Option<String>,
    width: Option<i64>,
}

impl EmbedImageBuilder {
    pub fn blurhash(mut self, value: impl Into<String>) -> Self {
        self.blurhash = Some(value.into());
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmbedImage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](EmbedImageBuilder::url)
    pub fn build(self) -> Result<EmbedImage, BuildError> {
        Ok(EmbedImage {
            blurhash: self.blurhash,
            height: self.height,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            width: self.width,
        })
    }
}
