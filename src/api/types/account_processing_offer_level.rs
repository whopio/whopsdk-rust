pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountProcessingOfferLevel {
    /// The processing level.
    #[serde(default)]
    pub level: i64,
    /// The monthly qualifying spend required to reach this level.
    #[serde(default)]
    pub minimum_spend: Money,
    /// The standard US and Canada card processing percentage at this level, regardless of the account's custom rate.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub processing_rate_percentage: f64,
    #[serde(default)]
    pub regional_rates: Vec<AccountProcessingOfferRegionalRate>,
}

impl AccountProcessingOfferLevel {
    pub fn builder() -> AccountProcessingOfferLevelBuilder {
        <AccountProcessingOfferLevelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountProcessingOfferLevelBuilder {
    level: Option<i64>,
    minimum_spend: Option<Money>,
    processing_rate_percentage: Option<f64>,
    regional_rates: Option<Vec<AccountProcessingOfferRegionalRate>>,
}

impl AccountProcessingOfferLevelBuilder {
    pub fn level(mut self, value: i64) -> Self {
        self.level = Some(value);
        self
    }

    pub fn minimum_spend(mut self, value: Money) -> Self {
        self.minimum_spend = Some(value);
        self
    }

    pub fn processing_rate_percentage(mut self, value: f64) -> Self {
        self.processing_rate_percentage = Some(value);
        self
    }

    pub fn regional_rates(mut self, value: Vec<AccountProcessingOfferRegionalRate>) -> Self {
        self.regional_rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountProcessingOfferLevel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`level`](AccountProcessingOfferLevelBuilder::level)
    /// - [`minimum_spend`](AccountProcessingOfferLevelBuilder::minimum_spend)
    /// - [`processing_rate_percentage`](AccountProcessingOfferLevelBuilder::processing_rate_percentage)
    /// - [`regional_rates`](AccountProcessingOfferLevelBuilder::regional_rates)
    pub fn build(self) -> Result<AccountProcessingOfferLevel, BuildError> {
        Ok(AccountProcessingOfferLevel {
            level: self
                .level
                .ok_or_else(|| BuildError::missing_field("level"))?,
            minimum_spend: self
                .minimum_spend
                .ok_or_else(|| BuildError::missing_field("minimum_spend"))?,
            processing_rate_percentage: self
                .processing_rate_percentage
                .ok_or_else(|| BuildError::missing_field("processing_rate_percentage"))?,
            regional_rates: self
                .regional_rates
                .ok_or_else(|| BuildError::missing_field("regional_rates"))?,
        })
    }
}
