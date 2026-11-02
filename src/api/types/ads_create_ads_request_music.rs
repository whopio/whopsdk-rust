pub use crate::prelude::*;

/// The looping track a TikTok carousel ad plays — an MP3 you uploaded, no larger than 10MB. Required for TikTok carousels (image creatives); TikTok-only.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestMusic {
    /// Uploaded MP3 file ID, prefixed `file_`.
    #[serde(default)]
    pub id: String,
}

impl CreateAdsRequestMusic {
    pub fn builder() -> CreateAdsRequestMusicBuilder {
        <CreateAdsRequestMusicBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestMusicBuilder {
    id: Option<String>,
}

impl CreateAdsRequestMusicBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestMusic`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateAdsRequestMusicBuilder::id)
    pub fn build(self) -> Result<CreateAdsRequestMusic, BuildError> {
        Ok(CreateAdsRequestMusic {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
