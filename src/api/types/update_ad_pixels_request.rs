pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdPixelsRequest {
    /// A credential that can send events to the pixel on the ad platform. For `meta`, a Conversions API access token. For `tiktok`, an Events API access token. For `thrad`, an API key with `events:write` for the tag. For `google`, the JSON key of a Google Cloud service account. Give the service account Standard access to the Google Ads account. In its project, turn on the Google Ads API and the Data Manager API. The project also needs Explorer access or higher to the Google Ads API. Use a service account made only for Whop, with no roles in its project. Whop never returns it.
    #[serde(default)]
    pub access_token: String,
}

impl UpdateAdPixelsRequest {
    pub fn builder() -> UpdateAdPixelsRequestBuilder {
        <UpdateAdPixelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdPixelsRequestBuilder {
    access_token: Option<String>,
}

impl UpdateAdPixelsRequestBuilder {
    pub fn access_token(mut self, value: impl Into<String>) -> Self {
        self.access_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdPixelsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_token`](UpdateAdPixelsRequestBuilder::access_token)
    pub fn build(self) -> Result<UpdateAdPixelsRequest, BuildError> {
        Ok(UpdateAdPixelsRequest {
            access_token: self
                .access_token
                .ok_or_else(|| BuildError::missing_field("access_token"))?,
        })
    }
}
