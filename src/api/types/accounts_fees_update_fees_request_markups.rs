pub use crate::prelude::*;

/// Markups on this subaccount, set by the platform it is connected to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestMarkups {
    /// The markup on card purchases settled by the subaccount. `null` clears the custom markup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_spend: Option<UpdateFeesRequestMarkupsCardSpend>,
    /// The new markup. Fields left out keep their current value. `null` clears the row so the markup returns to its default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_swaps: Option<UpdateFeesRequestMarkupsCryptoSwaps>,
    /// Markups on deposits, keyed by rail: `bank` or `crypto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposits: Option<HashMap<String, Option<UpdateFeesRequestMarkupsDepositsValue>>>,
    /// Markups on payments, keyed by payment method: `card`, `ach`, or `wire`. `null` clears a method's markup so it uses `payments`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods:
        Option<HashMap<String, Option<UpdateFeesRequestMarkupsPaymentMethodsValue>>>,
    /// The new markup. Fields left out keep their current value. `null` clears the row so the markup returns to its default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<UpdateFeesRequestMarkupsPayments>,
    /// Markups on withdrawals, keyed by payout method. Percentage and fixed components may be negative, down to negative 100, to subsidize Whop's fee from the platform's balance in the withdrawal currency. The creator's total fee cannot go below zero. An unfunded subsidy keeps the payout pending until funded or failed by the creation deadline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<HashMap<String, Option<UpdateFeesRequestMarkupsPayoutsValue>>>,
    /// The new markup. Fields left out keep their current value. `null` clears the row so the markup returns to its default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<UpdateFeesRequestMarkupsTransfers>,
}

impl UpdateFeesRequestMarkups {
    pub fn builder() -> UpdateFeesRequestMarkupsBuilder {
        <UpdateFeesRequestMarkupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestMarkupsBuilder {
    card_spend: Option<UpdateFeesRequestMarkupsCardSpend>,
    crypto_swaps: Option<UpdateFeesRequestMarkupsCryptoSwaps>,
    deposits: Option<HashMap<String, Option<UpdateFeesRequestMarkupsDepositsValue>>>,
    payment_methods: Option<HashMap<String, Option<UpdateFeesRequestMarkupsPaymentMethodsValue>>>,
    payments: Option<UpdateFeesRequestMarkupsPayments>,
    payouts: Option<HashMap<String, Option<UpdateFeesRequestMarkupsPayoutsValue>>>,
    transfers: Option<UpdateFeesRequestMarkupsTransfers>,
}

impl UpdateFeesRequestMarkupsBuilder {
    pub fn card_spend(mut self, value: UpdateFeesRequestMarkupsCardSpend) -> Self {
        self.card_spend = Some(value);
        self
    }

    pub fn crypto_swaps(mut self, value: UpdateFeesRequestMarkupsCryptoSwaps) -> Self {
        self.crypto_swaps = Some(value);
        self
    }

    pub fn deposits(
        mut self,
        value: HashMap<String, Option<UpdateFeesRequestMarkupsDepositsValue>>,
    ) -> Self {
        self.deposits = Some(value);
        self
    }

    pub fn payment_methods(
        mut self,
        value: HashMap<String, Option<UpdateFeesRequestMarkupsPaymentMethodsValue>>,
    ) -> Self {
        self.payment_methods = Some(value);
        self
    }

    pub fn payments(mut self, value: UpdateFeesRequestMarkupsPayments) -> Self {
        self.payments = Some(value);
        self
    }

    pub fn payouts(
        mut self,
        value: HashMap<String, Option<UpdateFeesRequestMarkupsPayoutsValue>>,
    ) -> Self {
        self.payouts = Some(value);
        self
    }

    pub fn transfers(mut self, value: UpdateFeesRequestMarkupsTransfers) -> Self {
        self.transfers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestMarkups`].
    pub fn build(self) -> Result<UpdateFeesRequestMarkups, BuildError> {
        Ok(UpdateFeesRequestMarkups {
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
