pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountParentFeesValue {
    /// Fixed markup in US dollars per transaction.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub fixed_fee_usd: f64,
    /// Percentage of the transaction charged as markup.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub percentage_fee: f64,
}

impl AccountParentFeesValue {
    pub fn builder() -> AccountParentFeesValueBuilder {
        <AccountParentFeesValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountParentFeesValueBuilder {
    fixed_fee_usd: Option<f64>,
    percentage_fee: Option<f64>,
}

impl AccountParentFeesValueBuilder {
    pub fn fixed_fee_usd(mut self, value: f64) -> Self {
        self.fixed_fee_usd = Some(value);
        self
    }

    pub fn percentage_fee(mut self, value: f64) -> Self {
        self.percentage_fee = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountParentFeesValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fixed_fee_usd`](AccountParentFeesValueBuilder::fixed_fee_usd)
    /// - [`percentage_fee`](AccountParentFeesValueBuilder::percentage_fee)
    pub fn build(self) -> Result<AccountParentFeesValue, BuildError> {
        Ok(AccountParentFeesValue {
            fixed_fee_usd: self
                .fixed_fee_usd
                .ok_or_else(|| BuildError::missing_field("fixed_fee_usd"))?,
            percentage_fee: self
                .percentage_fee
                .ok_or_else(|| BuildError::missing_field("percentage_fee"))?,
        })
    }
}
