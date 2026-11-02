pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConnectSocialAccountsRequest {
    /// The Account (biz_ identifier) to connect the social account for. An account-scoped API key may omit this to default to its own account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The platform to connect the social account on. Use `meta_business` to connect Meta Business assets, which is how Facebook Pages and Instagram accounts are connected — there is no separate `instagram` value. Use `tiktok` for TikTok accounts.
    pub platform: ConnectSocialAccountsRequestPlatform,
    /// Where to send the user once they finish connecting their accounts. Any `http` or `https` URL. If the connection fails, the user is redirected with a `social_account_error` query param.
    #[serde(default)]
    pub redirect_url: String,
    /// Capabilities to grant for the connected social account. `advertise` is required for both `meta_business` and `tiktok` connections — it is not conditional on whether you intend to run ads, and omitting it fails the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ConnectSocialAccountsRequestScopesItem>>,
}

impl ConnectSocialAccountsRequest {
    pub fn builder() -> ConnectSocialAccountsRequestBuilder {
        <ConnectSocialAccountsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectSocialAccountsRequestBuilder {
    account_id: Option<String>,
    platform: Option<ConnectSocialAccountsRequestPlatform>,
    redirect_url: Option<String>,
    scopes: Option<Vec<ConnectSocialAccountsRequestScopesItem>>,
}

impl ConnectSocialAccountsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn platform(mut self, value: ConnectSocialAccountsRequestPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn redirect_url(mut self, value: impl Into<String>) -> Self {
        self.redirect_url = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<ConnectSocialAccountsRequestScopesItem>) -> Self {
        self.scopes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectSocialAccountsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](ConnectSocialAccountsRequestBuilder::platform)
    /// - [`redirect_url`](ConnectSocialAccountsRequestBuilder::redirect_url)
    pub fn build(self) -> Result<ConnectSocialAccountsRequest, BuildError> {
        Ok(ConnectSocialAccountsRequest {
            account_id: self.account_id,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            redirect_url: self
                .redirect_url
                .ok_or_else(|| BuildError::missing_field("redirect_url"))?,
            scopes: self.scopes,
        })
    }
}
