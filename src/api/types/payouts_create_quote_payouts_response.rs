pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateQuotePayoutsResponse {
    /// Gross payout amount.
    #[serde(default)]
    pub amount: Money,
    /// How a USD balance pays a bank in another currency: it converts into the payout method's currency, and the payout leaves from the balance in that currency. Null when the payout is paid from the requested balance directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion: Option<CreateQuotePayoutsResponseConversion>,
    /// Exact amount quoted for delivery.
    #[serde(default)]
    pub destination_amount: Money,
    /// Quoted exchange rate from the source currency to the destination currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub exchange_rate: f64,
    /// When the quote expires.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    /// Fee charged for the payout.
    #[serde(default)]
    pub fee: Money,
    /// Provider-backed payout quote ID, prefixed `pout_`.
    #[serde(default)]
    pub id: String,
    /// Interbank mid-market rate, with no spread, for the same conversion as `exchange_rate`. The gap between the two is the cost of the conversion. `null` when the payout provider does no foreign exchange, such as USD to a USD bank account. Also `null` when no market rate from the last 15 minutes is available, and on card-funded quotes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub market_exchange_rate: Option<f64>,
    /// When `market_exchange_rate` was taken from the market feed, as an ISO 8601 timestamp. `null` when `market_exchange_rate` is `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub market_exchange_rate_at: Option<DateTime<FixedOffset>>,
    /// Amount remaining after fees.
    #[serde(default)]
    pub net_amount: Money,
    pub object: CreateQuotePayoutsResponseObject,
    /// Server-signed quote token to submit to POST /payouts.
    #[serde(default)]
    pub quote_token: String,
    /// An optional payout method to connect for estimated savings. The quote still uses the requested saved method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_method: Option<CreateQuotePayoutsResponseRecommendedMethod>,
    /// Server time when the response was prepared, as an ISO 8601 timestamp. Subtract it from `expires_at` to get the remaining quote lifetime without relying on your own clock.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub server_time: DateTime<FixedOffset>,
}

impl CreateQuotePayoutsResponse {
    pub fn builder() -> CreateQuotePayoutsResponseBuilder {
        <CreateQuotePayoutsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateQuotePayoutsResponseBuilder {
    amount: Option<Money>,
    conversion: Option<CreateQuotePayoutsResponseConversion>,
    destination_amount: Option<Money>,
    exchange_rate: Option<f64>,
    expires_at: Option<DateTime<FixedOffset>>,
    fee: Option<Money>,
    id: Option<String>,
    market_exchange_rate: Option<f64>,
    market_exchange_rate_at: Option<DateTime<FixedOffset>>,
    net_amount: Option<Money>,
    object: Option<CreateQuotePayoutsResponseObject>,
    quote_token: Option<String>,
    recommended_method: Option<CreateQuotePayoutsResponseRecommendedMethod>,
    server_time: Option<DateTime<FixedOffset>>,
}

impl CreateQuotePayoutsResponseBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn conversion(mut self, value: CreateQuotePayoutsResponseConversion) -> Self {
        self.conversion = Some(value);
        self
    }

    pub fn destination_amount(mut self, value: Money) -> Self {
        self.destination_amount = Some(value);
        self
    }

    pub fn exchange_rate(mut self, value: f64) -> Self {
        self.exchange_rate = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn fee(mut self, value: Money) -> Self {
        self.fee = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn market_exchange_rate(mut self, value: f64) -> Self {
        self.market_exchange_rate = Some(value);
        self
    }

    pub fn market_exchange_rate_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.market_exchange_rate_at = Some(value);
        self
    }

    pub fn net_amount(mut self, value: Money) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn object(mut self, value: CreateQuotePayoutsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn quote_token(mut self, value: impl Into<String>) -> Self {
        self.quote_token = Some(value.into());
        self
    }

    pub fn recommended_method(
        mut self,
        value: CreateQuotePayoutsResponseRecommendedMethod,
    ) -> Self {
        self.recommended_method = Some(value);
        self
    }

    pub fn server_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.server_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateQuotePayoutsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreateQuotePayoutsResponseBuilder::amount)
    /// - [`destination_amount`](CreateQuotePayoutsResponseBuilder::destination_amount)
    /// - [`exchange_rate`](CreateQuotePayoutsResponseBuilder::exchange_rate)
    /// - [`expires_at`](CreateQuotePayoutsResponseBuilder::expires_at)
    /// - [`fee`](CreateQuotePayoutsResponseBuilder::fee)
    /// - [`id`](CreateQuotePayoutsResponseBuilder::id)
    /// - [`net_amount`](CreateQuotePayoutsResponseBuilder::net_amount)
    /// - [`object`](CreateQuotePayoutsResponseBuilder::object)
    /// - [`quote_token`](CreateQuotePayoutsResponseBuilder::quote_token)
    /// - [`server_time`](CreateQuotePayoutsResponseBuilder::server_time)
    pub fn build(self) -> Result<CreateQuotePayoutsResponse, BuildError> {
        Ok(CreateQuotePayoutsResponse {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            conversion: self.conversion,
            destination_amount: self
                .destination_amount
                .ok_or_else(|| BuildError::missing_field("destination_amount"))?,
            exchange_rate: self
                .exchange_rate
                .ok_or_else(|| BuildError::missing_field("exchange_rate"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            fee: self.fee.ok_or_else(|| BuildError::missing_field("fee"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            market_exchange_rate: self.market_exchange_rate,
            market_exchange_rate_at: self.market_exchange_rate_at,
            net_amount: self
                .net_amount
                .ok_or_else(|| BuildError::missing_field("net_amount"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            quote_token: self
                .quote_token
                .ok_or_else(|| BuildError::missing_field("quote_token"))?,
            recommended_method: self.recommended_method,
            server_time: self
                .server_time
                .ok_or_else(|| BuildError::missing_field("server_time"))?,
        })
    }
}
