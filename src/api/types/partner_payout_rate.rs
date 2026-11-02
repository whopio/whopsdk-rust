pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PartnerPayoutRate {
    /// Income source that generates this percentage payout.
    pub income_source: PartnerPayoutRateIncomeSource,
    /// Partner's default percentage for this tier and income source. For example, 30 means 30%.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub percentage: f64,
}

impl PartnerPayoutRate {
    pub fn builder() -> PartnerPayoutRateBuilder {
        <PartnerPayoutRateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerPayoutRateBuilder {
    income_source: Option<PartnerPayoutRateIncomeSource>,
    percentage: Option<f64>,
}

impl PartnerPayoutRateBuilder {
    pub fn income_source(mut self, value: PartnerPayoutRateIncomeSource) -> Self {
        self.income_source = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartnerPayoutRate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`income_source`](PartnerPayoutRateBuilder::income_source)
    /// - [`percentage`](PartnerPayoutRateBuilder::percentage)
    pub fn build(self) -> Result<PartnerPayoutRate, BuildError> {
        Ok(PartnerPayoutRate {
            income_source: self
                .income_source
                .ok_or_else(|| BuildError::missing_field("income_source"))?,
            percentage: self
                .percentage
                .ok_or_else(|| BuildError::missing_field("percentage"))?,
        })
    }
}
