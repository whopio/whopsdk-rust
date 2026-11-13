pub use crate::prelude::*;

/// The account's last Economic Intelligence period, once it has ended. `null` while Economic Intelligence is on, or when it has never been on.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrievePreferencesResponseEconomicIntelligencePreviousPeriod {
    /// When the period ended, as an ISO 8601 timestamp.
    #[serde(default)]
    pub ended_at: String,
    /// Percentage of volume charged during the period, such as `3` for 3%.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub fee_percentage: f64,
}

impl RetrievePreferencesResponseEconomicIntelligencePreviousPeriod {
    pub fn builder() -> RetrievePreferencesResponseEconomicIntelligencePreviousPeriodBuilder {
        <RetrievePreferencesResponseEconomicIntelligencePreviousPeriodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePreferencesResponseEconomicIntelligencePreviousPeriodBuilder {
    ended_at: Option<String>,
    fee_percentage: Option<f64>,
}

impl RetrievePreferencesResponseEconomicIntelligencePreviousPeriodBuilder {
    pub fn ended_at(mut self, value: impl Into<String>) -> Self {
        self.ended_at = Some(value.into());
        self
    }

    pub fn fee_percentage(mut self, value: f64) -> Self {
        self.fee_percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePreferencesResponseEconomicIntelligencePreviousPeriod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ended_at`](RetrievePreferencesResponseEconomicIntelligencePreviousPeriodBuilder::ended_at)
    /// - [`fee_percentage`](RetrievePreferencesResponseEconomicIntelligencePreviousPeriodBuilder::fee_percentage)
    pub fn build(
        self,
    ) -> Result<RetrievePreferencesResponseEconomicIntelligencePreviousPeriod, BuildError> {
        Ok(
            RetrievePreferencesResponseEconomicIntelligencePreviousPeriod {
                ended_at: self
                    .ended_at
                    .ok_or_else(|| BuildError::missing_field("ended_at"))?,
                fee_percentage: self
                    .fee_percentage
                    .ok_or_else(|| BuildError::missing_field("fee_percentage"))?,
            },
        )
    }
}
