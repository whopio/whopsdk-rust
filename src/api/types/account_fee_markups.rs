pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountFeeMarkups {
    /// The markup on card purchases settled by the subaccount.
    #[serde(default)]
    pub card_spend: AccountFeeMarkup,
    /// The markup on cryptocurrency token swaps.
    #[serde(default)]
    pub crypto_swaps: AccountFeeMarkup,
    /// Markups on deposits into the account's balance, keyed by rail: `bank` and `crypto`.
    #[serde(default)]
    pub deposits: HashMap<String, AccountFeeMarkup>,
    /// Markups on payments, keyed by payment method: `card` (including Apple Pay and Google Pay), `ach` (US bank account and US bank transfer), and `wire`. A payment method without its own markup uses `payments`.
    #[serde(default)]
    pub payment_methods: HashMap<String, AccountFeeMarkup>,
    /// The markup on payments the subaccount collects without a checkout application fee, for every payment method without its own markup under `payment_methods`.
    #[serde(default)]
    pub payments: AccountFeeMarkup,
    /// Markups on withdrawals, keyed by payout method: `bank_wire`, `next_day_bank`, `rtp`, `crypto`, and `digital_wallet`.
    #[serde(default)]
    pub payouts: HashMap<String, AccountFeeMarkup>,
    /// The markup on transfers between Whop balances.
    #[serde(default)]
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
    card_spend: Option<AccountFeeMarkup>,
    crypto_swaps: Option<AccountFeeMarkup>,
    deposits: Option<HashMap<String, AccountFeeMarkup>>,
    payment_methods: Option<HashMap<String, AccountFeeMarkup>>,
    payments: Option<AccountFeeMarkup>,
    payouts: Option<HashMap<String, AccountFeeMarkup>>,
    transfers: Option<AccountFeeMarkup>,
}

impl AccountFeeMarkupsBuilder {
    pub fn card_spend(mut self, value: AccountFeeMarkup) -> Self {
        self.card_spend = Some(value);
        self
    }

    pub fn crypto_swaps(mut self, value: AccountFeeMarkup) -> Self {
        self.crypto_swaps = Some(value);
        self
    }

    pub fn deposits(mut self, value: HashMap<String, AccountFeeMarkup>) -> Self {
        self.deposits = Some(value);
        self
    }

    pub fn payment_methods(mut self, value: HashMap<String, AccountFeeMarkup>) -> Self {
        self.payment_methods = Some(value);
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
    /// - [`card_spend`](AccountFeeMarkupsBuilder::card_spend)
    /// - [`crypto_swaps`](AccountFeeMarkupsBuilder::crypto_swaps)
    /// - [`deposits`](AccountFeeMarkupsBuilder::deposits)
    /// - [`payment_methods`](AccountFeeMarkupsBuilder::payment_methods)
    /// - [`payments`](AccountFeeMarkupsBuilder::payments)
    /// - [`payouts`](AccountFeeMarkupsBuilder::payouts)
    /// - [`transfers`](AccountFeeMarkupsBuilder::transfers)
    pub fn build(self) -> Result<AccountFeeMarkups, BuildError> {
        Ok(AccountFeeMarkups {
            card_spend: self
                .card_spend
                .ok_or_else(|| BuildError::missing_field("card_spend"))?,
            crypto_swaps: self
                .crypto_swaps
                .ok_or_else(|| BuildError::missing_field("crypto_swaps"))?,
            deposits: self
                .deposits
                .ok_or_else(|| BuildError::missing_field("deposits"))?,
            payment_methods: self
                .payment_methods
                .ok_or_else(|| BuildError::missing_field("payment_methods"))?,
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
