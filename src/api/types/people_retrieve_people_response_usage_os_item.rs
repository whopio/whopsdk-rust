pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePeopleResponseUsageOsItem {
    #[serde(default)]
    pub events: i64,
    #[serde(default)]
    pub value: String,
}

impl RetrievePeopleResponseUsageOsItem {
    pub fn builder() -> RetrievePeopleResponseUsageOsItemBuilder {
        <RetrievePeopleResponseUsageOsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePeopleResponseUsageOsItemBuilder {
    events: Option<i64>,
    value: Option<String>,
}

impl RetrievePeopleResponseUsageOsItemBuilder {
    pub fn events(mut self, value: i64) -> Self {
        self.events = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePeopleResponseUsageOsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](RetrievePeopleResponseUsageOsItemBuilder::events)
    /// - [`value`](RetrievePeopleResponseUsageOsItemBuilder::value)
    pub fn build(self) -> Result<RetrievePeopleResponseUsageOsItem, BuildError> {
        Ok(RetrievePeopleResponseUsageOsItem {
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
