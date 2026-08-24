pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrieveStatsResponseDataPointsItemBreakdownItem {
    /// The property value, for example usd or visa.
    #[serde(default)]
    pub name: String,
    /// The metric's value for this entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
}

impl RetrieveStatsResponseDataPointsItemBreakdownItem {
    pub fn builder() -> RetrieveStatsResponseDataPointsItemBreakdownItemBuilder {
        <RetrieveStatsResponseDataPointsItemBreakdownItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveStatsResponseDataPointsItemBreakdownItemBuilder {
    name: Option<String>,
    value: Option<f64>,
}

impl RetrieveStatsResponseDataPointsItemBreakdownItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveStatsResponseDataPointsItemBreakdownItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RetrieveStatsResponseDataPointsItemBreakdownItemBuilder::name)
    pub fn build(self) -> Result<RetrieveStatsResponseDataPointsItemBreakdownItem, BuildError> {
        Ok(RetrieveStatsResponseDataPointsItemBreakdownItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self.value,
        })
    }
}
