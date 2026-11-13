pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdPixel {
    /// The business whose claimed campaigns send conversions to this pixel, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// When the pixel was added, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Why the ad platform rejected the last delivery. Null unless `status` is `errored`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The pixel's ID on the ad platform: a Meta pixel or dataset ID, TikTok pixel code, Google Ads customer ID, or Thrad tag ID.
    #[serde(default)]
    pub external_id: String,
    /// Ad pixel ID, prefixed `adpx_`.
    #[serde(default)]
    pub id: String,
    /// The pixel's name on the ad platform. Null when the ad platform returned none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ad platform the pixel belongs to.
    pub platform: AdPixelPlatform,
    /// `active` receives conversions. `errored` stopped because the ad platform rejected the access token; update the token to resume. `disabled` was turned off by Whop.
    pub status: AdPixelStatus,
    /// When the pixel was last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl AdPixel {
    pub fn builder() -> AdPixelBuilder {
        <AdPixelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdPixelBuilder {
    account_id: Option<String>,
    created_at: Option<String>,
    error_message: Option<String>,
    external_id: Option<String>,
    id: Option<String>,
    name: Option<String>,
    platform: Option<AdPixelPlatform>,
    status: Option<AdPixelStatus>,
    updated_at: Option<String>,
}

impl AdPixelBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn platform(mut self, value: AdPixelPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn status(mut self, value: AdPixelStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdPixel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](AdPixelBuilder::account_id)
    /// - [`created_at`](AdPixelBuilder::created_at)
    /// - [`external_id`](AdPixelBuilder::external_id)
    /// - [`id`](AdPixelBuilder::id)
    /// - [`platform`](AdPixelBuilder::platform)
    /// - [`status`](AdPixelBuilder::status)
    /// - [`updated_at`](AdPixelBuilder::updated_at)
    pub fn build(self) -> Result<AdPixel, BuildError> {
        Ok(AdPixel {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            error_message: self.error_message,
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
