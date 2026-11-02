pub use crate::prelude::*;

/// Fee and delivery estimate for paying out the requested amount through this method. Null unless an amount was provided, or when the estimate is unavailable.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMethodsResponseDataItemQuote {
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
    pub instant: Option<ListMethodsResponseDataItemQuoteInstant>,
    /// Why instant delivery is unavailable for this method. `minimum_crypto_sales_not_met` means the account has not reached the total sales required for instant cryptocurrency payouts. `null` when this restriction does not apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_unavailable_reason:
        Option<ListMethodsResponseDataItemQuoteInstantUnavailableReason>,
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
    pub standard: Option<ListMethodsResponseDataItemQuoteStandard>,
}

impl ListMethodsResponseDataItemQuote {
    pub fn builder() -> ListMethodsResponseDataItemQuoteBuilder {
        <ListMethodsResponseDataItemQuoteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMethodsResponseDataItemQuoteBuilder {
    amount: Option<f64>,
    currency: Option<String>,
    exchange_rate: Option<f64>,
    instant: Option<ListMethodsResponseDataItemQuoteInstant>,
    instant_unavailable_reason: Option<ListMethodsResponseDataItemQuoteInstantUnavailableReason>,
    max_limit: Option<f64>,
    min_limit: Option<f64>,
    standard: Option<ListMethodsResponseDataItemQuoteStandard>,
}

impl ListMethodsResponseDataItemQuoteBuilder {
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

    pub fn instant(mut self, value: ListMethodsResponseDataItemQuoteInstant) -> Self {
        self.instant = Some(value);
        self
    }

    pub fn instant_unavailable_reason(
        mut self,
        value: ListMethodsResponseDataItemQuoteInstantUnavailableReason,
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

    pub fn standard(mut self, value: ListMethodsResponseDataItemQuoteStandard) -> Self {
        self.standard = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMethodsResponseDataItemQuote`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](ListMethodsResponseDataItemQuoteBuilder::amount)
    /// - [`currency`](ListMethodsResponseDataItemQuoteBuilder::currency)
    /// - [`exchange_rate`](ListMethodsResponseDataItemQuoteBuilder::exchange_rate)
    /// - [`min_limit`](ListMethodsResponseDataItemQuoteBuilder::min_limit)
    pub fn build(self) -> Result<ListMethodsResponseDataItemQuote, BuildError> {
        Ok(ListMethodsResponseDataItemQuote {
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
