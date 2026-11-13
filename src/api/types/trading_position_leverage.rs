pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TradingPositionLeverage {
    /// `cross` shares margin across positions. `isolated` limits margin to this position.
    pub r#type: TradingPositionLeverageType,
    /// Multiplier applied to the position's margin, such as `10` for 10x.
    #[serde(default)]
    pub value: i64,
}

impl TradingPositionLeverage {
    pub fn builder() -> TradingPositionLeverageBuilder {
        <TradingPositionLeverageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradingPositionLeverageBuilder {
    r#type: Option<TradingPositionLeverageType>,
    value: Option<i64>,
}

impl TradingPositionLeverageBuilder {
    pub fn r#type(mut self, value: TradingPositionLeverageType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TradingPositionLeverage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TradingPositionLeverageBuilder::r#type)
    /// - [`value`](TradingPositionLeverageBuilder::value)
    pub fn build(self) -> Result<TradingPositionLeverage, BuildError> {
        Ok(TradingPositionLeverage {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
