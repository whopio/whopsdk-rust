pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutsListQueryRequest {
    /// The owning account ID (a biz_ identifier). Provide this or user_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The owning user ID (a user_ identifier). Provide this or account_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Optional currency code filter, for example `usd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Filter to payouts whose `status` reads this word, matching exactly what this version displays — `reversed` finds settled payouts the bank later returned. Requires Api-Version-Date 2026-08-21 or later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPayoutsRequestStatus>,
    /// Filter by how the payout was created. Payouts created before source tracking or through internal tooling carry no source and never match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ListPayoutsRequestSource>,
    /// Filter to payouts sent to one saved payout method (a pytk_ identifier). An unknown id matches nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_method_id: Option<String>,
    /// Only payouts created before this ISO 8601 time (exclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only payouts created at or after this ISO 8601 time (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// Number of payouts to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to fetch the page after (from page_info.end_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of payouts to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to fetch the page before (from page_info.start_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl PayoutsListQueryRequest {
    pub fn builder() -> PayoutsListQueryRequestBuilder {
        <PayoutsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutsListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
    currency: Option<String>,
    status: Option<ListPayoutsRequestStatus>,
    source: Option<ListPayoutsRequestSource>,
    payout_method_id: Option<String>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl PayoutsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPayoutsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn source(mut self, value: ListPayoutsRequestSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn payout_method_id(mut self, value: impl Into<String>) -> Self {
        self.payout_method_id = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
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

    /// Consumes the builder and constructs a [`PayoutsListQueryRequest`].
    pub fn build(self) -> Result<PayoutsListQueryRequest, BuildError> {
        Ok(PayoutsListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
            currency: self.currency,
            status: self.status,
            source: self.source,
            payout_method_id: self.payout_method_id,
            created_before: self.created_before,
            created_after: self.created_after,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
