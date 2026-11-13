pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainStats {
    /// People who viewed a page on the domain with the Whop pixel in the last 7 days, refreshed hourly.
    #[serde(rename = "unique_visitors_last_7_days")]
    #[serde(default)]
    pub unique_visitors_last7days: i64,
}

impl DomainStats {
    pub fn builder() -> DomainStatsBuilder {
        <DomainStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainStatsBuilder {
    unique_visitors_last7days: Option<i64>,
}

impl DomainStatsBuilder {
    pub fn unique_visitors_last7days(mut self, value: i64) -> Self {
        self.unique_visitors_last7days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainStats`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unique_visitors_last7days`](DomainStatsBuilder::unique_visitors_last7days)
    pub fn build(self) -> Result<DomainStats, BuildError> {
        Ok(DomainStats {
            unique_visitors_last7days: self
                .unique_visitors_last7days
                .ok_or_else(|| BuildError::missing_field("unique_visitors_last7days"))?,
        })
    }
}
