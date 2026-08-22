pub use crate::prelude::*;

/// An ads performance report. Always returns a summary. The `granularity` field contains a per-bucket time series when the `granularity` arg is set; the `breakdown` field contains per-entity rows when the `breakdown` arg is set.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdReport {
    /// Per-entity rows over the date range. `null` when the `breakdown` arg on `adReport` is omitted; otherwise contains one row per ad campaign, ad group, or ad inside the requested scope at the requested level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Vec<AdReportBreakdownItem>>,
    /// Per-bucket time series over the date range, ordered ascending by `bucketStart`. `null` when the `granularity` arg on `adReport` is omitted; otherwise contains rows at the requested grain (`daily` or `hourly`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<Vec<AdReportGranularityItem>>,
    /// Aggregate totals and rates over the date range.
    #[serde(default)]
    pub summary: AdReportSummary,
}

impl AdReport {
    pub fn builder() -> AdReportBuilder {
        <AdReportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdReportBuilder {
    breakdown: Option<Vec<AdReportBreakdownItem>>,
    granularity: Option<Vec<AdReportGranularityItem>>,
    summary: Option<AdReportSummary>,
}

impl AdReportBuilder {
    pub fn breakdown(mut self, value: Vec<AdReportBreakdownItem>) -> Self {
        self.breakdown = Some(value);
        self
    }

    pub fn granularity(mut self, value: Vec<AdReportGranularityItem>) -> Self {
        self.granularity = Some(value);
        self
    }

    pub fn summary(mut self, value: AdReportSummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdReport`].
    /// This method will fail if any of the following fields are not set:
    /// - [`summary`](AdReportBuilder::summary)
    pub fn build(self) -> Result<AdReport, BuildError> {
        Ok(AdReport {
            breakdown: self.breakdown,
            granularity: self.granularity,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
