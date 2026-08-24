pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountReserveType {
    /// Amount held for this reason, in native units, as a decimal string.
    #[serde(default)]
    pub amount: String,
    /// Days money is currently held for this reason before it unlocks, or `null` when release depends on something other than time. Money already held keeps the terms it was taken under.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_period_days: Option<i64>,
    /// Percentage of each incoming payment currently held for this reason, or `null` when the reason is not a percentage of anything. Money already held keeps the release date it was given, which `unlocks_by_date` reflects.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percentage: Option<f64>,
    /// Why this part of the balance is held. `regular` is the account's standing risk reserve; `bnpl` and `sequra` cover buy-now-pay-later settlement; `preshipment_hold` covers a physical order that has not shipped yet; `fraud_hold` is held while activity is reviewed.
    pub r#type: AccountReserveTypeType,
}

impl AccountReserveType {
    pub fn builder() -> AccountReserveTypeBuilder {
        <AccountReserveTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountReserveTypeBuilder {
    amount: Option<String>,
    hold_period_days: Option<i64>,
    percentage: Option<f64>,
    r#type: Option<AccountReserveTypeType>,
}

impl AccountReserveTypeBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn hold_period_days(mut self, value: i64) -> Self {
        self.hold_period_days = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    pub fn r#type(mut self, value: AccountReserveTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountReserveType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](AccountReserveTypeBuilder::amount)
    /// - [`r#type`](AccountReserveTypeBuilder::r#type)
    pub fn build(self) -> Result<AccountReserveType, BuildError> {
        Ok(AccountReserveType {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            hold_period_days: self.hold_period_days,
            percentage: self.percentage,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
