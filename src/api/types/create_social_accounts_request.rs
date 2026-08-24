pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateSocialAccountsRequest {
    /// The Account (biz_ identifier) to create the social account for. An account-scoped API key may omit this to default to its own account. Account API keys cannot update their own account's branding through Update Account; use a user-authenticated path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The platform to create the social account on. `facebook` requires the account's `banner_image`, `logo`, and `description`; configure them with [Update Account](/api-reference/beta/accounts/update-account).
    pub platform: CreateSocialAccountsRequestPlatform,
}

impl CreateSocialAccountsRequest {
    pub fn builder() -> CreateSocialAccountsRequestBuilder {
        <CreateSocialAccountsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSocialAccountsRequestBuilder {
    account_id: Option<String>,
    platform: Option<CreateSocialAccountsRequestPlatform>,
}

impl CreateSocialAccountsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn platform(mut self, value: CreateSocialAccountsRequestPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSocialAccountsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](CreateSocialAccountsRequestBuilder::platform)
    pub fn build(self) -> Result<CreateSocialAccountsRequest, BuildError> {
        Ok(CreateSocialAccountsRequest {
            account_id: self.account_id,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
        })
    }
}
