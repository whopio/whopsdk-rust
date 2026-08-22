pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SocialAccountPost {
    /// The post's call-to-action button, for example shop_now (Facebook only; null for Instagram and TikTok).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_to_action: Option<SocialAccountPostCallToAction>,
    /// The URL the post's call-to-action drives to (Facebook only; null for Instagram and TikTok).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_url: Option<String>,
    /// An iframe-embeddable URL for previewing the post inline (the platform's player or post embed). For TikTok this is the only preview, since media_url is null; for Facebook and Instagram it supplements media_url. Null when no public embed is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,
    /// The platform's own identifier for the post or media. Use it to reference the post on an ad.
    #[serde(default)]
    pub id: String,
    /// The URL of the post's media — the image for image posts, the playable video file for video posts. Null for TikTok, which exposes no raw file (use embed_url). Meta URLs are signed and expire after roughly 24 hours, so don't store them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    #[serde(default)]
    pub restrictions: Vec<SocialAccountPostRestrictionsItem>,
    /// Poster image for video posts (always set for TikTok, which is video-only); null for image posts, where media_url is already the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
}

impl SocialAccountPost {
    pub fn builder() -> SocialAccountPostBuilder {
        <SocialAccountPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SocialAccountPostBuilder {
    call_to_action: Option<SocialAccountPostCallToAction>,
    destination_url: Option<String>,
    embed_url: Option<String>,
    id: Option<String>,
    media_url: Option<String>,
    restrictions: Option<Vec<SocialAccountPostRestrictionsItem>>,
    thumbnail_url: Option<String>,
}

impl SocialAccountPostBuilder {
    pub fn call_to_action(mut self, value: SocialAccountPostCallToAction) -> Self {
        self.call_to_action = Some(value);
        self
    }

    pub fn destination_url(mut self, value: impl Into<String>) -> Self {
        self.destination_url = Some(value.into());
        self
    }

    pub fn embed_url(mut self, value: impl Into<String>) -> Self {
        self.embed_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn media_url(mut self, value: impl Into<String>) -> Self {
        self.media_url = Some(value.into());
        self
    }

    pub fn restrictions(mut self, value: Vec<SocialAccountPostRestrictionsItem>) -> Self {
        self.restrictions = Some(value);
        self
    }

    pub fn thumbnail_url(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SocialAccountPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SocialAccountPostBuilder::id)
    /// - [`restrictions`](SocialAccountPostBuilder::restrictions)
    pub fn build(self) -> Result<SocialAccountPost, BuildError> {
        Ok(SocialAccountPost {
            call_to_action: self.call_to_action,
            destination_url: self.destination_url,
            embed_url: self.embed_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            media_url: self.media_url,
            restrictions: self
                .restrictions
                .ok_or_else(|| BuildError::missing_field("restrictions"))?,
            thumbnail_url: self.thumbnail_url,
        })
    }
}
