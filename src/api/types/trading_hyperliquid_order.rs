pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TradingHyperliquidOrder {
    /// Whether the order can only reduce an existing position, or `null` when Hyperliquid omits it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Trigger price in USD for take-profit and stop-loss orders, or `null` for orders without a trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<Money>,
}

impl TradingHyperliquidOrder {
    pub fn builder() -> TradingHyperliquidOrderBuilder {
        <TradingHyperliquidOrderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradingHyperliquidOrderBuilder {
    reduce_only: Option<bool>,
    trigger_price: Option<Money>,
}

impl TradingHyperliquidOrderBuilder {
    pub fn reduce_only(mut self, value: bool) -> Self {
        self.reduce_only = Some(value);
        self
    }

    pub fn trigger_price(mut self, value: Money) -> Self {
        self.trigger_price = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TradingHyperliquidOrder`].
    pub fn build(self) -> Result<TradingHyperliquidOrder, BuildError> {
        Ok(TradingHyperliquidOrder {
            reduce_only: self.reduce_only,
            trigger_price: self.trigger_price,
        })
    }
}
