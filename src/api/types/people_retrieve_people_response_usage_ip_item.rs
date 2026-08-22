pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePeopleResponseUsageIpItem {
    #[serde(default)]
    pub events: i64,
    #[serde(default)]
    pub value: String,
}

impl RetrievePeopleResponseUsageIpItem {
    pub fn builder() -> RetrievePeopleResponseUsageIpItemBuilder {
        <RetrievePeopleResponseUsageIpItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePeopleResponseUsageIpItemBuilder {
    events: Option<i64>,
    value: Option<String>,
}

impl RetrievePeopleResponseUsageIpItemBuilder {
    pub fn events(mut self, value: i64) -> Self {
        self.events = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePeopleResponseUsageIpItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](RetrievePeopleResponseUsageIpItemBuilder::events)
    /// - [`value`](RetrievePeopleResponseUsageIpItemBuilder::value)
    pub fn build(self) -> Result<RetrievePeopleResponseUsageIpItem, BuildError> {
        Ok(RetrievePeopleResponseUsageIpItem {
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
