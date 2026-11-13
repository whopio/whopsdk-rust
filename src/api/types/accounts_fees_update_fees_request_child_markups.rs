pub use crate::prelude::*;

/// Default markups for subaccounts. Available on accounts without a parent, even before any accounts connect.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestChildMarkups {
    /// The markup on card purchases settled by the subaccount. `null` clears the custom markup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_spend: Option<UpdateFeesRequestChildMarkupsCardSpend>,
    /// The new markup. Fields left out keep their current value. `null` clears the row so the markup returns to its default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_swaps: Option<UpdateFeesRequestChildMarkupsCryptoSwaps>,
    /// Markups on deposits, keyed by rail: `bank` or `crypto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposits: Option<HashMap<String, Option<UpdateFeesRequestChildMarkupsDepositsValue>>>,
    /// Markups on payments, keyed by payment method: `card`, `ach`, or `wire`. `null` clears a method's markup so it uses `payments`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods:
        Option<HashMap<String, Option<UpdateFeesRequestChildMarkupsPaymentMethodsValue>>>,
    /// The new markup. Fields left out keep their current value. `null` clears the row so the markup returns to its default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<UpdateFeesRequestChildMarkupsPayments>,
    /// Markups on withdrawals, keyed by payout method. Percentage and fixed components may be negative, down to negative 100, to subsidize Whop's fee from the platform's balance in the withdrawal currency. The creator's total fee cannot go below zero. An unfunded subsidy keeps the payout pending until funded or failed by the creation deadline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<HashMap<String, Option<UpdateFeesRequestChildMarkupsPayoutsValue>>>,
    /// The new markup. Fields left out keep their current value. `null` clears the row so the markup returns to its default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<UpdateFeesRequestChildMarkupsTransfers>,
}

impl UpdateFeesRequestChildMarkups {
    pub fn builder() -> UpdateFeesRequestChildMarkupsBuilder {
        <UpdateFeesRequestChildMarkupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestChildMarkupsBuilder {
    card_spend: Option<UpdateFeesRequestChildMarkupsCardSpend>,
    crypto_swaps: Option<UpdateFeesRequestChildMarkupsCryptoSwaps>,
    deposits: Option<HashMap<String, Option<UpdateFeesRequestChildMarkupsDepositsValue>>>,
    payment_methods:
        Option<HashMap<String, Option<UpdateFeesRequestChildMarkupsPaymentMethodsValue>>>,
    payments: Option<UpdateFeesRequestChildMarkupsPayments>,
    payouts: Option<HashMap<String, Option<UpdateFeesRequestChildMarkupsPayoutsValue>>>,
    transfers: Option<UpdateFeesRequestChildMarkupsTransfers>,
}

impl UpdateFeesRequestChildMarkupsBuilder {
    pub fn card_spend(mut self, value: UpdateFeesRequestChildMarkupsCardSpend) -> Self {
        self.card_spend = Some(value);
        self
    }

    pub fn crypto_swaps(mut self, value: UpdateFeesRequestChildMarkupsCryptoSwaps) -> Self {
        self.crypto_swaps = Some(value);
        self
    }

    pub fn deposits(
        mut self,
        value: HashMap<String, Option<UpdateFeesRequestChildMarkupsDepositsValue>>,
    ) -> Self {
        self.deposits = Some(value);
        self
    }

    pub fn payment_methods(
        mut self,
        value: HashMap<String, Option<UpdateFeesRequestChildMarkupsPaymentMethodsValue>>,
    ) -> Self {
        self.payment_methods = Some(value);
        self
    }

    pub fn payments(mut self, value: UpdateFeesRequestChildMarkupsPayments) -> Self {
        self.payments = Some(value);
        self
    }

    pub fn payouts(
        mut self,
        value: HashMap<String, Option<UpdateFeesRequestChildMarkupsPayoutsValue>>,
    ) -> Self {
        self.payouts = Some(value);
        self
    }

    pub fn transfers(mut self, value: UpdateFeesRequestChildMarkupsTransfers) -> Self {
        self.transfers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestChildMarkups`].
    pub fn build(self) -> Result<UpdateFeesRequestChildMarkups, BuildError> {
        Ok(UpdateFeesRequestChildMarkups {
            card_spend: self.card_spend,
            crypto_swaps: self.crypto_swaps,
            deposits: self.deposits,
            payment_methods: self.payment_methods,
            payments: self.payments,
            payouts: self.payouts,
            transfers: self.transfers,
        })
    }
}
