pub use crate::prelude::*;

/// The balance cache associated with the account by currency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LedgerAccountTreasuryBalance {
    /// The amount of the balance.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub balance: f64,
    /// The balance converted to USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub balance_usd: f64,
    /// The currency of the balance.
    pub currency: Currencies,
    /// The amount of the balance that is pending.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub pending_balance: f64,
    /// The amount of the balance that is reserved.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub reserve_balance: f64,
    /// The amount of the balance that is withdrawable.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_withdrawable_balance: f64,
}

impl LedgerAccountTreasuryBalance {
    pub fn builder() -> LedgerAccountTreasuryBalanceBuilder {
        <LedgerAccountTreasuryBalanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerAccountTreasuryBalanceBuilder {
    balance: Option<f64>,
    balance_usd: Option<f64>,
    currency: Option<Currencies>,
    pending_balance: Option<f64>,
    reserve_balance: Option<f64>,
    total_withdrawable_balance: Option<f64>,
}

impl LedgerAccountTreasuryBalanceBuilder {
    pub fn balance(mut self, value: f64) -> Self {
        self.balance = Some(value);
        self
    }

    pub fn balance_usd(mut self, value: f64) -> Self {
        self.balance_usd = Some(value);
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn pending_balance(mut self, value: f64) -> Self {
        self.pending_balance = Some(value);
        self
    }

    pub fn reserve_balance(mut self, value: f64) -> Self {
        self.reserve_balance = Some(value);
        self
    }

    pub fn total_withdrawable_balance(mut self, value: f64) -> Self {
        self.total_withdrawable_balance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LedgerAccountTreasuryBalance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balance`](LedgerAccountTreasuryBalanceBuilder::balance)
    /// - [`balance_usd`](LedgerAccountTreasuryBalanceBuilder::balance_usd)
    /// - [`currency`](LedgerAccountTreasuryBalanceBuilder::currency)
    /// - [`pending_balance`](LedgerAccountTreasuryBalanceBuilder::pending_balance)
    /// - [`reserve_balance`](LedgerAccountTreasuryBalanceBuilder::reserve_balance)
    /// - [`total_withdrawable_balance`](LedgerAccountTreasuryBalanceBuilder::total_withdrawable_balance)
    pub fn build(self) -> Result<LedgerAccountTreasuryBalance, BuildError> {
        Ok(LedgerAccountTreasuryBalance {
            balance: self
                .balance
                .ok_or_else(|| BuildError::missing_field("balance"))?,
            balance_usd: self
                .balance_usd
                .ok_or_else(|| BuildError::missing_field("balance_usd"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            pending_balance: self
                .pending_balance
                .ok_or_else(|| BuildError::missing_field("pending_balance"))?,
            reserve_balance: self
                .reserve_balance
                .ok_or_else(|| BuildError::missing_field("reserve_balance"))?,
            total_withdrawable_balance: self
                .total_withdrawable_balance
                .ok_or_else(|| BuildError::missing_field("total_withdrawable_balance"))?,
        })
    }
}
