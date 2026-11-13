pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WaitlistEntriesListQueryRequest {
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
    /// Only return signups for this variant, prefixed `plan_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
    /// Only return signups submitted to this seller account, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only return signups for variants on this product, prefixed `prod_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// List the signups submitted on behalf of this account, prefixed `biz_`, instead of your own and seller signups. Requires `waitlist_entry:read` on the account. The other filters still apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_account_id: Option<String>,
    /// Only return signups in this state. Canceled signups are returned only when `status` is `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListWaitlistEntriesRequestStatus>,
    /// Search signups by the user's name or username, or the name of the account they were submitted on behalf of. An account ID, prefixed `biz_`, matches signups submitted on behalf of that account, and a user ID, prefixed `user_`, or an exact email address matches that user's signups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only return signups submitted at or before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only return signups submitted at or after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// The field to sort results by. Defaults to `created_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListWaitlistEntriesRequestOrder>,
    /// The sort direction for results. Defaults to descending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListWaitlistEntriesRequestDirection>,
}

impl WaitlistEntriesListQueryRequest {
    pub fn builder() -> WaitlistEntriesListQueryRequestBuilder {
        <WaitlistEntriesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaitlistEntriesListQueryRequestBuilder {
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    variant_id: Option<String>,
    account_id: Option<String>,
    product_id: Option<String>,
    buyer_account_id: Option<String>,
    status: Option<ListWaitlistEntriesRequestStatus>,
    query: Option<String>,
    created_before: Option<String>,
    created_after: Option<String>,
    order: Option<ListWaitlistEntriesRequestOrder>,
    direction: Option<ListWaitlistEntriesRequestDirection>,
}

impl WaitlistEntriesListQueryRequestBuilder {
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

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn buyer_account_id(mut self, value: impl Into<String>) -> Self {
        self.buyer_account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListWaitlistEntriesRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListWaitlistEntriesRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListWaitlistEntriesRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WaitlistEntriesListQueryRequest`].
    pub fn build(self) -> Result<WaitlistEntriesListQueryRequest, BuildError> {
        Ok(WaitlistEntriesListQueryRequest {
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            variant_id: self.variant_id,
            account_id: self.account_id,
            product_id: self.product_id,
            buyer_account_id: self.buyer_account_id,
            status: self.status,
            query: self.query,
            created_before: self.created_before,
            created_after: self.created_after,
            order: self.order,
            direction: self.direction,
        })
    }
}
