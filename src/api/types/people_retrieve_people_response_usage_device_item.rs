pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePeopleResponseUsageDeviceItem {
    #[serde(default)]
    pub events: i64,
    #[serde(default)]
    pub value: String,
}

impl RetrievePeopleResponseUsageDeviceItem {
    pub fn builder() -> RetrievePeopleResponseUsageDeviceItemBuilder {
        <RetrievePeopleResponseUsageDeviceItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePeopleResponseUsageDeviceItemBuilder {
    events: Option<i64>,
    value: Option<String>,
}

impl RetrievePeopleResponseUsageDeviceItemBuilder {
    pub fn events(mut self, value: i64) -> Self {
        self.events = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePeopleResponseUsageDeviceItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](RetrievePeopleResponseUsageDeviceItemBuilder::events)
    /// - [`value`](RetrievePeopleResponseUsageDeviceItemBuilder::value)
    pub fn build(self) -> Result<RetrievePeopleResponseUsageDeviceItem, BuildError> {
        Ok(RetrievePeopleResponseUsageDeviceItem {
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
