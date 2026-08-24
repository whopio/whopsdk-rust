pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayoutsMethodsListQueryRequest {
    /// The owning account ID (a biz_ identifier). Provide this or user_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The owning user ID (a user_ identifier). Provide this or account_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Optional status filter. `created` means saved but unused, `active` means a payout through it succeeded, `broken` means the last payout failed and the method needs fixing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListMethodsRequestStatus>,
    /// Optional withdrawal amount in whole currency units, for example `250.00`. When provided, each method includes a quote with the estimated fee, amount received, and delivery date for that amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// Currency code of the amount, for example `usd`. Only meaningful with amount or include_limits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// When true, the response also carries limits — the live per-speed payout caps the account's payout requests are validated against, in the requested currency. Requires the payout:withdrawal:read scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_limits: Option<bool>,
    /// Number of payout methods to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to fetch the page after (from page_info.end_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of payout methods to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to fetch the page before (from page_info.start_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl PayoutsMethodsListQueryRequest {
    pub fn builder() -> PayoutsMethodsListQueryRequestBuilder {
        <PayoutsMethodsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutsMethodsListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
    status: Option<ListMethodsRequestStatus>,
    amount: Option<f64>,
    currency: Option<String>,
    include_limits: Option<bool>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl PayoutsMethodsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListMethodsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn include_limits(mut self, value: bool) -> Self {
        self.include_limits = Some(value);
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

    /// Consumes the builder and constructs a [`PayoutsMethodsListQueryRequest`].
    pub fn build(self) -> Result<PayoutsMethodsListQueryRequest, BuildError> {
        Ok(PayoutsMethodsListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
            status: self.status,
            amount: self.amount,
            currency: self.currency,
            include_limits: self.include_limits,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
