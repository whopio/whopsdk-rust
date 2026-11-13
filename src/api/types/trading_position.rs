pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TradingPosition {
    /// Average entry price in USD, or `null` when the provider omits it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_price: Option<Money>,
    /// Hyperliquid perpetual details. Present on Hyperliquid positions, otherwise `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperliquid: Option<TradingHyperliquidPosition>,
    /// Trading position ID, prefixed `trdpos_`. Stable for a market within one trading account.
    #[serde(default)]
    pub id: String,
    /// Market symbol on the provider, such as `ETH`.
    #[serde(default)]
    pub market: String,
    pub object: TradingPositionObject,
    /// Current position value in USD.
    #[serde(default)]
    pub position_value: Money,
    pub side: TradingPositionSide,
    /// Absolute position size as a decimal string.
    #[serde(default)]
    pub size: String,
    /// Unrealized profit or loss in USD. Negative for a loss.
    #[serde(default)]
    pub unrealized_pnl: Money,
}

impl TradingPosition {
    pub fn builder() -> TradingPositionBuilder {
        <TradingPositionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradingPositionBuilder {
    entry_price: Option<Money>,
    hyperliquid: Option<TradingHyperliquidPosition>,
    id: Option<String>,
    market: Option<String>,
    object: Option<TradingPositionObject>,
    position_value: Option<Money>,
    side: Option<TradingPositionSide>,
    size: Option<String>,
    unrealized_pnl: Option<Money>,
}

impl TradingPositionBuilder {
    pub fn entry_price(mut self, value: Money) -> Self {
        self.entry_price = Some(value);
        self
    }

    pub fn hyperliquid(mut self, value: TradingHyperliquidPosition) -> Self {
        self.hyperliquid = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn market(mut self, value: impl Into<String>) -> Self {
        self.market = Some(value.into());
        self
    }

    pub fn object(mut self, value: TradingPositionObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn position_value(mut self, value: Money) -> Self {
        self.position_value = Some(value);
        self
    }

    pub fn side(mut self, value: TradingPositionSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn size(mut self, value: impl Into<String>) -> Self {
        self.size = Some(value.into());
        self
    }

    pub fn unrealized_pnl(mut self, value: Money) -> Self {
        self.unrealized_pnl = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TradingPosition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TradingPositionBuilder::id)
    /// - [`market`](TradingPositionBuilder::market)
    /// - [`object`](TradingPositionBuilder::object)
    /// - [`position_value`](TradingPositionBuilder::position_value)
    /// - [`side`](TradingPositionBuilder::side)
    /// - [`size`](TradingPositionBuilder::size)
    /// - [`unrealized_pnl`](TradingPositionBuilder::unrealized_pnl)
    pub fn build(self) -> Result<TradingPosition, BuildError> {
        Ok(TradingPosition {
            entry_price: self.entry_price,
            hyperliquid: self.hyperliquid,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            market: self
                .market
                .ok_or_else(|| BuildError::missing_field("market"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            position_value: self
                .position_value
                .ok_or_else(|| BuildError::missing_field("position_value"))?,
            side: self.side.ok_or_else(|| BuildError::missing_field("side"))?,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
            unrealized_pnl: self
                .unrealized_pnl
                .ok_or_else(|| BuildError::missing_field("unrealized_pnl"))?,
        })
    }
}
