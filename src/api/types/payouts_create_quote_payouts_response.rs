pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateQuotePayoutsResponse {
    /// Gross payout amount.
    #[serde(default)]
    pub amount: Money,
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
    /// Amount remaining after fees.
    #[serde(default)]
    pub net_amount: Money,
    pub object: CreateQuotePayoutsResponseObject,
    /// Server-signed quote token to submit to POST /payouts.
    #[serde(default)]
    pub quote_token: String,
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
    destination_amount: Option<Money>,
    exchange_rate: Option<f64>,
    expires_at: Option<DateTime<FixedOffset>>,
    fee: Option<Money>,
    id: Option<String>,
    net_amount: Option<Money>,
    object: Option<CreateQuotePayoutsResponseObject>,
    quote_token: Option<String>,
}

impl CreateQuotePayoutsResponseBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
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
    pub fn build(self) -> Result<CreateQuotePayoutsResponse, BuildError> {
        Ok(CreateQuotePayoutsResponse {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
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
            net_amount: self
                .net_amount
                .ok_or_else(|| BuildError::missing_field("net_amount"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            quote_token: self
                .quote_token
                .ok_or_else(|| BuildError::missing_field("quote_token"))?,
        })
    }
}
