pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefreshSocialAccountsRequest {
    /// The Account (biz_ identifier) the external account is connected to. An account-scoped API key may omit this to default to its own account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl RefreshSocialAccountsRequest {
    pub fn builder() -> RefreshSocialAccountsRequestBuilder {
        <RefreshSocialAccountsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefreshSocialAccountsRequestBuilder {
    account_id: Option<String>,
}

impl RefreshSocialAccountsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RefreshSocialAccountsRequest`].
    pub fn build(self) -> Result<RefreshSocialAccountsRequest, BuildError> {
        Ok(RefreshSocialAccountsRequest {
            account_id: self.account_id,
        })
    }
}
