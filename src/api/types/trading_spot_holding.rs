pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TradingSpotHolding {
    /// The average price paid per unit of what is still held, in USD, counting the oldest units as sold first. `null` when it is not known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_entry_price: Option<Money>,
    /// What was paid in USD for the units still held.
    #[serde(default)]
    pub cost_basis: Money,
    /// The spot market's `id` from `GET /trades/markets`, such as `BTC`, `XAUT`, or a tokenized stock's ticker such as `AAPL`.
    #[serde(default)]
    pub market: String,
    /// Gains and losses already locked in by selling, in USD.
    #[serde(default)]
    pub realized_pnl: Money,
    /// The value of the units still held minus their `cost_basis`, in USD.
    #[serde(default)]
    pub unrealized_pnl: Money,
    /// `unrealized_pnl` as a percentage of `cost_basis`, where `2` means up 2%. `null` when it is not known.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub unrealized_pnl_percentage: Option<f64>,
}

impl TradingSpotHolding {
    pub fn builder() -> TradingSpotHoldingBuilder {
        <TradingSpotHoldingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradingSpotHoldingBuilder {
    average_entry_price: Option<Money>,
    cost_basis: Option<Money>,
    market: Option<String>,
    realized_pnl: Option<Money>,
    unrealized_pnl: Option<Money>,
    unrealized_pnl_percentage: Option<f64>,
}

impl TradingSpotHoldingBuilder {
    pub fn average_entry_price(mut self, value: Money) -> Self {
        self.average_entry_price = Some(value);
        self
    }

    pub fn cost_basis(mut self, value: Money) -> Self {
        self.cost_basis = Some(value);
        self
    }

    pub fn market(mut self, value: impl Into<String>) -> Self {
        self.market = Some(value.into());
        self
    }

    pub fn realized_pnl(mut self, value: Money) -> Self {
        self.realized_pnl = Some(value);
        self
    }

    pub fn unrealized_pnl(mut self, value: Money) -> Self {
        self.unrealized_pnl = Some(value);
        self
    }

    pub fn unrealized_pnl_percentage(mut self, value: f64) -> Self {
        self.unrealized_pnl_percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TradingSpotHolding`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cost_basis`](TradingSpotHoldingBuilder::cost_basis)
    /// - [`market`](TradingSpotHoldingBuilder::market)
    /// - [`realized_pnl`](TradingSpotHoldingBuilder::realized_pnl)
    /// - [`unrealized_pnl`](TradingSpotHoldingBuilder::unrealized_pnl)
    pub fn build(self) -> Result<TradingSpotHolding, BuildError> {
        Ok(TradingSpotHolding {
            average_entry_price: self.average_entry_price,
            cost_basis: self
                .cost_basis
                .ok_or_else(|| BuildError::missing_field("cost_basis"))?,
            market: self
                .market
                .ok_or_else(|| BuildError::missing_field("market"))?,
            realized_pnl: self
                .realized_pnl
                .ok_or_else(|| BuildError::missing_field("realized_pnl"))?,
            unrealized_pnl: self
                .unrealized_pnl
                .ok_or_else(|| BuildError::missing_field("unrealized_pnl"))?,
            unrealized_pnl_percentage: self.unrealized_pnl_percentage,
        })
    }
}
