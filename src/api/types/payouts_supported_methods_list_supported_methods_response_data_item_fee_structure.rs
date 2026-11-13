pub use crate::prelude::*;

/// Fee terms for payouts to this method: a percentage of the payout amount plus a fixed fee. The quotes carry the exact fee for a specific amount.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSupportedMethodsResponseDataItemFeeStructure {
    /// Fixed fee charged on every payout, on top of the percentage.
    #[serde(default)]
    pub fixed: Money,
    /// Percentage of the payout amount charged as a fee.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub percentage: f64,
}

impl ListSupportedMethodsResponseDataItemFeeStructure {
    pub fn builder() -> ListSupportedMethodsResponseDataItemFeeStructureBuilder {
        <ListSupportedMethodsResponseDataItemFeeStructureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSupportedMethodsResponseDataItemFeeStructureBuilder {
    fixed: Option<Money>,
    percentage: Option<f64>,
}

impl ListSupportedMethodsResponseDataItemFeeStructureBuilder {
    pub fn fixed(mut self, value: Money) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSupportedMethodsResponseDataItemFeeStructure`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fixed`](ListSupportedMethodsResponseDataItemFeeStructureBuilder::fixed)
    /// - [`percentage`](ListSupportedMethodsResponseDataItemFeeStructureBuilder::percentage)
    pub fn build(self) -> Result<ListSupportedMethodsResponseDataItemFeeStructure, BuildError> {
        Ok(ListSupportedMethodsResponseDataItemFeeStructure {
            fixed: self
                .fixed
                .ok_or_else(|| BuildError::missing_field("fixed"))?,
            percentage: self
                .percentage
                .ok_or_else(|| BuildError::missing_field("percentage"))?,
        })
    }
}
