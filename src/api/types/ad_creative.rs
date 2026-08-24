pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdCreative {
    /// The saved crop window for this creative, in source image pixels. Null for the original asset or a format that has not been cropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<AdCreativeCrop>,
    /// The placement variant this asset covers, or null for the original asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<AdCreativeFormat>,
    /// The creative attachment's file id.
    #[serde(default)]
    pub id: String,
    /// The kind of asset, image or video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// CDN url of the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AdCreative {
    pub fn builder() -> AdCreativeBuilder {
        <AdCreativeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdCreativeBuilder {
    crop: Option<AdCreativeCrop>,
    format: Option<AdCreativeFormat>,
    id: Option<String>,
    media_type: Option<String>,
    url: Option<String>,
}

impl AdCreativeBuilder {
    pub fn crop(mut self, value: AdCreativeCrop) -> Self {
        self.crop = Some(value);
        self
    }

    pub fn format(mut self, value: AdCreativeFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn media_type(mut self, value: impl Into<String>) -> Self {
        self.media_type = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdCreative`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AdCreativeBuilder::id)
    pub fn build(self) -> Result<AdCreative, BuildError> {
        Ok(AdCreative {
            crop: self.crop,
            format: self.format,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            media_type: self.media_type,
            url: self.url,
        })
    }
}
