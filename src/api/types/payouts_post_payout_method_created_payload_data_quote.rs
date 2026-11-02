pub use crate::prelude::*;

/// Fee and delivery estimate for paying out the requested amount through this method. Null unless an amount was provided, or when the estimate is unavailable.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostPayoutMethodCreatedPayloadDataQuote {
    /// The payout amount the quote is for.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// Currency of the quoted amount.
    #[serde(default)]
    pub currency: String,
    /// Exchange rate from the payout currency to the destination currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub exchange_rate: f64,
    /// Instant-delivery estimate. Null if the method does not support instant delivery, instant delivery is unavailable for the account, or the amount does not cover the fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant: Option<PostPayoutMethodCreatedPayloadDataQuoteInstant>,
    /// Why instant delivery is unavailable for this method. `minimum_crypto_sales_not_met` means the account has not reached the total sales required for instant cryptocurrency payouts. `null` when this restriction does not apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_unavailable_reason:
        Option<PostPayoutMethodCreatedPayloadDataQuoteInstantUnavailableReason>,
    /// Maximum payout amount for this method, in the payout currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_limit: Option<f64>,
    /// Minimum payout amount for this method, in the payout currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub min_limit: f64,
    /// Standard-delivery estimate. Null if the method does not support standard delivery, or the amount does not cover the fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<PostPayoutMethodCreatedPayloadDataQuoteStandard>,
}

impl PostPayoutMethodCreatedPayloadDataQuote {
    pub fn builder() -> PostPayoutMethodCreatedPayloadDataQuoteBuilder {
        <PostPayoutMethodCreatedPayloadDataQuoteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostPayoutMethodCreatedPayloadDataQuoteBuilder {
    amount: Option<f64>,
    currency: Option<String>,
    exchange_rate: Option<f64>,
    instant: Option<PostPayoutMethodCreatedPayloadDataQuoteInstant>,
    instant_unavailable_reason:
        Option<PostPayoutMethodCreatedPayloadDataQuoteInstantUnavailableReason>,
    max_limit: Option<f64>,
    min_limit: Option<f64>,
    standard: Option<PostPayoutMethodCreatedPayloadDataQuoteStandard>,
}

impl PostPayoutMethodCreatedPayloadDataQuoteBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn exchange_rate(mut self, value: f64) -> Self {
        self.exchange_rate = Some(value);
        self
    }

    pub fn instant(mut self, value: PostPayoutMethodCreatedPayloadDataQuoteInstant) -> Self {
        self.instant = Some(value);
        self
    }

    pub fn instant_unavailable_reason(
        mut self,
        value: PostPayoutMethodCreatedPayloadDataQuoteInstantUnavailableReason,
    ) -> Self {
        self.instant_unavailable_reason = Some(value);
        self
    }

    pub fn max_limit(mut self, value: f64) -> Self {
        self.max_limit = Some(value);
        self
    }

    pub fn min_limit(mut self, value: f64) -> Self {
        self.min_limit = Some(value);
        self
    }

    pub fn standard(mut self, value: PostPayoutMethodCreatedPayloadDataQuoteStandard) -> Self {
        self.standard = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostPayoutMethodCreatedPayloadDataQuote`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PostPayoutMethodCreatedPayloadDataQuoteBuilder::amount)
    /// - [`currency`](PostPayoutMethodCreatedPayloadDataQuoteBuilder::currency)
    /// - [`exchange_rate`](PostPayoutMethodCreatedPayloadDataQuoteBuilder::exchange_rate)
    /// - [`min_limit`](PostPayoutMethodCreatedPayloadDataQuoteBuilder::min_limit)
    pub fn build(self) -> Result<PostPayoutMethodCreatedPayloadDataQuote, BuildError> {
        Ok(PostPayoutMethodCreatedPayloadDataQuote {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            exchange_rate: self
                .exchange_rate
                .ok_or_else(|| BuildError::missing_field("exchange_rate"))?,
            instant: self.instant,
            instant_unavailable_reason: self.instant_unavailable_reason,
            max_limit: self.max_limit,
            min_limit: self
                .min_limit
                .ok_or_else(|| BuildError::missing_field("min_limit"))?,
            standard: self.standard,
        })
    }
}
