pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FinancialActivityListQueryRequest {
    /// The owning account ID (a biz_ identifier). Provide this or user_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The owning user ID (a user_ identifier). Provide this or account_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// When true, aggregates the authenticated user's personal ledger with the businesses they own (owner role with balance read) into one feed. Requires user_id to be the authenticated user; cannot be combined with account_id or the settlement-date filters. Each returned row includes the owning `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_owned_accounts: Option<bool>,
    /// Whether to include the `resource` field in the response or not. Consider passing `false` if you need a fast response without as many rich details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resource: Option<bool>,
    /// Optional ledger line categories to include. Some categories (for example `onchain_deposit`, which covers inbound crypto deposits such as MoonPay onramps) are only returned when explicitly requested here.
    #[serde(default)]
    pub line_types: Vec<Option<ListFinancialActivityRequestLineTypesItem>>,
    /// Optional direction filter. `money_in` returns positive activity and `money_out` returns negative activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListFinancialActivityRequestDirection>,
    /// Optional prefixed resource ID. Returns activity associated with that resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Optional ledger activity ID (for example `line_3`). Returns at most that one activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// Optional currency code filter, for example `usd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Only include rows posted after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub posted_after: Option<DateTime<FixedOffset>>,
    /// Only include rows posted before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub posted_before: Option<DateTime<FixedOffset>>,
    /// Only include rows whose funds became withdrawable on or after this `YYYY-MM-DD` settlement date (UTC), distinct from posted_at. Requires currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_after: Option<NaiveDate>,
    /// Only include rows whose funds became withdrawable on or before this `YYYY-MM-DD` settlement date (UTC). Set equal to available_after for a single day. Requires currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_before: Option<NaiveDate>,
    /// Maximum number of rows to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor returned by the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl FinancialActivityListQueryRequest {
    pub fn builder() -> FinancialActivityListQueryRequestBuilder {
        <FinancialActivityListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancialActivityListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
    include_owned_accounts: Option<bool>,
    include_resource: Option<bool>,
    line_types: Option<Vec<Option<ListFinancialActivityRequestLineTypesItem>>>,
    direction: Option<ListFinancialActivityRequestDirection>,
    resource_id: Option<String>,
    activity_id: Option<String>,
    currency: Option<String>,
    posted_after: Option<DateTime<FixedOffset>>,
    posted_before: Option<DateTime<FixedOffset>>,
    available_after: Option<NaiveDate>,
    available_before: Option<NaiveDate>,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl FinancialActivityListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn include_owned_accounts(mut self, value: bool) -> Self {
        self.include_owned_accounts = Some(value);
        self
    }

    pub fn include_resource(mut self, value: bool) -> Self {
        self.include_resource = Some(value);
        self
    }

    pub fn line_types(
        mut self,
        value: Vec<Option<ListFinancialActivityRequestLineTypesItem>>,
    ) -> Self {
        self.line_types = Some(value);
        self
    }

    pub fn direction(mut self, value: ListFinancialActivityRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn activity_id(mut self, value: impl Into<String>) -> Self {
        self.activity_id = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn posted_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.posted_after = Some(value);
        self
    }

    pub fn posted_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.posted_before = Some(value);
        self
    }

    pub fn available_after(mut self, value: NaiveDate) -> Self {
        self.available_after = Some(value);
        self
    }

    pub fn available_before(mut self, value: NaiveDate) -> Self {
        self.available_before = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FinancialActivityListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line_types`](FinancialActivityListQueryRequestBuilder::line_types)
    pub fn build(self) -> Result<FinancialActivityListQueryRequest, BuildError> {
        Ok(FinancialActivityListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
            include_owned_accounts: self.include_owned_accounts,
            include_resource: self.include_resource,
            line_types: self
                .line_types
                .ok_or_else(|| BuildError::missing_field("line_types"))?,
            direction: self.direction,
            resource_id: self.resource_id,
            activity_id: self.activity_id,
            currency: self.currency,
            posted_after: self.posted_after,
            posted_before: self.posted_before,
            available_after: self.available_after,
            available_before: self.available_before,
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
