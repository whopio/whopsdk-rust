pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePeopleResponseUsageBrowserItem {
    #[serde(default)]
    pub events: i64,
    #[serde(default)]
    pub value: String,
}

impl RetrievePeopleResponseUsageBrowserItem {
    pub fn builder() -> RetrievePeopleResponseUsageBrowserItemBuilder {
        <RetrievePeopleResponseUsageBrowserItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePeopleResponseUsageBrowserItemBuilder {
    events: Option<i64>,
    value: Option<String>,
}

impl RetrievePeopleResponseUsageBrowserItemBuilder {
    pub fn events(mut self, value: i64) -> Self {
        self.events = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePeopleResponseUsageBrowserItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](RetrievePeopleResponseUsageBrowserItemBuilder::events)
    /// - [`value`](RetrievePeopleResponseUsageBrowserItemBuilder::value)
    pub fn build(self) -> Result<RetrievePeopleResponseUsageBrowserItem, BuildError> {
        Ok(RetrievePeopleResponseUsageBrowserItem {
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
