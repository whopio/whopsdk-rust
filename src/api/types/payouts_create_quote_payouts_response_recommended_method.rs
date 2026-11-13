pub use crate::prelude::*;

/// An optional payout method to connect for estimated savings. The quote still uses the requested saved method.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateQuotePayoutsResponseRecommendedMethod {
    /// Two-letter ISO country code for connecting the method.
    #[serde(default)]
    pub country: String,
    /// Currency to select when connecting the method.
    #[serde(default)]
    pub destination_currency: String,
    /// Estimated arrival when paying out with the suggested method, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub estimated_arrival: Option<DateTime<FixedOffset>>,
    /// Estimated total cost, including same-currency exchange-rate loss.
    #[serde(default)]
    pub estimated_fee: Money,
    /// Estimated increase in the delivered amount after fees compared with this quote.
    #[serde(default)]
    pub estimated_savings: Money,
    /// Display name of the suggested payout method.
    #[serde(default)]
    pub name: String,
    /// Supported payout method to connect, prefixed `podst_`.
    #[serde(default)]
    pub supported_payout_method_id: String,
}

impl CreateQuotePayoutsResponseRecommendedMethod {
    pub fn builder() -> CreateQuotePayoutsResponseRecommendedMethodBuilder {
        <CreateQuotePayoutsResponseRecommendedMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateQuotePayoutsResponseRecommendedMethodBuilder {
    country: Option<String>,
    destination_currency: Option<String>,
    estimated_arrival: Option<DateTime<FixedOffset>>,
    estimated_fee: Option<Money>,
    estimated_savings: Option<Money>,
    name: Option<String>,
    supported_payout_method_id: Option<String>,
}

impl CreateQuotePayoutsResponseRecommendedMethodBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn destination_currency(mut self, value: impl Into<String>) -> Self {
        self.destination_currency = Some(value.into());
        self
    }

    pub fn estimated_arrival(mut self, value: DateTime<FixedOffset>) -> Self {
        self.estimated_arrival = Some(value);
        self
    }

    pub fn estimated_fee(mut self, value: Money) -> Self {
        self.estimated_fee = Some(value);
        self
    }

    pub fn estimated_savings(mut self, value: Money) -> Self {
        self.estimated_savings = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn supported_payout_method_id(mut self, value: impl Into<String>) -> Self {
        self.supported_payout_method_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateQuotePayoutsResponseRecommendedMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](CreateQuotePayoutsResponseRecommendedMethodBuilder::country)
    /// - [`destination_currency`](CreateQuotePayoutsResponseRecommendedMethodBuilder::destination_currency)
    /// - [`estimated_fee`](CreateQuotePayoutsResponseRecommendedMethodBuilder::estimated_fee)
    /// - [`estimated_savings`](CreateQuotePayoutsResponseRecommendedMethodBuilder::estimated_savings)
    /// - [`name`](CreateQuotePayoutsResponseRecommendedMethodBuilder::name)
    /// - [`supported_payout_method_id`](CreateQuotePayoutsResponseRecommendedMethodBuilder::supported_payout_method_id)
    pub fn build(self) -> Result<CreateQuotePayoutsResponseRecommendedMethod, BuildError> {
        Ok(CreateQuotePayoutsResponseRecommendedMethod {
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            destination_currency: self
                .destination_currency
                .ok_or_else(|| BuildError::missing_field("destination_currency"))?,
            estimated_arrival: self.estimated_arrival,
            estimated_fee: self
                .estimated_fee
                .ok_or_else(|| BuildError::missing_field("estimated_fee"))?,
            estimated_savings: self
                .estimated_savings
                .ok_or_else(|| BuildError::missing_field("estimated_savings"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            supported_payout_method_id: self
                .supported_payout_method_id
                .ok_or_else(|| BuildError::missing_field("supported_payout_method_id"))?,
        })
    }
}
