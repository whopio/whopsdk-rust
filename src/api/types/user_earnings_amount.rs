pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserEarningsAmount {
    /// Gross income in USD over the last 24 hours.
    #[serde(rename = "last_24_hours")]
    #[serde(default)]
    pub last24hours: String,
    /// Gross income in USD over the last 30 days.
    #[serde(rename = "last_30_days")]
    #[serde(default)]
    pub last30days: String,
    /// Gross income in USD over the last 7 days.
    #[serde(rename = "last_7_days")]
    #[serde(default)]
    pub last7days: String,
    /// All-time gross income in USD.
    #[serde(default)]
    pub lifetime: String,
}

impl UserEarningsAmount {
    pub fn builder() -> UserEarningsAmountBuilder {
        <UserEarningsAmountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserEarningsAmountBuilder {
    last24hours: Option<String>,
    last30days: Option<String>,
    last7days: Option<String>,
    lifetime: Option<String>,
}

impl UserEarningsAmountBuilder {
    pub fn last24hours(mut self, value: impl Into<String>) -> Self {
        self.last24hours = Some(value.into());
        self
    }

    pub fn last30days(mut self, value: impl Into<String>) -> Self {
        self.last30days = Some(value.into());
        self
    }

    pub fn last7days(mut self, value: impl Into<String>) -> Self {
        self.last7days = Some(value.into());
        self
    }

    pub fn lifetime(mut self, value: impl Into<String>) -> Self {
        self.lifetime = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserEarningsAmount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`last24hours`](UserEarningsAmountBuilder::last24hours)
    /// - [`last30days`](UserEarningsAmountBuilder::last30days)
    /// - [`last7days`](UserEarningsAmountBuilder::last7days)
    /// - [`lifetime`](UserEarningsAmountBuilder::lifetime)
    pub fn build(self) -> Result<UserEarningsAmount, BuildError> {
        Ok(UserEarningsAmount {
            last24hours: self
                .last24hours
                .ok_or_else(|| BuildError::missing_field("last24hours"))?,
            last30days: self
                .last30days
                .ok_or_else(|| BuildError::missing_field("last30days"))?,
            last7days: self
                .last7days
                .ok_or_else(|| BuildError::missing_field("last7days"))?,
            lifetime: self
                .lifetime
                .ok_or_else(|| BuildError::missing_field("lifetime"))?,
        })
    }
}
