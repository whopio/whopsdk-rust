pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListStatsResponseDataItem {
    /// A short description of what the metric measures.
    #[serde(default)]
    pub description: String,
    /// The metric's key. Pass it to GET /stats/{metric} to query its values.
    #[serde(default)]
    pub key: String,
    /// Human-readable display name for the metric.
    #[serde(default)]
    pub name: String,
    /// The properties you can use with this metric — pass one as a filter (property=value) to narrow the series, or as breakdown_by=property to split it.
    #[serde(default)]
    pub properties: Vec<String>,
    /// How to read the metric's values: count is an integer, currency is a decimal amount, and percent is a number where 1.6 means 1.6%.
    pub unit: ListStatsResponseDataItemUnit,
    /// Snapshot metrics only: the trailing windows you can pass as snapshot_window, for example 30d. Absent on live metrics, which use from/to instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<Vec<String>>,
}

impl ListStatsResponseDataItem {
    pub fn builder() -> ListStatsResponseDataItemBuilder {
        <ListStatsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStatsResponseDataItemBuilder {
    description: Option<String>,
    key: Option<String>,
    name: Option<String>,
    properties: Option<Vec<String>>,
    unit: Option<ListStatsResponseDataItemUnit>,
    windows: Option<Vec<String>>,
}

impl ListStatsResponseDataItemBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn properties(mut self, value: Vec<String>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn unit(mut self, value: ListStatsResponseDataItemUnit) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn windows(mut self, value: Vec<String>) -> Self {
        self.windows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListStatsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](ListStatsResponseDataItemBuilder::description)
    /// - [`key`](ListStatsResponseDataItemBuilder::key)
    /// - [`name`](ListStatsResponseDataItemBuilder::name)
    /// - [`properties`](ListStatsResponseDataItemBuilder::properties)
    /// - [`unit`](ListStatsResponseDataItemBuilder::unit)
    pub fn build(self) -> Result<ListStatsResponseDataItem, BuildError> {
        Ok(ListStatsResponseDataItem {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            properties: self
                .properties
                .ok_or_else(|| BuildError::missing_field("properties"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            windows: self.windows,
        })
    }
}
