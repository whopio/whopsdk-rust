pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountBalanceSettlement {
    /// Amount expected that day, in native units, as a decimal string.
    #[serde(default)]
    pub amount: String,
    /// The day this money is expected to finish settling, as an ISO 8601 date.
    #[serde(default)]
    pub date: String,
}

impl AccountBalanceSettlement {
    pub fn builder() -> AccountBalanceSettlementBuilder {
        <AccountBalanceSettlementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountBalanceSettlementBuilder {
    amount: Option<String>,
    date: Option<String>,
}

impl AccountBalanceSettlementBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountBalanceSettlement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](AccountBalanceSettlementBuilder::amount)
    /// - [`date`](AccountBalanceSettlementBuilder::date)
    pub fn build(self) -> Result<AccountBalanceSettlement, BuildError> {
        Ok(AccountBalanceSettlement {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
        })
    }
}
