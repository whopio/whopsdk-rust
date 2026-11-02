pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountFeeMarkups {
    /// The markup on cryptocurrency token swaps.
    pub crypto_swaps: AccountFeeMarkup,
    /// Markups on deposits into the account's balance, keyed by rail: `bank` and `crypto`.
    #[serde(default)]
    pub deposits: HashMap<String, AccountFeeMarkup>,
    /// The markup on payments the connected account collects without a checkout application fee.
    pub payments: AccountFeeMarkup,
    /// Markups on withdrawals, keyed by payout method: `bank_wire`, `next_day_bank`, `rtp`, `crypto`, and `digital_wallet`.
    #[serde(default)]
    pub payouts: HashMap<String, AccountFeeMarkup>,
    /// The markup on transfers between Whop balances.
    pub transfers: AccountFeeMarkup,
}

impl AccountFeeMarkups {
    pub fn builder() -> AccountFeeMarkupsBuilder {
        <AccountFeeMarkupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountFeeMarkupsBuilder {
    crypto_swaps: Option<AccountFeeMarkup>,
    deposits: Option<HashMap<String, AccountFeeMarkup>>,
    payments: Option<AccountFeeMarkup>,
    payouts: Option<HashMap<String, AccountFeeMarkup>>,
    transfers: Option<AccountFeeMarkup>,
}

impl AccountFeeMarkupsBuilder {
    pub fn crypto_swaps(mut self, value: AccountFeeMarkup) -> Self {
        self.crypto_swaps = Some(value);
        self
    }

    pub fn deposits(mut self, value: HashMap<String, AccountFeeMarkup>) -> Self {
        self.deposits = Some(value);
        self
    }

    pub fn payments(mut self, value: AccountFeeMarkup) -> Self {
        self.payments = Some(value);
        self
    }

    pub fn payouts(mut self, value: HashMap<String, AccountFeeMarkup>) -> Self {
        self.payouts = Some(value);
        self
    }

    pub fn transfers(mut self, value: AccountFeeMarkup) -> Self {
        self.transfers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountFeeMarkups`].
    /// This method will fail if any of the following fields are not set:
    /// - [`crypto_swaps`](AccountFeeMarkupsBuilder::crypto_swaps)
    /// - [`deposits`](AccountFeeMarkupsBuilder::deposits)
    /// - [`payments`](AccountFeeMarkupsBuilder::payments)
    /// - [`payouts`](AccountFeeMarkupsBuilder::payouts)
    /// - [`transfers`](AccountFeeMarkupsBuilder::transfers)
    pub fn build(self) -> Result<AccountFeeMarkups, BuildError> {
        Ok(AccountFeeMarkups {
            crypto_swaps: self
                .crypto_swaps
                .ok_or_else(|| BuildError::missing_field("crypto_swaps"))?,
            deposits: self
                .deposits
                .ok_or_else(|| BuildError::missing_field("deposits"))?,
            payments: self
                .payments
                .ok_or_else(|| BuildError::missing_field("payments"))?,
            payouts: self
                .payouts
                .ok_or_else(|| BuildError::missing_field("payouts"))?,
            transfers: self
                .transfers
                .ok_or_else(|| BuildError::missing_field("transfers"))?,
        })
    }
}
