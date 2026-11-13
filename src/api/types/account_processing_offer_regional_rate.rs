pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountProcessingOfferRegionalRate {
    /// The standard card processing percentage at this level for cards from this region, regardless of the account's custom rate.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub processing_rate_percentage: f64,
    /// The card region this fee applies to.
    pub region: AccountProcessingOfferRegionalRateRegion,
}

impl AccountProcessingOfferRegionalRate {
    pub fn builder() -> AccountProcessingOfferRegionalRateBuilder {
        <AccountProcessingOfferRegionalRateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountProcessingOfferRegionalRateBuilder {
    processing_rate_percentage: Option<f64>,
    region: Option<AccountProcessingOfferRegionalRateRegion>,
}

impl AccountProcessingOfferRegionalRateBuilder {
    pub fn processing_rate_percentage(mut self, value: f64) -> Self {
        self.processing_rate_percentage = Some(value);
        self
    }

    pub fn region(mut self, value: AccountProcessingOfferRegionalRateRegion) -> Self {
        self.region = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountProcessingOfferRegionalRate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`processing_rate_percentage`](AccountProcessingOfferRegionalRateBuilder::processing_rate_percentage)
    /// - [`region`](AccountProcessingOfferRegionalRateBuilder::region)
    pub fn build(self) -> Result<AccountProcessingOfferRegionalRate, BuildError> {
        Ok(AccountProcessingOfferRegionalRate {
            processing_rate_percentage: self
                .processing_rate_percentage
                .ok_or_else(|| BuildError::missing_field("processing_rate_percentage"))?,
            region: self
                .region
                .ok_or_else(|| BuildError::missing_field("region"))?,
        })
    }
}
