pub use crate::prelude::*;

/// A cached balance for a LedgerAccount in respect to a currency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LedgerAccountBalancesItem {
    /// The amount of the balance.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub balance: f64,
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
}

impl LedgerAccountBalancesItem {
    pub fn builder() -> LedgerAccountBalancesItemBuilder {
        <LedgerAccountBalancesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerAccountBalancesItemBuilder {
    balance: Option<f64>,
    currency: Option<Currencies>,
    pending_balance: Option<f64>,
    reserve_balance: Option<f64>,
}

impl LedgerAccountBalancesItemBuilder {
    pub fn balance(mut self, value: f64) -> Self {
        self.balance = Some(value);
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

    /// Consumes the builder and constructs a [`LedgerAccountBalancesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balance`](LedgerAccountBalancesItemBuilder::balance)
    /// - [`currency`](LedgerAccountBalancesItemBuilder::currency)
    /// - [`pending_balance`](LedgerAccountBalancesItemBuilder::pending_balance)
    /// - [`reserve_balance`](LedgerAccountBalancesItemBuilder::reserve_balance)
    pub fn build(self) -> Result<LedgerAccountBalancesItem, BuildError> {
        Ok(LedgerAccountBalancesItem {
            balance: self
                .balance
                .ok_or_else(|| BuildError::missing_field("balance"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            pending_balance: self
                .pending_balance
                .ok_or_else(|| BuildError::missing_field("pending_balance"))?,
            reserve_balance: self
                .reserve_balance
                .ok_or_else(|| BuildError::missing_field("reserve_balance"))?,
        })
    }
}
