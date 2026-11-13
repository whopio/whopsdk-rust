pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountEconomicIntelligencePreviousPeriod {
    /// When the period ended, as an ISO 8601 timestamp.
    #[serde(default)]
    pub ended_at: String,
    /// Percentage of volume charged during the period, such as `3` for 3%.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub fee_percentage: f64,
}

impl AccountEconomicIntelligencePreviousPeriod {
    pub fn builder() -> AccountEconomicIntelligencePreviousPeriodBuilder {
        <AccountEconomicIntelligencePreviousPeriodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountEconomicIntelligencePreviousPeriodBuilder {
    ended_at: Option<String>,
    fee_percentage: Option<f64>,
}

impl AccountEconomicIntelligencePreviousPeriodBuilder {
    pub fn ended_at(mut self, value: impl Into<String>) -> Self {
        self.ended_at = Some(value.into());
        self
    }

    pub fn fee_percentage(mut self, value: f64) -> Self {
        self.fee_percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountEconomicIntelligencePreviousPeriod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ended_at`](AccountEconomicIntelligencePreviousPeriodBuilder::ended_at)
    /// - [`fee_percentage`](AccountEconomicIntelligencePreviousPeriodBuilder::fee_percentage)
    pub fn build(self) -> Result<AccountEconomicIntelligencePreviousPeriod, BuildError> {
        Ok(AccountEconomicIntelligencePreviousPeriod {
            ended_at: self
                .ended_at
                .ok_or_else(|| BuildError::missing_field("ended_at"))?,
            fee_percentage: self
                .fee_percentage
                .ok_or_else(|| BuildError::missing_field("fee_percentage"))?,
        })
    }
}
