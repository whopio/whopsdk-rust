pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TradingSpotAccount {
    #[serde(default)]
    pub holdings: Vec<TradingSpotHolding>,
}

impl TradingSpotAccount {
    pub fn builder() -> TradingSpotAccountBuilder {
        <TradingSpotAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradingSpotAccountBuilder {
    holdings: Option<Vec<TradingSpotHolding>>,
}

impl TradingSpotAccountBuilder {
    pub fn holdings(mut self, value: Vec<TradingSpotHolding>) -> Self {
        self.holdings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TradingSpotAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`holdings`](TradingSpotAccountBuilder::holdings)
    pub fn build(self) -> Result<TradingSpotAccount, BuildError> {
        Ok(TradingSpotAccount {
            holdings: self
                .holdings
                .ok_or_else(|| BuildError::missing_field("holdings"))?,
        })
    }
}
