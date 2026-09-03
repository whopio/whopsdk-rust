pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateQuotePayoutsRequest {
    /// Account to pay out from, prefixed `biz_`. Provide exactly one of `account_id` or `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The amount to pay out in the specified currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The currency to pay out. When omitted, uses `usd` if that balance can cover a withdrawal, otherwise the account's only other funded currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The saved payout method to quote (a potk_ identifier).
    #[serde(default)]
    pub payout_method_id: String,
    /// Whether the parent platform covers the payout fee instead of the account being paid out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_covers_fees: Option<bool>,
    /// How fast the funds should arrive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<CreateQuotePayoutsRequestSpeed>,
    /// Text that appears on the recipient's bank statement. Must be 5-22 alphanumeric characters (A-Z, a-z, 0-9). Omit or pass `null` to use the default descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    /// User to pay out from, prefixed `user_`. Provide exactly one of `account_id` or `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CreateQuotePayoutsRequest {
    pub fn builder() -> CreateQuotePayoutsRequestBuilder {
        <CreateQuotePayoutsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateQuotePayoutsRequestBuilder {
    account_id: Option<String>,
    amount: Option<f64>,
    currency: Option<String>,
    payout_method_id: Option<String>,
    platform_covers_fees: Option<bool>,
    speed: Option<CreateQuotePayoutsRequestSpeed>,
    statement_descriptor: Option<String>,
    user_id: Option<String>,
}

impl CreateQuotePayoutsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
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

    pub fn payout_method_id(mut self, value: impl Into<String>) -> Self {
        self.payout_method_id = Some(value.into());
        self
    }

    pub fn platform_covers_fees(mut self, value: bool) -> Self {
        self.platform_covers_fees = Some(value);
        self
    }

    pub fn speed(mut self, value: CreateQuotePayoutsRequestSpeed) -> Self {
        self.speed = Some(value);
        self
    }

    pub fn statement_descriptor(mut self, value: impl Into<String>) -> Self {
        self.statement_descriptor = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateQuotePayoutsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreateQuotePayoutsRequestBuilder::amount)
    /// - [`payout_method_id`](CreateQuotePayoutsRequestBuilder::payout_method_id)
    pub fn build(self) -> Result<CreateQuotePayoutsRequest, BuildError> {
        Ok(CreateQuotePayoutsRequest {
            account_id: self.account_id,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self.currency,
            payout_method_id: self
                .payout_method_id
                .ok_or_else(|| BuildError::missing_field("payout_method_id"))?,
            platform_covers_fees: self.platform_covers_fees,
            speed: self.speed,
            statement_descriptor: self.statement_descriptor,
            user_id: self.user_id,
        })
    }
}
