pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConnectSocialAccountsRequest {
    /// The Account (biz_ identifier) to connect the external account for. An account-scoped API key may omit this to default to its own account. Omit for user profile connections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The platform to connect the external account on. Use `tiktok` to connect the authenticated user’s TikTok profile, `linkedin` for their LinkedIn profile, `youtube` for their YouTube channel, or `x` for their X profile. Use `quickbooks` to connect the account’s QuickBooks Online company: pass `account_id`, and the user picks the company in QuickBooks.
    pub platform: ConnectSocialAccountsRequestPlatform,
    /// Where to send the user once they finish connecting their accounts. Must be an `https` URL on whop.com; any other host is rejected. On success, the user is redirected with an `external_account_connected` query param naming the platform; if the connection fails, with an `external_account_error` query param instead. The same values also arrive as `social_account_connected` / `social_account_error`.
    #[serde(default)]
    pub redirect_url: String,
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
        })
    }
}
