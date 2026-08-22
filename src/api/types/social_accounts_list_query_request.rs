pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SocialAccountsListQueryRequest {
    /// The Account that the social accounts are connected to. Provide either this or user_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The User that the social accounts are connected to. Provide either this or account_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Only return social accounts for the platform that is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<ListSocialAccountsRequestPlatform>,
    /// Only return social accounts that are verified on the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// Only return social accounts that have these scopes.
    #[serde(default)]
    pub scopes: Vec<Option<ListSocialAccountsRequestScopesItem>>,
    /// The number of social accounts to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to fetch the page after (from page_info.end_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of social accounts to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to fetch the page before (from page_info.start_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The field to sort social accounts by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListSocialAccountsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListSocialAccountsRequestDirection>,
}

impl SocialAccountsListQueryRequest {
    pub fn builder() -> SocialAccountsListQueryRequestBuilder {
        <SocialAccountsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SocialAccountsListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
    platform: Option<ListSocialAccountsRequestPlatform>,
    verified: Option<bool>,
    scopes: Option<Vec<Option<ListSocialAccountsRequestScopesItem>>>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListSocialAccountsRequestOrder>,
    direction: Option<ListSocialAccountsRequestDirection>,
}

impl SocialAccountsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn platform(mut self, value: ListSocialAccountsRequestPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    pub fn scopes(mut self, value: Vec<Option<ListSocialAccountsRequestScopesItem>>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListSocialAccountsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListSocialAccountsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SocialAccountsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scopes`](SocialAccountsListQueryRequestBuilder::scopes)
    pub fn build(self) -> Result<SocialAccountsListQueryRequest, BuildError> {
        Ok(SocialAccountsListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
            platform: self.platform,
            verified: self.verified,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
        })
    }
}
