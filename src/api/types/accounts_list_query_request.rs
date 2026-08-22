pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountsListQueryRequest {
    /// The number of accounts to return (default 10, max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns accounts after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of accounts to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns accounts before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The field to sort accounts by. `volume` requires `stats:read` on the parent account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListAccountsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListAccountsRequestDirection>,
    /// Return only accounts with this status: `active` (includes accounts that have not entered payments review) or `suspended`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListAccountsRequestStatus>,
    /// Free-text filter on account title or ID. `%` and `_` match literally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return only accounts created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// Return only accounts created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Return only accounts whose lifetime USD volume is at least this value. Requires `stats:read` on the parent account.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub volume_min: Option<f64>,
    /// Return only accounts whose lifetime USD volume is at most this value. Requires `stats:read` on the parent account.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub volume_max: Option<f64>,
    /// For platforms: the parent account ID whose direct connected accounts to return. Requires `payout:account:read` on the parent account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_account_id: Option<String>,
}

impl AccountsListQueryRequest {
    pub fn builder() -> AccountsListQueryRequestBuilder {
        <AccountsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsListQueryRequestBuilder {
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListAccountsRequestOrder>,
    direction: Option<ListAccountsRequestDirection>,
    status: Option<ListAccountsRequestStatus>,
    query: Option<String>,
    created_after: Option<DateTime<FixedOffset>>,
    created_before: Option<DateTime<FixedOffset>>,
    volume_min: Option<f64>,
    volume_max: Option<f64>,
    parent_account_id: Option<String>,
}

impl AccountsListQueryRequestBuilder {
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

    pub fn order(mut self, value: ListAccountsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListAccountsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn status(mut self, value: ListAccountsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn volume_min(mut self, value: f64) -> Self {
        self.volume_min = Some(value);
        self
    }

    pub fn volume_max(mut self, value: f64) -> Self {
        self.volume_max = Some(value);
        self
    }

    pub fn parent_account_id(mut self, value: impl Into<String>) -> Self {
        self.parent_account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountsListQueryRequest`].
    pub fn build(self) -> Result<AccountsListQueryRequest, BuildError> {
        Ok(AccountsListQueryRequest {
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
            status: self.status,
            query: self.query,
            created_after: self.created_after,
            created_before: self.created_before,
            volume_min: self.volume_min,
            volume_max: self.volume_max,
            parent_account_id: self.parent_account_id,
        })
    }
}
