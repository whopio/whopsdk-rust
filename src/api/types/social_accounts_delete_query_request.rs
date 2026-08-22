pub use crate::prelude::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SocialAccountsDeleteQueryRequest {
    /// The Account that the social account is connected to. Provide either this or user_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The User that the social account is connected to. Provide either this or account_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl SocialAccountsDeleteQueryRequest {
    pub fn builder() -> SocialAccountsDeleteQueryRequestBuilder {
        <SocialAccountsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SocialAccountsDeleteQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
}

impl SocialAccountsDeleteQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SocialAccountsDeleteQueryRequest`].
    pub fn build(self) -> Result<SocialAccountsDeleteQueryRequest, BuildError> {
        Ok(SocialAccountsDeleteQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
        })
    }
}
