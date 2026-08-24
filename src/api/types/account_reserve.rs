pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountReserve {
    /// Total held in this currency, in native units, as a decimal string. `usd` and `usdt` are reported as one `usd` entry, matching how the balance row groups them.
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub by_type: Vec<AccountReserveType>,
    /// Lowercase ISO currency code, such as `usd` or `eur`.
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub unlocks_by_date: Vec<AccountReserveUnlock>,
}

impl AccountReserve {
    pub fn builder() -> AccountReserveBuilder {
        <AccountReserveBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountReserveBuilder {
    amount: Option<String>,
    by_type: Option<Vec<AccountReserveType>>,
    currency: Option<String>,
    unlocks_by_date: Option<Vec<AccountReserveUnlock>>,
}

impl AccountReserveBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn by_type(mut self, value: Vec<AccountReserveType>) -> Self {
        self.by_type = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn unlocks_by_date(mut self, value: Vec<AccountReserveUnlock>) -> Self {
        self.unlocks_by_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountReserve`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](AccountReserveBuilder::amount)
    /// - [`by_type`](AccountReserveBuilder::by_type)
    /// - [`currency`](AccountReserveBuilder::currency)
    /// - [`unlocks_by_date`](AccountReserveBuilder::unlocks_by_date)
    pub fn build(self) -> Result<AccountReserve, BuildError> {
        Ok(AccountReserve {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            by_type: self
                .by_type
                .ok_or_else(|| BuildError::missing_field("by_type"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            unlocks_by_date: self
                .unlocks_by_date
                .ok_or_else(|| BuildError::missing_field("unlocks_by_date"))?,
        })
    }
}
