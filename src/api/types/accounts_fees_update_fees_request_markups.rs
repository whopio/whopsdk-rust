pub use crate::prelude::*;

/// Markups on this connected account, set by the platform it is connected to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestMarkups {
    /// The new markup. Fields left out keep their current value; `null` clears the row so the markup returns to its default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_swaps: Option<UpdateFeesRequestMarkupsCryptoSwaps>,
    /// Markups on deposits, keyed by rail: `bank` or `crypto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposits: Option<HashMap<String, Option<UpdateFeesRequestMarkupsDepositsValue>>>,
    /// The new markup. Fields left out keep their current value; `null` clears the row so the markup returns to its default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<UpdateFeesRequestMarkupsPayments>,
    /// Markups on withdrawals, keyed by payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<HashMap<String, Option<UpdateFeesRequestMarkupsPayoutsValue>>>,
    /// The new markup. Fields left out keep their current value; `null` clears the row so the markup returns to its default.
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
    crypto_swaps: Option<UpdateFeesRequestMarkupsCryptoSwaps>,
    deposits: Option<HashMap<String, Option<UpdateFeesRequestMarkupsDepositsValue>>>,
    payments: Option<UpdateFeesRequestMarkupsPayments>,
    payouts: Option<HashMap<String, Option<UpdateFeesRequestMarkupsPayoutsValue>>>,
    transfers: Option<UpdateFeesRequestMarkupsTransfers>,
}

impl UpdateFeesRequestMarkupsBuilder {
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
            crypto_swaps: self.crypto_swaps,
            deposits: self.deposits,
            payments: self.payments,
            payouts: self.payouts,
            transfers: self.transfers,
        })
    }
}
