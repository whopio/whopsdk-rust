pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Market {
    /// How much the price moved over the last 24 hours, as a percentage where `2` means up 2%. Set for tokenized stocks; `null` when it is not available.
    #[serde(rename = "change_24h_percentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub change24h_percentage: Option<f64>,
    /// The market's name: a perpetual's Hyperliquid name such as `BTC` or `xyz:TSLA`, which you send as `market` when you create a trade, or a spot asset's symbol such as `BTC`, `XAUT`, or a tokenized stock's ticker such as `AAPL`. A perpetual and a spot market can share an `id`, so `id` and `instrument_type` together identify a market.
    #[serde(default)]
    pub id: String,
    /// Whether this is a perpetual or a spot market. With `id`, it identifies the market.
    pub instrument_type: MarketInstrumentType,
    /// A logo image URL for the asset. Set for tokenized stocks; `null` when there is no logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// The highest cross leverage an order on this perpetual can use. `null` for a spot market, which trades without leverage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<i64>,
    /// The current price in USD: the midpoint between the best bid and ask, or a tokenized stock's latest market price. `null` when no price is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Money>,
    /// The most decimals an order's `size` can have on this market.
    #[serde(default)]
    pub size_decimals: i64,
}

impl Market {
    pub fn builder() -> MarketBuilder {
        <MarketBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarketBuilder {
    change24h_percentage: Option<f64>,
    id: Option<String>,
    instrument_type: Option<MarketInstrumentType>,
    logo_url: Option<String>,
    max_leverage: Option<i64>,
    price: Option<Money>,
    size_decimals: Option<i64>,
}

impl MarketBuilder {
    pub fn change24h_percentage(mut self, value: f64) -> Self {
        self.change24h_percentage = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn instrument_type(mut self, value: MarketInstrumentType) -> Self {
        self.instrument_type = Some(value);
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn max_leverage(mut self, value: i64) -> Self {
        self.max_leverage = Some(value);
        self
    }

    pub fn price(mut self, value: Money) -> Self {
        self.price = Some(value);
        self
    }

    pub fn size_decimals(mut self, value: i64) -> Self {
        self.size_decimals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Market`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MarketBuilder::id)
    /// - [`instrument_type`](MarketBuilder::instrument_type)
    /// - [`size_decimals`](MarketBuilder::size_decimals)
    pub fn build(self) -> Result<Market, BuildError> {
        Ok(Market {
            change24h_percentage: self.change24h_percentage,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            instrument_type: self
                .instrument_type
                .ok_or_else(|| BuildError::missing_field("instrument_type"))?,
            logo_url: self.logo_url,
            max_leverage: self.max_leverage,
            price: self.price,
            size_decimals: self
                .size_decimals
                .ok_or_else(|| BuildError::missing_field("size_decimals"))?,
        })
    }
}
