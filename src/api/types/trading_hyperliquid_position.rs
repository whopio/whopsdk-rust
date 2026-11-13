pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TradingHyperliquidPosition {
    /// Funding paid on the position over several windows, in USD.
    #[serde(default)]
    pub cumulative_funding: TradingCumulativeFunding,
    /// Margin mode and multiplier for the position.
    pub leverage: TradingPositionLeverage,
    /// Estimated liquidation price in USD, or `null` when Hyperliquid reports none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation_price: Option<Money>,
    /// Margin allocated to the position, in USD.
    #[serde(default)]
    pub margin_used: Money,
    /// Return on equity as a decimal ratio string, such as `0.1` for 10%.
    #[serde(default)]
    pub return_on_equity: String,
}

impl TradingHyperliquidPosition {
    pub fn builder() -> TradingHyperliquidPositionBuilder {
        <TradingHyperliquidPositionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradingHyperliquidPositionBuilder {
    cumulative_funding: Option<TradingCumulativeFunding>,
    leverage: Option<TradingPositionLeverage>,
    liquidation_price: Option<Money>,
    margin_used: Option<Money>,
    return_on_equity: Option<String>,
}

impl TradingHyperliquidPositionBuilder {
    pub fn cumulative_funding(mut self, value: TradingCumulativeFunding) -> Self {
        self.cumulative_funding = Some(value);
        self
    }

    pub fn leverage(mut self, value: TradingPositionLeverage) -> Self {
        self.leverage = Some(value);
        self
    }

    pub fn liquidation_price(mut self, value: Money) -> Self {
        self.liquidation_price = Some(value);
        self
    }

    pub fn margin_used(mut self, value: Money) -> Self {
        self.margin_used = Some(value);
        self
    }

    pub fn return_on_equity(mut self, value: impl Into<String>) -> Self {
        self.return_on_equity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TradingHyperliquidPosition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cumulative_funding`](TradingHyperliquidPositionBuilder::cumulative_funding)
    /// - [`leverage`](TradingHyperliquidPositionBuilder::leverage)
    /// - [`margin_used`](TradingHyperliquidPositionBuilder::margin_used)
    /// - [`return_on_equity`](TradingHyperliquidPositionBuilder::return_on_equity)
    pub fn build(self) -> Result<TradingHyperliquidPosition, BuildError> {
        Ok(TradingHyperliquidPosition {
            cumulative_funding: self
                .cumulative_funding
                .ok_or_else(|| BuildError::missing_field("cumulative_funding"))?,
            leverage: self
                .leverage
                .ok_or_else(|| BuildError::missing_field("leverage"))?,
            liquidation_price: self.liquidation_price,
            margin_used: self
                .margin_used
                .ok_or_else(|| BuildError::missing_field("margin_used"))?,
            return_on_equity: self
                .return_on_equity
                .ok_or_else(|| BuildError::missing_field("return_on_equity"))?,
        })
    }
}
