pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppUsageStats {
    /// Daily active users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dau: Option<i64>,
    /// Monthly active users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mau: Option<i64>,
    /// Total time users spent in the app over the last 24 hours, in seconds.
    #[serde(rename = "time_spent_last24_hours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_spent_last24hours: Option<i64>,
    /// Weekly active users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wau: Option<i64>,
}

impl AppUsageStats {
    pub fn builder() -> AppUsageStatsBuilder {
        <AppUsageStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppUsageStatsBuilder {
    dau: Option<i64>,
    mau: Option<i64>,
    time_spent_last24hours: Option<i64>,
    wau: Option<i64>,
}

impl AppUsageStatsBuilder {
    pub fn dau(mut self, value: i64) -> Self {
        self.dau = Some(value);
        self
    }

    pub fn mau(mut self, value: i64) -> Self {
        self.mau = Some(value);
        self
    }

    pub fn time_spent_last24hours(mut self, value: i64) -> Self {
        self.time_spent_last24hours = Some(value);
        self
    }

    pub fn wau(mut self, value: i64) -> Self {
        self.wau = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppUsageStats`].
    pub fn build(self) -> Result<AppUsageStats, BuildError> {
        Ok(AppUsageStats {
            dau: self.dau,
            mau: self.mau,
            time_spent_last24hours: self.time_spent_last24hours,
            wau: self.wau,
        })
    }
}
