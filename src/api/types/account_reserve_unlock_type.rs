pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountReserveUnlockType {
    /// Amount unlocking that day for this reason, in native units, as a decimal string.
    #[serde(default)]
    pub amount: String,
    /// Why this part of the day's unlock is held, matching `type` on the reserve's `by_type`.
    pub r#type: AccountReserveUnlockTypeType,
}

impl AccountReserveUnlockType {
    pub fn builder() -> AccountReserveUnlockTypeBuilder {
        <AccountReserveUnlockTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountReserveUnlockTypeBuilder {
    amount: Option<String>,
    r#type: Option<AccountReserveUnlockTypeType>,
}

impl AccountReserveUnlockTypeBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: AccountReserveUnlockTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountReserveUnlockType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](AccountReserveUnlockTypeBuilder::amount)
    /// - [`r#type`](AccountReserveUnlockTypeBuilder::r#type)
    pub fn build(self) -> Result<AccountReserveUnlockType, BuildError> {
        Ok(AccountReserveUnlockType {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
