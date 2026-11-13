pub use crate::prelude::*;

/// Only for a perpetual order that opens or adds to a position. A stop-loss for the order's full size, armed once the order fills and sent with it as its own trade, whose `parent_id` is this trade.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTradesRequestStopLoss {
    /// The USD price, as a decimal string Hyperliquid accepts without rounding, at which the stop-loss sends its market order. Below `price` (or the mid price for a market order) for a buy, above it for a sell.
    #[serde(default)]
    pub trigger_price: String,
}

impl CreateTradesRequestStopLoss {
    pub fn builder() -> CreateTradesRequestStopLossBuilder {
        <CreateTradesRequestStopLossBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTradesRequestStopLossBuilder {
    trigger_price: Option<String>,
}

impl CreateTradesRequestStopLossBuilder {
    pub fn trigger_price(mut self, value: impl Into<String>) -> Self {
        self.trigger_price = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTradesRequestStopLoss`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trigger_price`](CreateTradesRequestStopLossBuilder::trigger_price)
    pub fn build(self) -> Result<CreateTradesRequestStopLoss, BuildError> {
        Ok(CreateTradesRequestStopLoss {
            trigger_price: self
                .trigger_price
                .ok_or_else(|| BuildError::missing_field("trigger_price"))?,
        })
    }
}
