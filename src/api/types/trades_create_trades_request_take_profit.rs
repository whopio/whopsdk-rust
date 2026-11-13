pub use crate::prelude::*;

/// Only for a perpetual order that opens or adds to a position. A take-profit for the order's full size, armed once the order fills and sent with it as its own trade, whose `parent_id` is this trade.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTradesRequestTakeProfit {
    /// The USD price, as a decimal string Hyperliquid accepts without rounding, at which the take-profit sends its market order. Above `price` (or the mid price for a market order) for a buy, below it for a sell.
    #[serde(default)]
    pub trigger_price: String,
}

impl CreateTradesRequestTakeProfit {
    pub fn builder() -> CreateTradesRequestTakeProfitBuilder {
        <CreateTradesRequestTakeProfitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTradesRequestTakeProfitBuilder {
    trigger_price: Option<String>,
}

impl CreateTradesRequestTakeProfitBuilder {
    pub fn trigger_price(mut self, value: impl Into<String>) -> Self {
        self.trigger_price = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTradesRequestTakeProfit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trigger_price`](CreateTradesRequestTakeProfitBuilder::trigger_price)
    pub fn build(self) -> Result<CreateTradesRequestTakeProfit, BuildError> {
        Ok(CreateTradesRequestTakeProfit {
            trigger_price: self
                .trigger_price
                .ok_or_else(|| BuildError::missing_field("trigger_price"))?,
        })
    }
}
