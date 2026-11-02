pub use crate::prelude::*;

/// The looping track a TikTok carousel ad plays — an MP3 you uploaded, no larger than 10MB. Omitted leaves the ad's music untouched. Null removes it before launch; a submitted carousel takes a replacement track instead. TikTok-only.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdsRequestMusic {
    /// Uploaded MP3 file ID, prefixed `file_`.
    #[serde(default)]
    pub id: String,
}

impl UpdateAdsRequestMusic {
    pub fn builder() -> UpdateAdsRequestMusicBuilder {
        <UpdateAdsRequestMusicBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestMusicBuilder {
    id: Option<String>,
}

impl UpdateAdsRequestMusicBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestMusic`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateAdsRequestMusicBuilder::id)
    pub fn build(self) -> Result<UpdateAdsRequestMusic, BuildError> {
        Ok(UpdateAdsRequestMusic {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
