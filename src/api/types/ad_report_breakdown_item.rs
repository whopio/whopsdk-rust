pub use crate::prelude::*;

/// Per-entity ad performance row. Returned when the `breakdown` arg on `adReport` is set.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdReportBreakdownItem {
    /// Per-bucket time series for this entity over the date range, ordered ascending by `bucketStart`. `null` when the `granularity` arg on `adReport` is omitted; otherwise contains rows at the requested grain (`daily` or `hourly`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<Vec<AdReportBreakdownItemGranularityItem>>,
    /// Tag of the entity (ad campaign, ad group, or ad).
    #[serde(default)]
    pub id: String,
    /// The entity level of this row — matches the `breakdown` arg.
    pub level: AdReportBreakdownLevels,
    /// Display name of the entity, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Aggregate totals and rates for this entity over the date range.
    #[serde(default)]
    pub summary: AdReportBreakdownItemSummary,
}

impl AdReportBreakdownItem {
    pub fn builder() -> AdReportBreakdownItemBuilder {
        <AdReportBreakdownItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdReportBreakdownItemBuilder {
    granularity: Option<Vec<AdReportBreakdownItemGranularityItem>>,
    id: Option<String>,
    level: Option<AdReportBreakdownLevels>,
    name: Option<String>,
    summary: Option<AdReportBreakdownItemSummary>,
}

impl AdReportBreakdownItemBuilder {
    pub fn granularity(mut self, value: Vec<AdReportBreakdownItemGranularityItem>) -> Self {
        self.granularity = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn level(mut self, value: AdReportBreakdownLevels) -> Self {
        self.level = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn summary(mut self, value: AdReportBreakdownItemSummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdReportBreakdownItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AdReportBreakdownItemBuilder::id)
    /// - [`level`](AdReportBreakdownItemBuilder::level)
    /// - [`summary`](AdReportBreakdownItemBuilder::summary)
    pub fn build(self) -> Result<AdReportBreakdownItem, BuildError> {
        Ok(AdReportBreakdownItem {
            granularity: self.granularity,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            level: self
                .level
                .ok_or_else(|| BuildError::missing_field("level"))?,
            name: self.name,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
