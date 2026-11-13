pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAdPixelsRequest {
    /// A credential that can send events to the pixel on the ad platform. For `meta`, a Conversions API access token. For `tiktok`, an Events API access token. For `thrad`, an API key with `events:write` for the tag. For `google`, the JSON key of a Google Cloud service account. Give the service account Standard access to the Google Ads account. In its project, turn on the Google Ads API and the Data Manager API. The project also needs Explorer access or higher to the Google Ads API. Use a service account made only for Whop, with no roles in its project. Whop never returns it.
    #[serde(default)]
    pub access_token: String,
    /// The business whose claimed campaigns send conversions to this pixel, prefixed `biz_`. You must have claimed at least one of its campaigns.
    #[serde(default)]
    pub account_id: String,
    /// The pixel's ID on the ad platform: a Meta pixel or dataset ID, TikTok pixel code, Google Ads customer ID, or Thrad tag ID.
    #[serde(default)]
    pub external_id: String,
    /// The ad platform the pixel belongs to.
    pub platform: CreateAdPixelsRequestPlatform,
}

impl CreateAdPixelsRequest {
    pub fn builder() -> CreateAdPixelsRequestBuilder {
        <CreateAdPixelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdPixelsRequestBuilder {
    access_token: Option<String>,
    account_id: Option<String>,
    external_id: Option<String>,
    platform: Option<CreateAdPixelsRequestPlatform>,
}

impl CreateAdPixelsRequestBuilder {
    pub fn access_token(mut self, value: impl Into<String>) -> Self {
        self.access_token = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn platform(mut self, value: CreateAdPixelsRequestPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAdPixelsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_token`](CreateAdPixelsRequestBuilder::access_token)
    /// - [`account_id`](CreateAdPixelsRequestBuilder::account_id)
    /// - [`external_id`](CreateAdPixelsRequestBuilder::external_id)
    /// - [`platform`](CreateAdPixelsRequestBuilder::platform)
    pub fn build(self) -> Result<CreateAdPixelsRequest, BuildError> {
        Ok(CreateAdPixelsRequest {
            access_token: self
                .access_token
                .ok_or_else(|| BuildError::missing_field("access_token"))?,
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
        })
    }
}
