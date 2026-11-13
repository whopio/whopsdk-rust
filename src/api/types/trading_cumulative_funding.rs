pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TradingCumulativeFunding {
    /// Funding paid on this market across the account's history, in USD, or `null` when unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_time: Option<Money>,
    /// Funding paid since the position size last changed, in USD, or `null` when unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_change: Option<Money>,
    /// Funding paid since the position opened, in USD, or `null` when unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_open: Option<Money>,
}

impl TradingCumulativeFunding {
    pub fn builder() -> TradingCumulativeFundingBuilder {
        <TradingCumulativeFundingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradingCumulativeFundingBuilder {
    all_time: Option<Money>,
    since_change: Option<Money>,
    since_open: Option<Money>,
}

impl TradingCumulativeFundingBuilder {
    pub fn all_time(mut self, value: Money) -> Self {
        self.all_time = Some(value);
        self
    }

    pub fn since_change(mut self, value: Money) -> Self {
        self.since_change = Some(value);
        self
    }

    pub fn since_open(mut self, value: Money) -> Self {
        self.since_open = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TradingCumulativeFunding`].
    pub fn build(self) -> Result<TradingCumulativeFunding, BuildError> {
        Ok(TradingCumulativeFunding {
            all_time: self.all_time,
            since_change: self.since_change,
            since_open: self.since_open,
        })
    }
}
