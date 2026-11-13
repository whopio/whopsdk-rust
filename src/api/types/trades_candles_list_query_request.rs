pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TradesCandlesListQueryRequest {
    /// The tokenized stock's ticker, such as `NVDA`. This is the `id` of a spot market from List Markets.
    #[serde(default)]
    pub market: String,
    /// How much time each candle covers.
    pub interval: ListCandlesRequestInterval,
    /// Return candles that start at or after this time, as an ISO 8601 timestamp.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub from: DateTime<FixedOffset>,
    /// Return candles that start before this time, as an ISO 8601 timestamp. Defaults to now.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub to: Option<DateTime<FixedOffset>>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl TradesCandlesListQueryRequest {
    pub fn builder() -> TradesCandlesListQueryRequestBuilder {
        <TradesCandlesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradesCandlesListQueryRequestBuilder {
    market: Option<String>,
    interval: Option<ListCandlesRequestInterval>,
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl TradesCandlesListQueryRequestBuilder {
    pub fn market(mut self, value: impl Into<String>) -> Self {
        self.market = Some(value.into());
        self
    }

    pub fn interval(mut self, value: ListCandlesRequestInterval) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TradesCandlesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`market`](TradesCandlesListQueryRequestBuilder::market)
    /// - [`interval`](TradesCandlesListQueryRequestBuilder::interval)
    /// - [`from`](TradesCandlesListQueryRequestBuilder::from)
    pub fn build(self) -> Result<TradesCandlesListQueryRequest, BuildError> {
        Ok(TradesCandlesListQueryRequest {
            market: self
                .market
                .ok_or_else(|| BuildError::missing_field("market"))?,
            interval: self
                .interval
                .ok_or_else(|| BuildError::missing_field("interval"))?,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
