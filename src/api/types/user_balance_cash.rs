pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserBalanceCash {
    /// Available balance in the native currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub balance: f64,
    /// Available balance converted to USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub balance_usd: f64,
    /// Lowercase ISO currency code, such as `usd` or `eur`.
    #[serde(default)]
    pub currency: String,
    /// Balance moving to the user's own wallet or card, converted to USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub in_transit_balance_usd: f64,
    /// Pending balance converted to USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub pending_balance_usd: f64,
    /// USD price per native currency unit, or `null` when no exchange rate is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_usd: Option<f64>,
    /// Reserved balance converted to USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub reserve_balance_usd: f64,
    /// Withdrawable amount in the native currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_withdrawable_balance: f64,
}

impl UserBalanceCash {
    pub fn builder() -> UserBalanceCashBuilder {
        <UserBalanceCashBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBalanceCashBuilder {
    balance: Option<f64>,
    balance_usd: Option<f64>,
    currency: Option<String>,
    in_transit_balance_usd: Option<f64>,
    pending_balance_usd: Option<f64>,
    price_usd: Option<f64>,
    reserve_balance_usd: Option<f64>,
    total_withdrawable_balance: Option<f64>,
}

impl UserBalanceCashBuilder {
    pub fn balance(mut self, value: f64) -> Self {
        self.balance = Some(value);
        self
    }

    pub fn balance_usd(mut self, value: f64) -> Self {
        self.balance_usd = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn in_transit_balance_usd(mut self, value: f64) -> Self {
        self.in_transit_balance_usd = Some(value);
        self
    }

    pub fn pending_balance_usd(mut self, value: f64) -> Self {
        self.pending_balance_usd = Some(value);
        self
    }

    pub fn price_usd(mut self, value: f64) -> Self {
        self.price_usd = Some(value);
        self
    }

    pub fn reserve_balance_usd(mut self, value: f64) -> Self {
        self.reserve_balance_usd = Some(value);
        self
    }

    pub fn total_withdrawable_balance(mut self, value: f64) -> Self {
        self.total_withdrawable_balance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserBalanceCash`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balance`](UserBalanceCashBuilder::balance)
    /// - [`balance_usd`](UserBalanceCashBuilder::balance_usd)
    /// - [`currency`](UserBalanceCashBuilder::currency)
    /// - [`in_transit_balance_usd`](UserBalanceCashBuilder::in_transit_balance_usd)
    /// - [`pending_balance_usd`](UserBalanceCashBuilder::pending_balance_usd)
    /// - [`reserve_balance_usd`](UserBalanceCashBuilder::reserve_balance_usd)
    /// - [`total_withdrawable_balance`](UserBalanceCashBuilder::total_withdrawable_balance)
    pub fn build(self) -> Result<UserBalanceCash, BuildError> {
        Ok(UserBalanceCash {
            balance: self
                .balance
                .ok_or_else(|| BuildError::missing_field("balance"))?,
            balance_usd: self
                .balance_usd
                .ok_or_else(|| BuildError::missing_field("balance_usd"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            in_transit_balance_usd: self
                .in_transit_balance_usd
                .ok_or_else(|| BuildError::missing_field("in_transit_balance_usd"))?,
            pending_balance_usd: self
                .pending_balance_usd
                .ok_or_else(|| BuildError::missing_field("pending_balance_usd"))?,
            price_usd: self.price_usd,
            reserve_balance_usd: self
                .reserve_balance_usd
                .ok_or_else(|| BuildError::missing_field("reserve_balance_usd"))?,
            total_withdrawable_balance: self
                .total_withdrawable_balance
                .ok_or_else(|| BuildError::missing_field("total_withdrawable_balance"))?,
        })
    }
}
