pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserFees {
    /// Charged on bank deposits into the user's balance.
    pub bank_deposit: AccountFee,
    /// Fees on withdrawals, keyed by payout method: `bank_wire`, `same_day_bank`, `next_day_bank`, `rtp`, `crypto`, and `digital_wallet`.
    #[serde(default)]
    pub payouts: HashMap<String, AccountFee>,
    /// Charged when pending, not-yet-settled balance is advanced to fund the user's cards balance.
    pub pending_auto_topup: AccountFee,
    /// Charged on transfers from the user's balance to another Whop balance.
    pub transfers: AccountFee,
    /// The user these fees are charged to, prefixed `user_`.
    #[serde(default)]
    pub user_id: String,
}

impl UserFees {
    pub fn builder() -> UserFeesBuilder {
        <UserFeesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserFeesBuilder {
    bank_deposit: Option<AccountFee>,
    payouts: Option<HashMap<String, AccountFee>>,
    pending_auto_topup: Option<AccountFee>,
    transfers: Option<AccountFee>,
    user_id: Option<String>,
}

impl UserFeesBuilder {
    pub fn bank_deposit(mut self, value: AccountFee) -> Self {
        self.bank_deposit = Some(value);
        self
    }

    pub fn payouts(mut self, value: HashMap<String, AccountFee>) -> Self {
        self.payouts = Some(value);
        self
    }

    pub fn pending_auto_topup(mut self, value: AccountFee) -> Self {
        self.pending_auto_topup = Some(value);
        self
    }

    pub fn transfers(mut self, value: AccountFee) -> Self {
        self.transfers = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserFees`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_deposit`](UserFeesBuilder::bank_deposit)
    /// - [`payouts`](UserFeesBuilder::payouts)
    /// - [`pending_auto_topup`](UserFeesBuilder::pending_auto_topup)
    /// - [`transfers`](UserFeesBuilder::transfers)
    /// - [`user_id`](UserFeesBuilder::user_id)
    pub fn build(self) -> Result<UserFees, BuildError> {
        Ok(UserFees {
            bank_deposit: self
                .bank_deposit
                .ok_or_else(|| BuildError::missing_field("bank_deposit"))?,
            payouts: self
                .payouts
                .ok_or_else(|| BuildError::missing_field("payouts"))?,
            pending_auto_topup: self
                .pending_auto_topup
                .ok_or_else(|| BuildError::missing_field("pending_auto_topup"))?,
            transfers: self
                .transfers
                .ok_or_else(|| BuildError::missing_field("transfers"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
