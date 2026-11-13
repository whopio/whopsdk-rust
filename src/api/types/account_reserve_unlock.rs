pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountReserveUnlock {
    /// Amount unlocking that day across every reason, in native units, as a decimal string.
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub by_type: Vec<AccountReserveUnlockType>,
    /// The day this money unlocks, as an ISO 8601 date.
    #[serde(default)]
    pub date: String,
}

impl AccountReserveUnlock {
    pub fn builder() -> AccountReserveUnlockBuilder {
        <AccountReserveUnlockBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountReserveUnlockBuilder {
    amount: Option<String>,
    by_type: Option<Vec<AccountReserveUnlockType>>,
    date: Option<String>,
}

impl AccountReserveUnlockBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn by_type(mut self, value: Vec<AccountReserveUnlockType>) -> Self {
        self.by_type = Some(value);
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountReserveUnlock`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](AccountReserveUnlockBuilder::amount)
    /// - [`by_type`](AccountReserveUnlockBuilder::by_type)
    /// - [`date`](AccountReserveUnlockBuilder::date)
    pub fn build(self) -> Result<AccountReserveUnlock, BuildError> {
        Ok(AccountReserveUnlock {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            by_type: self
                .by_type
                .ok_or_else(|| BuildError::missing_field("by_type"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
        })
    }
}
