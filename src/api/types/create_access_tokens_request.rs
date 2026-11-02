pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAccessTokensRequest {
    /// The unique identifier of the company to generate the token for, starting with 'biz_'. The API key must have permission to access this company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The expiration timestamp for the access token. Defaults to 1 hour from now, with a maximum of 3 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// An array of permission scopes to grant to the access token. If empty or omitted, all permissions from the authenticating credential are inherited. Must be a subset of the credential's permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoped_actions: Option<Vec<String>>,
    /// The unique identifier of the user to generate the token for, starting with 'user_'. The API key must have permission to access this user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CreateAccessTokensRequest {
    pub fn builder() -> CreateAccessTokensRequestBuilder {
        <CreateAccessTokensRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAccessTokensRequestBuilder {
    account_id: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    scoped_actions: Option<Vec<String>>,
    user_id: Option<String>,
}

impl CreateAccessTokensRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn scoped_actions(mut self, value: Vec<String>) -> Self {
        self.scoped_actions = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAccessTokensRequest`].
    pub fn build(self) -> Result<CreateAccessTokensRequest, BuildError> {
        Ok(CreateAccessTokensRequest {
            account_id: self.account_id,
            expires_at: self.expires_at,
            scoped_actions: self.scoped_actions,
            user_id: self.user_id,
        })
    }
}
