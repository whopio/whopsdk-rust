pub use crate::prelude::*;

/// The merchant fee behind this stream, or null when the business is charged nothing on it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FeeOptionsPartnersResponseStreamsItemFee {
    /// The lowest the fee may be priced down to.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub minimum_percentage: f64,
    /// The fee in force, in percent units (2.7 means 2.7%).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub percentage: f64,
    /// For a regionally priced fee, the region `percentage` and `minimum_percentage` describe, and the region an adjustment is quoted against. Null for a fee that is not priced per region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_region: Option<String>,
    /// Every region this fee is priced for, so a caller can tell which regions a given value would reach. Empty for a fee that is not priced per region.
    #[serde(default)]
    pub regions: Vec<FeeOptionsPartnersResponseStreamsItemFeeRegionsItem>,
}

impl FeeOptionsPartnersResponseStreamsItemFee {
    pub fn builder() -> FeeOptionsPartnersResponseStreamsItemFeeBuilder {
        <FeeOptionsPartnersResponseStreamsItemFeeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeeOptionsPartnersResponseStreamsItemFeeBuilder {
    minimum_percentage: Option<f64>,
    percentage: Option<f64>,
    reference_region: Option<String>,
    regions: Option<Vec<FeeOptionsPartnersResponseStreamsItemFeeRegionsItem>>,
}

impl FeeOptionsPartnersResponseStreamsItemFeeBuilder {
    pub fn minimum_percentage(mut self, value: f64) -> Self {
        self.minimum_percentage = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    pub fn reference_region(mut self, value: impl Into<String>) -> Self {
        self.reference_region = Some(value.into());
        self
    }

    pub fn regions(
        mut self,
        value: Vec<FeeOptionsPartnersResponseStreamsItemFeeRegionsItem>,
    ) -> Self {
        self.regions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeeOptionsPartnersResponseStreamsItemFee`].
    /// This method will fail if any of the following fields are not set:
    /// - [`minimum_percentage`](FeeOptionsPartnersResponseStreamsItemFeeBuilder::minimum_percentage)
    /// - [`percentage`](FeeOptionsPartnersResponseStreamsItemFeeBuilder::percentage)
    /// - [`regions`](FeeOptionsPartnersResponseStreamsItemFeeBuilder::regions)
    pub fn build(self) -> Result<FeeOptionsPartnersResponseStreamsItemFee, BuildError> {
        Ok(FeeOptionsPartnersResponseStreamsItemFee {
            minimum_percentage: self
                .minimum_percentage
                .ok_or_else(|| BuildError::missing_field("minimum_percentage"))?,
            percentage: self
                .percentage
                .ok_or_else(|| BuildError::missing_field("percentage"))?,
            reference_region: self.reference_region,
            regions: self
                .regions
                .ok_or_else(|| BuildError::missing_field("regions"))?,
        })
    }
}
