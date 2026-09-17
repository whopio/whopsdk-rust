pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountFeeRate {
    /// The amount charged per event. `null` when the fee has no fixed component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<Money>,
    /// The percentage of the transaction, where `2` means 2%. `null` when the fee has no percentage component.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percentage: Option<f64>,
}

impl AccountFeeRate {
    pub fn builder() -> AccountFeeRateBuilder {
        <AccountFeeRateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountFeeRateBuilder {
    fixed: Option<Money>,
    percentage: Option<f64>,
}

impl AccountFeeRateBuilder {
    pub fn fixed(mut self, value: Money) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountFeeRate`].
    pub fn build(self) -> Result<AccountFeeRate, BuildError> {
        Ok(AccountFeeRate {
            fixed: self.fixed,
            percentage: self.percentage,
        })
    }
}
