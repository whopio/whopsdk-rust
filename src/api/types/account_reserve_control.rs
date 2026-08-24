pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountReserveControl {
    /// Number of days reserved funds are held before release.
    #[serde(default)]
    pub hold_period_days: i64,
    /// Percentage of incoming payment volume held in reserve. `null` when no reserve is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percentage: Option<f64>,
}

impl AccountReserveControl {
    pub fn builder() -> AccountReserveControlBuilder {
        <AccountReserveControlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountReserveControlBuilder {
    hold_period_days: Option<i64>,
    percentage: Option<f64>,
}

impl AccountReserveControlBuilder {
    pub fn hold_period_days(mut self, value: i64) -> Self {
        self.hold_period_days = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountReserveControl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hold_period_days`](AccountReserveControlBuilder::hold_period_days)
    pub fn build(self) -> Result<AccountReserveControl, BuildError> {
        Ok(AccountReserveControl {
            hold_period_days: self
                .hold_period_days
                .ok_or_else(|| BuildError::missing_field("hold_period_days"))?,
            percentage: self.percentage,
        })
    }
}
