pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Candle {
    /// The last price in the candle, in USD.
    #[serde(default)]
    pub close: Money,
    /// The highest price in the candle, in USD.
    #[serde(default)]
    pub high: Money,
    /// The lowest price in the candle, in USD.
    #[serde(default)]
    pub low: Money,
    /// The first price in the candle, in USD.
    #[serde(default)]
    pub open: Money,
    /// When the candle starts, as an ISO 8601 timestamp. The candle covers one `interval` from this time.
    #[serde(default)]
    pub started_at: String,
    /// The traded volume in the candle, in USD. `null` when no volume is reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Money>,
}

impl Candle {
    pub fn builder() -> CandleBuilder {
        <CandleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CandleBuilder {
    close: Option<Money>,
    high: Option<Money>,
    low: Option<Money>,
    open: Option<Money>,
    started_at: Option<String>,
    volume: Option<Money>,
}

impl CandleBuilder {
    pub fn close(mut self, value: Money) -> Self {
        self.close = Some(value);
        self
    }

    pub fn high(mut self, value: Money) -> Self {
        self.high = Some(value);
        self
    }

    pub fn low(mut self, value: Money) -> Self {
        self.low = Some(value);
        self
    }

    pub fn open(mut self, value: Money) -> Self {
        self.open = Some(value);
        self
    }

    pub fn started_at(mut self, value: impl Into<String>) -> Self {
        self.started_at = Some(value.into());
        self
    }

    pub fn volume(mut self, value: Money) -> Self {
        self.volume = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Candle`].
    /// This method will fail if any of the following fields are not set:
    /// - [`close`](CandleBuilder::close)
    /// - [`high`](CandleBuilder::high)
    /// - [`low`](CandleBuilder::low)
    /// - [`open`](CandleBuilder::open)
    /// - [`started_at`](CandleBuilder::started_at)
    pub fn build(self) -> Result<Candle, BuildError> {
        Ok(Candle {
            close: self
                .close
                .ok_or_else(|| BuildError::missing_field("close"))?,
            high: self.high.ok_or_else(|| BuildError::missing_field("high"))?,
            low: self.low.ok_or_else(|| BuildError::missing_field("low"))?,
            open: self.open.ok_or_else(|| BuildError::missing_field("open"))?,
            started_at: self
                .started_at
                .ok_or_else(|| BuildError::missing_field("started_at"))?,
            volume: self.volume,
        })
    }
}
