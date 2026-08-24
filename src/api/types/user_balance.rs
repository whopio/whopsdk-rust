pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserBalance {
    #[serde(default)]
    pub businesses: Vec<UserBalanceBusiness>,
    /// Combined USD balance across every account the user owns.
    #[serde(default)]
    pub businesses_total_usd: String,
    #[serde(default)]
    pub cash: Vec<UserBalanceCash>,
    /// Fiat cash in USD, including pending, in-transit, and reserve.
    #[serde(default)]
    pub cash_usd: String,
    #[serde(default)]
    pub crypto: Vec<UserBalanceToken>,
    /// Crypto holdings in USD.
    #[serde(default)]
    pub crypto_usd: String,
    /// Fiat pending and in-transit balances, plus in-flight treasury deposits, in USD.
    #[serde(default)]
    pub pending_usd: String,
    /// The user's personal balance in USD: cash (available + pending + in-transit + reserve) + crypto + in-flight treasury deposits. Excludes account balances (see businesses_total_usd).
    #[serde(default)]
    pub total_usd: String,
    /// Balance-to-wallet USDT0 withdrawals still in flight, in USD.
    #[serde(default)]
    pub treasury_pending_usd: String,
}

impl UserBalance {
    pub fn builder() -> UserBalanceBuilder {
        <UserBalanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBalanceBuilder {
    businesses: Option<Vec<UserBalanceBusiness>>,
    businesses_total_usd: Option<String>,
    cash: Option<Vec<UserBalanceCash>>,
    cash_usd: Option<String>,
    crypto: Option<Vec<UserBalanceToken>>,
    crypto_usd: Option<String>,
    pending_usd: Option<String>,
    total_usd: Option<String>,
    treasury_pending_usd: Option<String>,
}

impl UserBalanceBuilder {
    pub fn businesses(mut self, value: Vec<UserBalanceBusiness>) -> Self {
        self.businesses = Some(value);
        self
    }

    pub fn businesses_total_usd(mut self, value: impl Into<String>) -> Self {
        self.businesses_total_usd = Some(value.into());
        self
    }

    pub fn cash(mut self, value: Vec<UserBalanceCash>) -> Self {
        self.cash = Some(value);
        self
    }

    pub fn cash_usd(mut self, value: impl Into<String>) -> Self {
        self.cash_usd = Some(value.into());
        self
    }

    pub fn crypto(mut self, value: Vec<UserBalanceToken>) -> Self {
        self.crypto = Some(value);
        self
    }

    pub fn crypto_usd(mut self, value: impl Into<String>) -> Self {
        self.crypto_usd = Some(value.into());
        self
    }

    pub fn pending_usd(mut self, value: impl Into<String>) -> Self {
        self.pending_usd = Some(value.into());
        self
    }

    pub fn total_usd(mut self, value: impl Into<String>) -> Self {
        self.total_usd = Some(value.into());
        self
    }

    pub fn treasury_pending_usd(mut self, value: impl Into<String>) -> Self {
        self.treasury_pending_usd = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserBalance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`businesses`](UserBalanceBuilder::businesses)
    /// - [`businesses_total_usd`](UserBalanceBuilder::businesses_total_usd)
    /// - [`cash`](UserBalanceBuilder::cash)
    /// - [`cash_usd`](UserBalanceBuilder::cash_usd)
    /// - [`crypto`](UserBalanceBuilder::crypto)
    /// - [`crypto_usd`](UserBalanceBuilder::crypto_usd)
    /// - [`pending_usd`](UserBalanceBuilder::pending_usd)
    /// - [`total_usd`](UserBalanceBuilder::total_usd)
    /// - [`treasury_pending_usd`](UserBalanceBuilder::treasury_pending_usd)
    pub fn build(self) -> Result<UserBalance, BuildError> {
        Ok(UserBalance {
            businesses: self
                .businesses
                .ok_or_else(|| BuildError::missing_field("businesses"))?,
            businesses_total_usd: self
                .businesses_total_usd
                .ok_or_else(|| BuildError::missing_field("businesses_total_usd"))?,
            cash: self.cash.ok_or_else(|| BuildError::missing_field("cash"))?,
            cash_usd: self
                .cash_usd
                .ok_or_else(|| BuildError::missing_field("cash_usd"))?,
            crypto: self
                .crypto
                .ok_or_else(|| BuildError::missing_field("crypto"))?,
            crypto_usd: self
                .crypto_usd
                .ok_or_else(|| BuildError::missing_field("crypto_usd"))?,
            pending_usd: self
                .pending_usd
                .ok_or_else(|| BuildError::missing_field("pending_usd"))?,
            total_usd: self
                .total_usd
                .ok_or_else(|| BuildError::missing_field("total_usd"))?,
            treasury_pending_usd: self
                .treasury_pending_usd
                .ok_or_else(|| BuildError::missing_field("treasury_pending_usd"))?,
        })
    }
}
