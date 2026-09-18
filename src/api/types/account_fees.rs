pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountFees {
    /// The account these fees are charged to, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Charged on bank deposits into the account's balance.
    pub bank_deposit: AccountFee,
    /// Charged on recurring billing.
    pub billing: AccountFee,
    /// Charged to the buyer at checkout, on top of the price.
    pub buyer: AccountFee,
    /// Card payments. `percentage` and `fixed` are the rate in the headline `region`; every other acquirer region is under `regions`.
    pub card_processing: AccountFee,
    /// The default markups this account charges the accounts connected to it. `null` unless the account is a platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_markups: Option<AccountFeeMarkups>,
    /// Added to a payment whose card was issued outside the region where the payment was processed.
    pub cross_border: AccountFee,
    /// Charged when a payment is disputed.
    pub dispute: AccountFee,
    /// Charged when an early dispute alert lets Whop refund a payment before it becomes a dispute.
    pub dispute_alert: AccountFee,
    /// The early dispute alert fee when the alert comes through Verifi CDRN.
    pub dispute_alert_cdrn: AccountFee,
    /// The early dispute alert fee when the alert comes through Ethoca.
    pub dispute_alert_ethoca: AccountFee,
    /// The early dispute alert fee when the alert comes through Verifi RDR.
    pub dispute_alert_rdr: AccountFee,
    /// Charged on the amount recovered when Whop fights a dispute and wins.
    pub dispute_representment: AccountFee,
    /// Added to a payment settled in a currency other than the one it was charged in.
    pub foreign_exchange: AccountFee,
    /// Charged when a payment is screened for fraud.
    pub fraud_screening: AccountFee,
    /// Added to every payment while the account is classed as high risk.
    pub high_risk: AccountFee,
    /// Charged on payments attributed to the Whop marketplace.
    pub marketplace: AccountFee,
    /// What the platform this account is connected to adds on top of Whop's fees, collected by that platform. `null` unless the account has a parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markups: Option<AccountFeeMarkups>,
    /// Charged on payments routed through Whop's payment orchestration.
    pub orchestration: AccountFee,
    /// The platform this account is connected to, whose markups appear under `markups`. `null` for a standalone account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_account_id: Option<String>,
    /// Processing fees for every non-card payment method the platform prices, keyed by payment method type such as `us_bank_account` or `klarna`.
    #[serde(default)]
    pub payment_methods: HashMap<String, AccountFee>,
    /// Fees on withdrawals, keyed by payout method: `bank_wire`, `same_day_bank`, `next_day_bank`, `rtp`, `crypto`, and `digital_wallet`.
    #[serde(default)]
    pub payouts: HashMap<String, AccountFee>,
    /// Charged on a Whop Ads auto top-up that is funded from pending balance.
    pub pending_auto_topup: AccountFee,
    /// Whop's share of every payment, on top of card processing.
    pub platform_processing: AccountFee,
    /// Charged on payouts from a shared pool balance.
    pub pool_payout: AccountFee,
    /// Charged on revenue shared with the account.
    pub revshare: AccountFee,
    /// Charged when tax is calculated on a payment.
    pub tax_calculation: AccountFee,
    /// Charged when Whop collects and remits tax on the account's behalf.
    pub tax_service: AccountFee,
    /// Charged when a payment is authenticated with 3-D Secure.
    pub three_ds: AccountFee,
    /// Charged on transfers from the account's balance to another Whop balance.
    pub transfers: AccountFee,
}

impl AccountFees {
    pub fn builder() -> AccountFeesBuilder {
        <AccountFeesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountFeesBuilder {
    account_id: Option<String>,
    bank_deposit: Option<AccountFee>,
    billing: Option<AccountFee>,
    buyer: Option<AccountFee>,
    card_processing: Option<AccountFee>,
    child_markups: Option<AccountFeeMarkups>,
    cross_border: Option<AccountFee>,
    dispute: Option<AccountFee>,
    dispute_alert: Option<AccountFee>,
    dispute_alert_cdrn: Option<AccountFee>,
    dispute_alert_ethoca: Option<AccountFee>,
    dispute_alert_rdr: Option<AccountFee>,
    dispute_representment: Option<AccountFee>,
    foreign_exchange: Option<AccountFee>,
    fraud_screening: Option<AccountFee>,
    high_risk: Option<AccountFee>,
    marketplace: Option<AccountFee>,
    markups: Option<AccountFeeMarkups>,
    orchestration: Option<AccountFee>,
    parent_account_id: Option<String>,
    payment_methods: Option<HashMap<String, AccountFee>>,
    payouts: Option<HashMap<String, AccountFee>>,
    pending_auto_topup: Option<AccountFee>,
    platform_processing: Option<AccountFee>,
    pool_payout: Option<AccountFee>,
    revshare: Option<AccountFee>,
    tax_calculation: Option<AccountFee>,
    tax_service: Option<AccountFee>,
    three_ds: Option<AccountFee>,
    transfers: Option<AccountFee>,
}

impl AccountFeesBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn bank_deposit(mut self, value: AccountFee) -> Self {
        self.bank_deposit = Some(value);
        self
    }

    pub fn billing(mut self, value: AccountFee) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn buyer(mut self, value: AccountFee) -> Self {
        self.buyer = Some(value);
        self
    }

    pub fn card_processing(mut self, value: AccountFee) -> Self {
        self.card_processing = Some(value);
        self
    }

    pub fn child_markups(mut self, value: AccountFeeMarkups) -> Self {
        self.child_markups = Some(value);
        self
    }

    pub fn cross_border(mut self, value: AccountFee) -> Self {
        self.cross_border = Some(value);
        self
    }

    pub fn dispute(mut self, value: AccountFee) -> Self {
        self.dispute = Some(value);
        self
    }

    pub fn dispute_alert(mut self, value: AccountFee) -> Self {
        self.dispute_alert = Some(value);
        self
    }

    pub fn dispute_alert_cdrn(mut self, value: AccountFee) -> Self {
        self.dispute_alert_cdrn = Some(value);
        self
    }

    pub fn dispute_alert_ethoca(mut self, value: AccountFee) -> Self {
        self.dispute_alert_ethoca = Some(value);
        self
    }

    pub fn dispute_alert_rdr(mut self, value: AccountFee) -> Self {
        self.dispute_alert_rdr = Some(value);
        self
    }

    pub fn dispute_representment(mut self, value: AccountFee) -> Self {
        self.dispute_representment = Some(value);
        self
    }

    pub fn foreign_exchange(mut self, value: AccountFee) -> Self {
        self.foreign_exchange = Some(value);
        self
    }

    pub fn fraud_screening(mut self, value: AccountFee) -> Self {
        self.fraud_screening = Some(value);
        self
    }

    pub fn high_risk(mut self, value: AccountFee) -> Self {
        self.high_risk = Some(value);
        self
    }

    pub fn marketplace(mut self, value: AccountFee) -> Self {
        self.marketplace = Some(value);
        self
    }

    pub fn markups(mut self, value: AccountFeeMarkups) -> Self {
        self.markups = Some(value);
        self
    }

    pub fn orchestration(mut self, value: AccountFee) -> Self {
        self.orchestration = Some(value);
        self
    }

    pub fn parent_account_id(mut self, value: impl Into<String>) -> Self {
        self.parent_account_id = Some(value.into());
        self
    }

    pub fn payment_methods(mut self, value: HashMap<String, AccountFee>) -> Self {
        self.payment_methods = Some(value);
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

    pub fn platform_processing(mut self, value: AccountFee) -> Self {
        self.platform_processing = Some(value);
        self
    }

    pub fn pool_payout(mut self, value: AccountFee) -> Self {
        self.pool_payout = Some(value);
        self
    }

    pub fn revshare(mut self, value: AccountFee) -> Self {
        self.revshare = Some(value);
        self
    }

    pub fn tax_calculation(mut self, value: AccountFee) -> Self {
        self.tax_calculation = Some(value);
        self
    }

    pub fn tax_service(mut self, value: AccountFee) -> Self {
        self.tax_service = Some(value);
        self
    }

    pub fn three_ds(mut self, value: AccountFee) -> Self {
        self.three_ds = Some(value);
        self
    }

    pub fn transfers(mut self, value: AccountFee) -> Self {
        self.transfers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountFees`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](AccountFeesBuilder::account_id)
    /// - [`bank_deposit`](AccountFeesBuilder::bank_deposit)
    /// - [`billing`](AccountFeesBuilder::billing)
    /// - [`buyer`](AccountFeesBuilder::buyer)
    /// - [`card_processing`](AccountFeesBuilder::card_processing)
    /// - [`cross_border`](AccountFeesBuilder::cross_border)
    /// - [`dispute`](AccountFeesBuilder::dispute)
    /// - [`dispute_alert`](AccountFeesBuilder::dispute_alert)
    /// - [`dispute_alert_cdrn`](AccountFeesBuilder::dispute_alert_cdrn)
    /// - [`dispute_alert_ethoca`](AccountFeesBuilder::dispute_alert_ethoca)
    /// - [`dispute_alert_rdr`](AccountFeesBuilder::dispute_alert_rdr)
    /// - [`dispute_representment`](AccountFeesBuilder::dispute_representment)
    /// - [`foreign_exchange`](AccountFeesBuilder::foreign_exchange)
    /// - [`fraud_screening`](AccountFeesBuilder::fraud_screening)
    /// - [`high_risk`](AccountFeesBuilder::high_risk)
    /// - [`marketplace`](AccountFeesBuilder::marketplace)
    /// - [`orchestration`](AccountFeesBuilder::orchestration)
    /// - [`payment_methods`](AccountFeesBuilder::payment_methods)
    /// - [`payouts`](AccountFeesBuilder::payouts)
    /// - [`pending_auto_topup`](AccountFeesBuilder::pending_auto_topup)
    /// - [`platform_processing`](AccountFeesBuilder::platform_processing)
    /// - [`pool_payout`](AccountFeesBuilder::pool_payout)
    /// - [`revshare`](AccountFeesBuilder::revshare)
    /// - [`tax_calculation`](AccountFeesBuilder::tax_calculation)
    /// - [`tax_service`](AccountFeesBuilder::tax_service)
    /// - [`three_ds`](AccountFeesBuilder::three_ds)
    /// - [`transfers`](AccountFeesBuilder::transfers)
    pub fn build(self) -> Result<AccountFees, BuildError> {
        Ok(AccountFees {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            bank_deposit: self
                .bank_deposit
                .ok_or_else(|| BuildError::missing_field("bank_deposit"))?,
            billing: self
                .billing
                .ok_or_else(|| BuildError::missing_field("billing"))?,
            buyer: self
                .buyer
                .ok_or_else(|| BuildError::missing_field("buyer"))?,
            card_processing: self
                .card_processing
                .ok_or_else(|| BuildError::missing_field("card_processing"))?,
            child_markups: self.child_markups,
            cross_border: self
                .cross_border
                .ok_or_else(|| BuildError::missing_field("cross_border"))?,
            dispute: self
                .dispute
                .ok_or_else(|| BuildError::missing_field("dispute"))?,
            dispute_alert: self
                .dispute_alert
                .ok_or_else(|| BuildError::missing_field("dispute_alert"))?,
            dispute_alert_cdrn: self
                .dispute_alert_cdrn
                .ok_or_else(|| BuildError::missing_field("dispute_alert_cdrn"))?,
            dispute_alert_ethoca: self
                .dispute_alert_ethoca
                .ok_or_else(|| BuildError::missing_field("dispute_alert_ethoca"))?,
            dispute_alert_rdr: self
                .dispute_alert_rdr
                .ok_or_else(|| BuildError::missing_field("dispute_alert_rdr"))?,
            dispute_representment: self
                .dispute_representment
                .ok_or_else(|| BuildError::missing_field("dispute_representment"))?,
            foreign_exchange: self
                .foreign_exchange
                .ok_or_else(|| BuildError::missing_field("foreign_exchange"))?,
            fraud_screening: self
                .fraud_screening
                .ok_or_else(|| BuildError::missing_field("fraud_screening"))?,
            high_risk: self
                .high_risk
                .ok_or_else(|| BuildError::missing_field("high_risk"))?,
            marketplace: self
                .marketplace
                .ok_or_else(|| BuildError::missing_field("marketplace"))?,
            markups: self.markups,
            orchestration: self
                .orchestration
                .ok_or_else(|| BuildError::missing_field("orchestration"))?,
            parent_account_id: self.parent_account_id,
            payment_methods: self
                .payment_methods
                .ok_or_else(|| BuildError::missing_field("payment_methods"))?,
            payouts: self
                .payouts
                .ok_or_else(|| BuildError::missing_field("payouts"))?,
            pending_auto_topup: self
                .pending_auto_topup
                .ok_or_else(|| BuildError::missing_field("pending_auto_topup"))?,
            platform_processing: self
                .platform_processing
                .ok_or_else(|| BuildError::missing_field("platform_processing"))?,
            pool_payout: self
                .pool_payout
                .ok_or_else(|| BuildError::missing_field("pool_payout"))?,
            revshare: self
                .revshare
                .ok_or_else(|| BuildError::missing_field("revshare"))?,
            tax_calculation: self
                .tax_calculation
                .ok_or_else(|| BuildError::missing_field("tax_calculation"))?,
            tax_service: self
                .tax_service
                .ok_or_else(|| BuildError::missing_field("tax_service"))?,
            three_ds: self
                .three_ds
                .ok_or_else(|| BuildError::missing_field("three_ds"))?,
            transfers: self
                .transfers
                .ok_or_else(|| BuildError::missing_field("transfers"))?,
        })
    }
}
