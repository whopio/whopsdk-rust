pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrieveStatsResponseDataTotalsItem {
    /// The property value this total is for, or the metric's name when it isn't split by a property.
    #[serde(default)]
    pub name: String,
    /// The metric's whole-window value for this entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
}

impl RetrieveStatsResponseDataTotalsItem {
    pub fn builder() -> RetrieveStatsResponseDataTotalsItemBuilder {
        <RetrieveStatsResponseDataTotalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveStatsResponseDataTotalsItemBuilder {
    name: Option<String>,
    value: Option<f64>,
}

impl RetrieveStatsResponseDataTotalsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveStatsResponseDataTotalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RetrieveStatsResponseDataTotalsItemBuilder::name)
    pub fn build(self) -> Result<RetrieveStatsResponseDataTotalsItem, BuildError> {
        Ok(RetrieveStatsResponseDataTotalsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self.value,
        })
    }
}
