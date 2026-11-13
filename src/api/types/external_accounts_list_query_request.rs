pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExternalAccountsListQueryRequest {
    /// The Account that the external accounts are connected to. Provide either this or user_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The User that the external accounts are connected to. Provide either this or account_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Only return external accounts for the platform that is specified. An `aimerce` account is returned only with this filter, which requires the `company:manage_checkout` scope instead of `social_account:read`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<ListExternalAccountsRequestPlatform>,
    /// Only return external accounts linked with this trust level, such as `oauth` for accounts connected through OAuth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_level: Option<ListExternalAccountsRequestTrustLevel>,
    /// Only return external accounts that are verified on the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// Only return external accounts that have these scopes.
    #[serde(default)]
    pub scopes: Vec<Option<ListExternalAccountsRequestScopesItem>>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The field to sort external accounts by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListExternalAccountsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListExternalAccountsRequestDirection>,
}

impl ExternalAccountsListQueryRequest {
    pub fn builder() -> ExternalAccountsListQueryRequestBuilder {
        <ExternalAccountsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalAccountsListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
    platform: Option<ListExternalAccountsRequestPlatform>,
    trust_level: Option<ListExternalAccountsRequestTrustLevel>,
    verified: Option<bool>,
    scopes: Option<Vec<Option<ListExternalAccountsRequestScopesItem>>>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListExternalAccountsRequestOrder>,
    direction: Option<ListExternalAccountsRequestDirection>,
}

impl ExternalAccountsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn platform(mut self, value: ListExternalAccountsRequestPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn trust_level(mut self, value: ListExternalAccountsRequestTrustLevel) -> Self {
        self.trust_level = Some(value);
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    pub fn scopes(mut self, value: Vec<Option<ListExternalAccountsRequestScopesItem>>) -> Self {
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

    pub fn order(mut self, value: ListExternalAccountsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListExternalAccountsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExternalAccountsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scopes`](ExternalAccountsListQueryRequestBuilder::scopes)
    pub fn build(self) -> Result<ExternalAccountsListQueryRequest, BuildError> {
        Ok(ExternalAccountsListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
            platform: self.platform,
            trust_level: self.trust_level,
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
