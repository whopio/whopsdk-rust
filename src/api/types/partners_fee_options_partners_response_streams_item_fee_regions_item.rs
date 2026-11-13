pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FeeOptionsPartnersResponseStreamsItemFeeRegionsItem {
    /// The lowest fee this region may be priced down to.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub minimum_percentage: f64,
    /// The fee in force for this region.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub percentage: f64,
    /// The region this card row prices, e.g. `usa`.
    #[serde(default)]
    pub region: String,
}

impl FeeOptionsPartnersResponseStreamsItemFeeRegionsItem {
    pub fn builder() -> FeeOptionsPartnersResponseStreamsItemFeeRegionsItemBuilder {
        <FeeOptionsPartnersResponseStreamsItemFeeRegionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeeOptionsPartnersResponseStreamsItemFeeRegionsItemBuilder {
    minimum_percentage: Option<f64>,
    percentage: Option<f64>,
    region: Option<String>,
}

impl FeeOptionsPartnersResponseStreamsItemFeeRegionsItemBuilder {
    pub fn minimum_percentage(mut self, value: f64) -> Self {
        self.minimum_percentage = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FeeOptionsPartnersResponseStreamsItemFeeRegionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`minimum_percentage`](FeeOptionsPartnersResponseStreamsItemFeeRegionsItemBuilder::minimum_percentage)
    /// - [`percentage`](FeeOptionsPartnersResponseStreamsItemFeeRegionsItemBuilder::percentage)
    /// - [`region`](FeeOptionsPartnersResponseStreamsItemFeeRegionsItemBuilder::region)
    pub fn build(self) -> Result<FeeOptionsPartnersResponseStreamsItemFeeRegionsItem, BuildError> {
        Ok(FeeOptionsPartnersResponseStreamsItemFeeRegionsItem {
            minimum_percentage: self
                .minimum_percentage
                .ok_or_else(|| BuildError::missing_field("minimum_percentage"))?,
            percentage: self
                .percentage
                .ok_or_else(|| BuildError::missing_field("percentage"))?,
            region: self
                .region
                .ok_or_else(|| BuildError::missing_field("region"))?,
        })
    }
}
