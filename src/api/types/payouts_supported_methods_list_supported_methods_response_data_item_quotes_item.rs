pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSupportedMethodsResponseDataItemQuotesItem {
    /// The payout amount the quote is for.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// Currency of the quoted amount.
    #[serde(default)]
    pub currency: String,
    /// Currency the funds are delivered in.
    #[serde(default)]
    pub destination_currency: String,
    /// Exchange rate from the payout currency to the destination currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub exchange_rate: f64,
    /// Instant-delivery estimate. Null if unsupported, unavailable for the account, or the amount does not cover the fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant: Option<ListSupportedMethodsResponseDataItemQuotesItemInstant>,
    /// Maximum payout amount, in the payout currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_limit: Option<f64>,
    /// Minimum payout amount, in the payout currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub min_limit: f64,
    /// Standard-delivery estimate. Null if unsupported or the amount does not cover the fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<ListSupportedMethodsResponseDataItemQuotesItemStandard>,
}

impl ListSupportedMethodsResponseDataItemQuotesItem {
    pub fn builder() -> ListSupportedMethodsResponseDataItemQuotesItemBuilder {
        <ListSupportedMethodsResponseDataItemQuotesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSupportedMethodsResponseDataItemQuotesItemBuilder {
    amount: Option<f64>,
    currency: Option<String>,
    destination_currency: Option<String>,
    exchange_rate: Option<f64>,
    instant: Option<ListSupportedMethodsResponseDataItemQuotesItemInstant>,
    max_limit: Option<f64>,
    min_limit: Option<f64>,
    standard: Option<ListSupportedMethodsResponseDataItemQuotesItemStandard>,
}

impl ListSupportedMethodsResponseDataItemQuotesItemBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn destination_currency(mut self, value: impl Into<String>) -> Self {
        self.destination_currency = Some(value.into());
        self
    }

    pub fn exchange_rate(mut self, value: f64) -> Self {
        self.exchange_rate = Some(value);
        self
    }

    pub fn instant(mut self, value: ListSupportedMethodsResponseDataItemQuotesItemInstant) -> Self {
        self.instant = Some(value);
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

    pub fn standard(
        mut self,
        value: ListSupportedMethodsResponseDataItemQuotesItemStandard,
    ) -> Self {
        self.standard = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSupportedMethodsResponseDataItemQuotesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](ListSupportedMethodsResponseDataItemQuotesItemBuilder::amount)
    /// - [`currency`](ListSupportedMethodsResponseDataItemQuotesItemBuilder::currency)
    /// - [`destination_currency`](ListSupportedMethodsResponseDataItemQuotesItemBuilder::destination_currency)
    /// - [`exchange_rate`](ListSupportedMethodsResponseDataItemQuotesItemBuilder::exchange_rate)
    /// - [`min_limit`](ListSupportedMethodsResponseDataItemQuotesItemBuilder::min_limit)
    pub fn build(self) -> Result<ListSupportedMethodsResponseDataItemQuotesItem, BuildError> {
        Ok(ListSupportedMethodsResponseDataItemQuotesItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            destination_currency: self
                .destination_currency
                .ok_or_else(|| BuildError::missing_field("destination_currency"))?,
            exchange_rate: self
                .exchange_rate
                .ok_or_else(|| BuildError::missing_field("exchange_rate"))?,
            instant: self.instant,
            max_limit: self.max_limit,
            min_limit: self
                .min_limit
                .ok_or_else(|| BuildError::missing_field("min_limit"))?,
            standard: self.standard,
        })
    }
}
