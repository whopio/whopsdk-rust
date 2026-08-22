pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayoutsSupportedMethodsListQueryRequest {
    /// The owning account ID (a biz_ identifier). Provide this or user_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The owning user ID (a user_ identifier). Provide this or account_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// ISO 3166-1 alpha-2 country code for the bank account or wallet, such as `US`. Defaults to the country of supported_payout_method_id when one is given, otherwise the payout account's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Optional withdrawal amount in whole currency units, for example `250.00`. When provided, each destination includes per-currency fee and delivery quotes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// Currency code of the amount, for example `usd`. Only meaningful with amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Narrows the list to one supported payout method (a podst_ identifier) and includes the required_fields needed to save it as a payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_payout_method_id: Option<String>,
    /// Currency the supported payout method would deliver payouts in. Only meaningful with supported_payout_method_id; required fields vary by destination currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_currency: Option<String>,
    /// Number of supported payout methods to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to fetch the page after (from page_info.end_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of supported payout methods to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to fetch the page before (from page_info.start_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl PayoutsSupportedMethodsListQueryRequest {
    pub fn builder() -> PayoutsSupportedMethodsListQueryRequestBuilder {
        <PayoutsSupportedMethodsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutsSupportedMethodsListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
    country: Option<String>,
    amount: Option<f64>,
    currency: Option<String>,
    supported_payout_method_id: Option<String>,
    destination_currency: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl PayoutsSupportedMethodsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
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

    pub fn supported_payout_method_id(mut self, value: impl Into<String>) -> Self {
        self.supported_payout_method_id = Some(value.into());
        self
    }

    pub fn destination_currency(mut self, value: impl Into<String>) -> Self {
        self.destination_currency = Some(value.into());
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

    /// Consumes the builder and constructs a [`PayoutsSupportedMethodsListQueryRequest`].
    pub fn build(self) -> Result<PayoutsSupportedMethodsListQueryRequest, BuildError> {
        Ok(PayoutsSupportedMethodsListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
            country: self.country,
            amount: self.amount,
            currency: self.currency,
            supported_payout_method_id: self.supported_payout_method_id,
            destination_currency: self.destination_currency,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
