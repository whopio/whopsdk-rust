pub use crate::prelude::*;

/// Query parameters for pulse
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PulseQueryRequest {
    /// Filter to one or more types, comma separated — for example `purchase,card_spend`. These are the item's `type`, not its `event_name`: several types share the `ledger_line.created` event name. Omit for every type in the feed. Values outside the feed's own set are rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// The number of events to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor for fetching events after a previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for fetching events before a later page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl PulseQueryRequest {
    pub fn builder() -> PulseQueryRequestBuilder {
        <PulseQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PulseQueryRequestBuilder {
    event: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

impl PulseQueryRequestBuilder {
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
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

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PulseQueryRequest`].
    pub fn build(self) -> Result<PulseQueryRequest, BuildError> {
        Ok(PulseQueryRequest {
            event: self.event,
            first: self.first,
            after: self.after,
            before: self.before,
        })
    }
}
