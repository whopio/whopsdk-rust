pub use crate::prelude::*;

/// Configured fee terms for this payout method. Null when the method is not currently eligible. An amount-specific quote remains authoritative.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostPayoutMethodCreatedPayloadDataFeeStructure {
    /// Currency code of fixed_amount.
    #[serde(default)]
    pub currency: String,
    /// Fixed fee charged, denominated in `currency`.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub fixed_amount: f64,
    /// Percentage of the payout amount charged as a fee.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub percentage: f64,
}

impl PostPayoutMethodCreatedPayloadDataFeeStructure {
    pub fn builder() -> PostPayoutMethodCreatedPayloadDataFeeStructureBuilder {
        <PostPayoutMethodCreatedPayloadDataFeeStructureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostPayoutMethodCreatedPayloadDataFeeStructureBuilder {
    currency: Option<String>,
    fixed_amount: Option<f64>,
    percentage: Option<f64>,
}

impl PostPayoutMethodCreatedPayloadDataFeeStructureBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn fixed_amount(mut self, value: f64) -> Self {
        self.fixed_amount = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostPayoutMethodCreatedPayloadDataFeeStructure`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](PostPayoutMethodCreatedPayloadDataFeeStructureBuilder::currency)
    /// - [`fixed_amount`](PostPayoutMethodCreatedPayloadDataFeeStructureBuilder::fixed_amount)
    /// - [`percentage`](PostPayoutMethodCreatedPayloadDataFeeStructureBuilder::percentage)
    pub fn build(self) -> Result<PostPayoutMethodCreatedPayloadDataFeeStructure, BuildError> {
        Ok(PostPayoutMethodCreatedPayloadDataFeeStructure {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            fixed_amount: self
                .fixed_amount
                .ok_or_else(|| BuildError::missing_field("fixed_amount"))?,
            percentage: self
                .percentage
                .ok_or_else(|| BuildError::missing_field("percentage"))?,
        })
    }
}
