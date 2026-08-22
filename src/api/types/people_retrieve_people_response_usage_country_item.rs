pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePeopleResponseUsageCountryItem {
    #[serde(default)]
    pub events: i64,
    #[serde(default)]
    pub value: String,
}

impl RetrievePeopleResponseUsageCountryItem {
    pub fn builder() -> RetrievePeopleResponseUsageCountryItemBuilder {
        <RetrievePeopleResponseUsageCountryItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePeopleResponseUsageCountryItemBuilder {
    events: Option<i64>,
    value: Option<String>,
}

impl RetrievePeopleResponseUsageCountryItemBuilder {
    pub fn events(mut self, value: i64) -> Self {
        self.events = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePeopleResponseUsageCountryItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](RetrievePeopleResponseUsageCountryItemBuilder::events)
    /// - [`value`](RetrievePeopleResponseUsageCountryItemBuilder::value)
    pub fn build(self) -> Result<RetrievePeopleResponseUsageCountryItem, BuildError> {
        Ok(RetrievePeopleResponseUsageCountryItem {
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
