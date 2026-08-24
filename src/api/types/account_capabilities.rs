pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountCapabilities {
    /// Bank payins: debits, transfers, and local bank rails
    pub accept_bank_payments: AccountCapabilitiesAcceptBankPayments,
    /// Buy-now-pay-later payins; requires approval
    pub accept_bnpl_payments: AccountCapabilitiesAcceptBnplPayments,
    /// Card payins, including Apple Pay and Google Pay
    pub accept_card_payments: AccountCapabilitiesAcceptCardPayments,
    /// Deposits by bank wire or ACH to the account's virtual bank account
    pub bank_deposit: AccountCapabilitiesBankDeposit,
    /// Balance top-ups by charging a stored payment method
    pub card_deposit: AccountCapabilitiesCardDeposit,
    /// Issuing Whop cards; requires card application approval
    pub card_issuing: AccountCapabilitiesCardIssuing,
    /// On-chain deposits to the account's crypto wallet
    pub crypto_deposit: AccountCapabilitiesCryptoDeposit,
    /// On-chain payouts to a crypto wallet
    pub crypto_payout: AccountCapabilitiesCryptoPayout,
    /// Instant payouts to an eligible payout destination
    pub instant_payout: AccountCapabilitiesInstantPayout,
    /// Launching ad campaigns through Whop Ads. `inactive` while a requested ads services agreement is awaiting the account's signature.
    pub run_ads: AccountCapabilitiesRunAds,
    /// Standard payouts to an external payout destination
    pub standard_payout: AccountCapabilitiesStandardPayout,
    /// Transfers to other accounts
    pub transfer: AccountCapabilitiesTransfer,
}

impl AccountCapabilities {
    pub fn builder() -> AccountCapabilitiesBuilder {
        <AccountCapabilitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountCapabilitiesBuilder {
    accept_bank_payments: Option<AccountCapabilitiesAcceptBankPayments>,
    accept_bnpl_payments: Option<AccountCapabilitiesAcceptBnplPayments>,
    accept_card_payments: Option<AccountCapabilitiesAcceptCardPayments>,
    bank_deposit: Option<AccountCapabilitiesBankDeposit>,
    card_deposit: Option<AccountCapabilitiesCardDeposit>,
    card_issuing: Option<AccountCapabilitiesCardIssuing>,
    crypto_deposit: Option<AccountCapabilitiesCryptoDeposit>,
    crypto_payout: Option<AccountCapabilitiesCryptoPayout>,
    instant_payout: Option<AccountCapabilitiesInstantPayout>,
    run_ads: Option<AccountCapabilitiesRunAds>,
    standard_payout: Option<AccountCapabilitiesStandardPayout>,
    transfer: Option<AccountCapabilitiesTransfer>,
}

impl AccountCapabilitiesBuilder {
    pub fn accept_bank_payments(mut self, value: AccountCapabilitiesAcceptBankPayments) -> Self {
        self.accept_bank_payments = Some(value);
        self
    }

    pub fn accept_bnpl_payments(mut self, value: AccountCapabilitiesAcceptBnplPayments) -> Self {
        self.accept_bnpl_payments = Some(value);
        self
    }

    pub fn accept_card_payments(mut self, value: AccountCapabilitiesAcceptCardPayments) -> Self {
        self.accept_card_payments = Some(value);
        self
    }

    pub fn bank_deposit(mut self, value: AccountCapabilitiesBankDeposit) -> Self {
        self.bank_deposit = Some(value);
        self
    }

    pub fn card_deposit(mut self, value: AccountCapabilitiesCardDeposit) -> Self {
        self.card_deposit = Some(value);
        self
    }

    pub fn card_issuing(mut self, value: AccountCapabilitiesCardIssuing) -> Self {
        self.card_issuing = Some(value);
        self
    }

    pub fn crypto_deposit(mut self, value: AccountCapabilitiesCryptoDeposit) -> Self {
        self.crypto_deposit = Some(value);
        self
    }

    pub fn crypto_payout(mut self, value: AccountCapabilitiesCryptoPayout) -> Self {
        self.crypto_payout = Some(value);
        self
    }

    pub fn instant_payout(mut self, value: AccountCapabilitiesInstantPayout) -> Self {
        self.instant_payout = Some(value);
        self
    }

    pub fn run_ads(mut self, value: AccountCapabilitiesRunAds) -> Self {
        self.run_ads = Some(value);
        self
    }

    pub fn standard_payout(mut self, value: AccountCapabilitiesStandardPayout) -> Self {
        self.standard_payout = Some(value);
        self
    }

    pub fn transfer(mut self, value: AccountCapabilitiesTransfer) -> Self {
        self.transfer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountCapabilities`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accept_bank_payments`](AccountCapabilitiesBuilder::accept_bank_payments)
    /// - [`accept_bnpl_payments`](AccountCapabilitiesBuilder::accept_bnpl_payments)
    /// - [`accept_card_payments`](AccountCapabilitiesBuilder::accept_card_payments)
    /// - [`bank_deposit`](AccountCapabilitiesBuilder::bank_deposit)
    /// - [`card_deposit`](AccountCapabilitiesBuilder::card_deposit)
    /// - [`card_issuing`](AccountCapabilitiesBuilder::card_issuing)
    /// - [`crypto_deposit`](AccountCapabilitiesBuilder::crypto_deposit)
    /// - [`crypto_payout`](AccountCapabilitiesBuilder::crypto_payout)
    /// - [`instant_payout`](AccountCapabilitiesBuilder::instant_payout)
    /// - [`run_ads`](AccountCapabilitiesBuilder::run_ads)
    /// - [`standard_payout`](AccountCapabilitiesBuilder::standard_payout)
    /// - [`transfer`](AccountCapabilitiesBuilder::transfer)
    pub fn build(self) -> Result<AccountCapabilities, BuildError> {
        Ok(AccountCapabilities {
            accept_bank_payments: self
                .accept_bank_payments
                .ok_or_else(|| BuildError::missing_field("accept_bank_payments"))?,
            accept_bnpl_payments: self
                .accept_bnpl_payments
                .ok_or_else(|| BuildError::missing_field("accept_bnpl_payments"))?,
            accept_card_payments: self
                .accept_card_payments
                .ok_or_else(|| BuildError::missing_field("accept_card_payments"))?,
            bank_deposit: self
                .bank_deposit
                .ok_or_else(|| BuildError::missing_field("bank_deposit"))?,
            card_deposit: self
                .card_deposit
                .ok_or_else(|| BuildError::missing_field("card_deposit"))?,
            card_issuing: self
                .card_issuing
                .ok_or_else(|| BuildError::missing_field("card_issuing"))?,
            crypto_deposit: self
                .crypto_deposit
                .ok_or_else(|| BuildError::missing_field("crypto_deposit"))?,
            crypto_payout: self
                .crypto_payout
                .ok_or_else(|| BuildError::missing_field("crypto_payout"))?,
            instant_payout: self
                .instant_payout
                .ok_or_else(|| BuildError::missing_field("instant_payout"))?,
            run_ads: self
                .run_ads
                .ok_or_else(|| BuildError::missing_field("run_ads"))?,
            standard_payout: self
                .standard_payout
                .ok_or_else(|| BuildError::missing_field("standard_payout"))?,
            transfer: self
                .transfer
                .ok_or_else(|| BuildError::missing_field("transfer"))?,
        })
    }
}
