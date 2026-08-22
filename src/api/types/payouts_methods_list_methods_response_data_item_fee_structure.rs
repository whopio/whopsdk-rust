pub use crate::prelude::*;

/// Configured fee terms for this payout method. Null when the method is not currently eligible. An amount-specific quote remains authoritative.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMethodsResponseDataItemFeeStructure {
    /// Currency code of fixed_amount.
    #[serde(default)]
    pub currency: String,
    /// Fixed fee charged, denominated in `currency`.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub fixed_amount: f64,
    /// Percentage of the withdrawal amount charged as a fee.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub percentage: f64,
}

impl ListMethodsResponseDataItemFeeStructure {
    pub fn builder() -> ListMethodsResponseDataItemFeeStructureBuilder {
        <ListMethodsResponseDataItemFeeStructureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMethodsResponseDataItemFeeStructureBuilder {
    currency: Option<String>,
    fixed_amount: Option<f64>,
    percentage: Option<f64>,
}

impl ListMethodsResponseDataItemFeeStructureBuilder {
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

    /// Consumes the builder and constructs a [`ListMethodsResponseDataItemFeeStructure`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](ListMethodsResponseDataItemFeeStructureBuilder::currency)
    /// - [`fixed_amount`](ListMethodsResponseDataItemFeeStructureBuilder::fixed_amount)
    /// - [`percentage`](ListMethodsResponseDataItemFeeStructureBuilder::percentage)
    pub fn build(self) -> Result<ListMethodsResponseDataItemFeeStructure, BuildError> {
        Ok(ListMethodsResponseDataItemFeeStructure {
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
